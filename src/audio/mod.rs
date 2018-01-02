use rodio::{self, Source, Sink, Decoder};
use std::fs::File;
use std::io::{BufReader, Cursor};
use std::io::prelude::*;
use std::collections::HashMap;
use rodio::source::Buffered;

pub struct AudioPlayer   {
    audios : HashMap<String, Vec<u8>>
}

impl AudioPlayer {
    pub fn new() -> AudioPlayer {
        AudioPlayer{
            audios : HashMap::new()
        }
    }

    pub fn register_audio(&mut self, key : &str, audio_byte : Vec<u8> ) {
        self.audios.insert(String::from(key), audio_byte);

    }

    pub fn play_audio_simple(&self, key : &str, should_loop : bool){
        let slice = self.audios.get(key).unwrap().as_slice();
        let endpoint = rodio::get_default_endpoint().unwrap();
        let mut sink = rodio::play_once(&endpoint, Cursor::new(slice)).unwrap();
        sink.detach();
    }
}