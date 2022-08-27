use std::{
    fs::File,
    io::{BufReader, Cursor},
};

use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream, Sink};

pub fn play_audio(audio: Vec<u8>) -> Result<()> {
    println!("playing audio");

    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let cursor = Cursor::new(audio);
    sink.append(Decoder::new(cursor)?);

    sink.sleep_until_end();
    Ok(())
}
