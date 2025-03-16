use std::{collections::HashSet, path::Path};

use sdl2::{
    event::Event,
    image::{InitFlag, LoadTexture},
    keyboard::Keycode,
    pixels::Color,
    rect::Rect,
    render::WindowCanvas,
    video::Window,
    Sdl,
};

use crate::{
    ctx::{Ctx, Key},
    sprite::Sprite,
};

pub struct SdlCtx {
    sdl: Sdl,
    canvas: WindowCanvas,
    keys_down: HashSet<Key>,
}

impl SdlCtx {
    pub fn new() -> Result<Self, String> {
        let sdl = sdl2::init()?;
        let window = Self::prepare_window(&sdl)?;
        let canvas = Self::prepare_canvas(window)?;

        Ok(Self {
            sdl,
            canvas,
            keys_down: HashSet::new(),
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
}

impl Ctx for SdlCtx {
    type Error = String;

    fn fill_background(&mut self, color: crate::ctx::Rgb) -> Result<(), Self::Error> {
        self.canvas.set_draw_color(Color::RGB(50, 50, 50));
        self.canvas.clear();
        let pos = self.to_world_position((0.0, 0.0));
        let size = self.to_world_scale((10.0, 10.0));
        self.canvas
            .set_draw_color(Color::RGB(color.0, color.1, color.2));
        self.canvas.fill_rect(Rect::new(
            pos.0.floor() as i32,
            pos.1.floor() as i32,
            size.0.floor() as u32,
            size.1.floor() as u32,
        ))?;
        Ok(())
    }

    fn draw_sprite(
        &mut self,
        position: (f64, f64),
        size: (f64, f64),
        sprite: impl Sprite,
    ) -> Result<(), Self::Error> {
        let src_position = sprite.offset();
        let src_size = sprite.size();

        let texture_creator = self.canvas.texture_creator();
        let texture = texture_creator.load_texture(Path::new(sprite.path()))?;

        let dest_position = self.to_world_position(position);
        let dest_size = self.to_world_scale(size);

        self.canvas.copy(
            &texture,
            Rect::new(
                src_position.0 as i32,
                src_position.1 as i32,
                src_size.0,
                src_size.1,
            ),
            Rect::new(
                dest_position.0.floor() as i32,
                dest_position.1.floor() as i32,
                dest_size.0.floor() as u32,
                dest_size.1.floor() as u32,
            ),
        )
    }

    fn key_down(&self, key: crate::ctx::Key) -> bool {
        self.keys_down.contains(&key)
    }

    fn pre_step(&mut self) -> Result<(), String> {
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
        self.canvas.present();
        Ok(())
    }

    fn window_size(&self) -> (f64, f64) {
        let (x, y) = self.canvas.window().size();
        (x as f64, y as f64)
    }
}
