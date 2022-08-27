use std::{fs::File, io::BufReader};

use anyhow::Result;
use rodio::{source::Source, Decoder, OutputStream, Sink};

pub fn play_audio() -> Result<()> {
    println!("playing audio");

    let (_stream, handle) = OutputStream::try_default()?;
    let sink = Sink::try_new(&handle)?;

    let file = BufReader::new(File::open("./disco.mp3")?);
    sink.append(Decoder::new_mp3(file)?);

    sink.sleep_until_end();
    Ok(())
}
