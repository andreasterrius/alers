use rodio::{self, Source, Sink, Decoder};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::io::prelude::*;
use std::collections::HashMap;
use rodio::source::Buffered;

pub struct AudioManager {
    audios: HashMap<String, Vec<u8>>,
}

impl AudioManager {
    pub fn new() -> AudioManager {
        AudioManager { audios: HashMap::new() }
    }

    pub fn register_audio(&mut self, key: &str, audio_byte: Vec<u8>) {
        self.audios.insert(String::from(key), audio_byte);

    }

    //TODO: Make this more efficient since currently this thing clones each time it wants to play a sound
    //TODO: It lags when a lot of sound is played at the same time in win
    pub fn play_audio_simple(&self, key: &str, should_loop: bool) {
        let audio = self.audios.get(key).unwrap().clone();
        //let slice = audio.as_slice();
        let endpoint = rodio::get_default_endpoint().unwrap();
        let sink = rodio::play_once(&endpoint, Cursor::new(audio)).unwrap();
        sink.detach();
    }
}
