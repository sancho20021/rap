use std::{fs::File, io::BufReader, thread, time::Duration};

use rodio::{Decoder, OutputStream, OutputStreamHandle, Source, Sink};

pub fn play(beat: &Vec<Duration>, sound_path: &str) {
    let mut pauses = vec![beat[0]];
    for i in 1..beat.len() {
        pauses.push(beat[i] - beat[i - 1]);
    }

    let mut beeper = Beeper::new(sound_path);
    for pause in pauses {
        thread::sleep(pause);
        beeper.beep();
    }
    beeper.sleep_until_end();
}

pub struct Beeper {
    stream: OutputStream,
    stream_handle: OutputStreamHandle,
    sink: Sink,
    sound_path: String,
}

impl Beeper {
    pub fn new(sound_path: &str) -> Self {
        let (stream, stream_handle) = OutputStream::try_default().expect("Output stream failed to open");
        let sink = Sink::try_new(&stream_handle).expect("Sink open failed");
        Self {
            stream,
            stream_handle,
            sink,
            sound_path: sound_path.to_string(),
        }
    }

    pub fn beep(&mut self) {
        let file = BufReader::new(File::open(&self.sound_path).unwrap());
        self.sink = self.stream_handle.play_once(file).expect("Failed to play sound");
    }

    pub fn sleep_until_end(&self) {
        self.sink.sleep_until_end();
    }
}
