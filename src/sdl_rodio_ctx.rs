use std::{
    collections::{HashMap, HashSet, VecDeque},
    marker::PhantomData,
    time::Instant,
};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::{Texture, TextureCreator, WindowCanvas},
    video::{Window, WindowContext},
    Sdl,
};

use crate::{
    ctx::{Ctx, Rgb},
    sprite::Sprite,
};
use rodio::{Decoder, OutputStreamHandle, Sink, Source};

enum AudioEvent {
    Info(AudioInfo),
    Stop,
}
struct AudioInfo {
    path: &'static str,
    volume: f32,
    repeat: bool,
}

fn audio_thread() -> std::sync::mpsc::Sender<AudioEvent> {
    fn consume_sink(sink: Sink, stream_handle: &OutputStreamHandle) -> Result<Sink, String> {
        sink.pause();
        let sink = Sink::try_new(stream_handle).map_err(|e| e.to_string())?;

        Ok(sink)
    }

    let (sender, receiver) = std::sync::mpsc::channel::<AudioEvent>();

    std::thread::spawn(move || {
        let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() else {
            return Err("unable to open audio channel".to_owned());
        };
        let mut sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        loop {
            let Ok(event) = receiver.recv() else {
                break;
            };

            let info = match event {
                AudioEvent::Info(info) => info,
                AudioEvent::Stop => {
                    sink = consume_sink(sink, &stream_handle)?;
                    continue;
                }
            };
            sink = consume_sink(sink, &stream_handle)?;
            sink.set_volume(info.volume);
            let file = std::io::BufReader::new(
                std::fs::File::open(info.path)
                    .map_err(|_| format!("audio file at {} not found", info.path))?,
            );
            let source = Decoder::new(file).map_err(|e| e.to_string())?;
            if info.repeat {
                sink.append(source.repeat_infinite());
            } else {
                sink.append(source);
            }
        }

        Ok(())
    });

    sender
}

enum QueueItem {
    Sprite {
        sprite_path: &'static str,
        sprite_offset: (u32, u32),
        sprite_size: (u32, u32),
        position: (f64, f64),
        size: (f64, f64),
    },
    FillBackground(Rgb),
    DrawRect {
        color: Rgb,
        position: (f64, f64),
        size: (f64, f64),
    },
}

pub struct SdlRodioCtx {
    sdl: Sdl,
    canvas: WindowCanvas,
    playing_music: Option<crate::ctx::Music>,
    keys_down: HashSet<crate::ctx::Key>,
    render_queue: VecDeque<QueueItem>,
    music_handle: std::sync::mpsc::Sender<AudioEvent>,
    effect_handle: std::sync::mpsc::Sender<AudioEvent>,
    started: Instant,
}

impl SdlRodioCtx {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let window = Self::prepare_window(&sdl)?;
        let canvas = Self::prepare_canvas(window)?;

        Ok(Self {
            sdl,
            canvas,
            playing_music: None,
            music_handle: audio_thread(),
            effect_handle: audio_thread(),
            keys_down: HashSet::new(),
            render_queue: VecDeque::new(),
            started: Instant::now(),
        })
    }

    fn prepare_window(sdl: &Sdl) -> Result<Window, String> {
        let video_subsystem = sdl.video()?;
        let _image_context = sdl2::image::init(InitFlag::PNG | InitFlag::JPG)?;
        let window = video_subsystem
            .window("the adventures of lemonhead", 720, 720)
            .position_centered()
            .resizable()
            .maximized()
            .build()
            .map_err(|e| e.to_string())?;
        Ok(window)
    }

    fn prepare_canvas(window: Window) -> Result<WindowCanvas, String> {
        window
            .into_canvas()
            .software()
            .build()
            .map_err(|e| e.to_string())
    }

    fn draw_queue(&mut self) -> Result<(), String> {
        let texture_creator = self.canvas.texture_creator();
        let mut texture_cache = HashMap::new();
        loop {
            let Some(item) = self.render_queue.pop_front() else {
                break Ok(());
            };
            match item {
                QueueItem::Sprite {
                    sprite_path,
                    sprite_offset,
                    sprite_size,
                    position,
                    size,
                } => {
                    if !texture_cache.contains_key(&sprite_path) {
                        let texture = texture_creator.load_texture(sprite_path)?;
                        texture_cache.insert(sprite_path, texture);
                    }

                    let texture = texture_cache.get(&sprite_path).expect("we have to insert");

                    self.canvas.copy(
                        texture,
                        Rect::new(
                            sprite_offset.0 as i32,
                            sprite_offset.1 as i32,
                            sprite_size.0,
                            sprite_size.1,
                        ),
                        Rect::new(
                            position.0 as i32,
                            position.1 as i32,
                            size.0 as u32,
                            size.1 as u32,
                        ),
                    )?;
                }
                QueueItem::FillBackground(color) => {
                    self.canvas
                        .set_draw_color(Color::RGB(color.0, color.1, color.2));
                    self.canvas.clear();
                }
                QueueItem::DrawRect {
                    color,
                    position,
                    size,
                } => {
                    self.canvas
                        .set_draw_color(Color::RGB(color.0, color.1, color.2));
                    self.canvas.fill_rect(Rect::new(
                        position.0 as i32,
                        position.1 as i32,
                        size.0 as u32,
                        size.1 as u32,
                    ))?;
                }
            }
        }
    }
}

impl Ctx for SdlRodioCtx {
    type Error = String;

    fn fill_background(&mut self, color: crate::ctx::Rgb) -> Result<(), Self::Error> {
        self.render_queue
            .push_back(QueueItem::FillBackground(color));
        Ok(())
    }

    fn fill_screen_rect(
        &mut self,
        color: crate::ctx::Rgb,
        position: (f64, f64),
        size: (f64, f64),
    ) -> Result<(), Self::Error> {
        self.render_queue.push_back(QueueItem::DrawRect {
            color,
            position,
            size,
        });
        Ok(())
    }

    fn draw_sprite(
        &mut self,
        position: (f64, f64),
        size: (f64, f64),
        sprite: &impl Sprite,
    ) -> Result<(), Self::Error> {
        let position = self.to_screen_position(position);
        let size = self.to_screen_scale(size);
        self.render_queue.push_back(QueueItem::Sprite {
            sprite_path: sprite.path(),
            sprite_offset: sprite.offset(),
            sprite_size: sprite.size(),
            position,
            size,
        });
        Ok(())
    }

    fn key_down(&self, key: crate::ctx::Key) -> bool {
        self.keys_down.contains(&key)
    }

    fn pre_step(&mut self) -> Result<(), String> {
        self.keys_down.remove(&Key::Interact);
        use crate::ctx::Key;
        for event in self.sdl.event_pump()?.poll_iter() {
            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => {
                    self.keys_down.insert(Key::Quit);
                }
                Event::KeyDown {
                    keycode: Some(key), ..
                } => {
                    let key = match key {
                        Keycode::Space => Key::Interact,
                        Keycode::A | Keycode::Left => Key::Left,
                        Keycode::D | Keycode::Right => Key::Right,
                        _ => continue,
                    };
                    self.keys_down.insert(key);
                }
                Event::KeyUp {
                    keycode: Some(key), ..
                } => {
                    let key = match key {
                        Keycode::Space => Key::Interact,
                        Keycode::A | Keycode::Left => Key::Left,
                        Keycode::D | Keycode::Right => Key::Right,
                        _ => continue,
                    };
                    self.keys_down.remove(&key);
                }
                _ => {}
            }
        }
        Ok(())
    }

    fn post_step(&mut self) -> Result<(), Self::Error> {
        self.fill_border()?;
        self.draw_queue()?;
        self.canvas.present();
        std::thread::sleep(std::time::Duration::new(0, 1_000_000_000u32 / 60));
        Ok(())
    }

    fn window_size(&self) -> (f64, f64) {
        let (x, y) = self.canvas.window().size();
        (x as f64, y as f64)
    }

    fn play_effect(&mut self, effect: crate::ctx::Effect) -> Result<(), Self::Error> {
        self.effect_handle
            .send(AudioEvent::Info(AudioInfo {
                repeat: false,
                path: effect.path(),
                volume: effect.volume(),
            }))
            .map_err(|e| e.to_string())
    }

    fn set_music(&mut self, music: crate::ctx::Music) -> Result<(), Self::Error> {
        if self
            .playing_music
            .as_ref()
            .is_some_and(|other| *other == music)
        {
            return Ok(());
        }
        self.music_handle
            .send(AudioEvent::Info(AudioInfo {
                repeat: true,
                path: music.path(),
                volume: music.volume(),
            }))
            .map_err(|e| e.to_string())?;
        self.playing_music = Some(music);
        Ok(())
    }

    fn stop_music(&mut self) -> Result<(), Self::Error> {
        self.music_handle
            .send(AudioEvent::Stop)
            .map_err(|e| e.to_string())?;
        self.playing_music = None;
        Ok(())
    }

    fn seconds_elapsed(&self) -> f64 {
        (Instant::now() - self.started).as_secs_f64()
    }
}
