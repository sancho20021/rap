use rap_library::{
    play::{play, Beeper},
    record_beat::record_beat,
};
use rodio::{Decoder, OutputStream, Sink, Source};
use std::{
    fs::File,
    io::{self, BufReader, Cursor},
    time::Duration, thread,
};

// use hyphenation::{Standard, Language, Load, Hyphenator, Iter};

fn readline() -> String {
    let mut buffer = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut buffer).unwrap();
    return buffer.trim().to_owned();
}

fn main() {
    let beat_timestamps = record_beat();
    let start = beat_timestamps[0];
    let mut beat = vec![];
    for x in beat_timestamps {
        beat.push(x - start);
    }

    play(&beat, "metronome.wav");
}
