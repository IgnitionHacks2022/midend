use std::{
    fs::File,
    io::{BufReader, Cursor},
};

use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream, Sink};

/// Play audio from buffer
pub fn play_audio(audio: Vec<u8>) -> Result<()> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let cursor = Cursor::new(audio);
    sink.append(Decoder::new(cursor)?);

    sink.sleep_until_end();
    Ok(())
}

/// Play audio file from path
pub fn play_audio_file(filepath: &str) -> Result<()> {
    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let file = BufReader::new(File::open(filepath)?);
    sink.append(Decoder::new(file)?);

    sink.sleep_until_end();
    Ok(())
}
