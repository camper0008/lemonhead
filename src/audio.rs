pub enum Configuration {
    Play(f32, &'static str),
    Repeat(bool),
    Stop,
}

use std::{
    fs::File,
    io::BufReader,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use rodio::{Decoder, OutputStreamHandle, Sink, Source};

fn consume_sink(sink: Sink, stream_handle: &OutputStreamHandle) -> Result<Sink, String> {
    sink.pause();
    let sink = Sink::try_new(stream_handle).map_err(|e| e.to_string())?;

    Ok(sink)
}

pub fn audio_thread() -> Sender<Configuration> {
    let (sender, receiver): (Sender<Configuration>, Receiver<Configuration>) = mpsc::channel();

    thread::spawn(move || {
        let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() else {
            return Err("unable to open audio channel".to_owned());
        };
        let mut sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;
        let mut repeat = false;

        loop {
            let Ok(config) = receiver.recv() else {
                break;
            };
            let (volume, path) = match config {
                Configuration::Stop => {
                    sink = consume_sink(sink, &stream_handle)?;
                    continue;
                }
                Configuration::Play(volume, path) => (volume, path),
                Configuration::Repeat(value) => {
                    repeat = value;
                    continue;
                }
            };

            sink = consume_sink(sink, &stream_handle)?;
            sink.set_volume(volume);
            let file = BufReader::new(
                File::open(path).map_err(|_| format!("audio file at {path} not found"))?,
            );
            let source = Decoder::new(file).map_err(|e| e.to_string())?;
            if repeat {
                sink.append(source.repeat_infinite());
            } else {
                sink.append(source);
            }
        }

        Ok(())
    });

    sender
}
