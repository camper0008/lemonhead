pub enum AudioConfiguration {
    Play(f32, &'static str),
    Stop,
}

use std::{
    fs::File,
    io::BufReader,
    sync::mpsc::{self, Receiver, Sender},
    thread,
};

use rodio::{Decoder, Sink};

pub fn audio_thread() -> Sender<AudioConfiguration> {
    let (sender, receiver): (Sender<AudioConfiguration>, Receiver<AudioConfiguration>) =
        mpsc::channel();

    thread::spawn(move || {
        let Ok((_stream, stream_handle)) = rodio::OutputStream::try_default() else {
            return Err("unable to open audio channel".to_owned());
        };
        let sink = Sink::try_new(&stream_handle).map_err(|e| e.to_string())?;

        loop {
            let Ok(config) = receiver.recv() else {
                break;
            };
            let (volume, path) = match config {
                AudioConfiguration::Stop => {
                    sink.stop();
                    continue;
                }
                AudioConfiguration::Play(volume, path) => (volume, path),
            };
            sink.set_volume(volume);
            let file = BufReader::new(
                File::open(path).map_err(|_| format!("audio file at {path} not found"))?,
            );
            let source = Decoder::new(file).map_err(|e| e.to_string())?;
            sink.append(source);
        }

        Ok(())
    });

    sender
}
