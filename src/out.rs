use std::{fs::File, io::BufReader, rc::Rc, thread, time::Duration};

use rodio::{Decoder, OutputStream, Source};

use self::nametree::Tree;
use crate::Result;

mod nametree;

pub fn tree<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    let mut nametree = Tree::new();
    for file in files {
        nametree.insert(&file);
    }

    println!("{}", nametree);
}

pub fn list<I>(files: I)
where
    I: IntoIterator<Item = Rc<str>>,
{
    for file in files {
        println!("{}", file);
    }
}

pub fn simple<I>(files: I, repeat: bool) -> Result<()>
where
    I: IntoIterator<Item = Rc<str>>,
{
    for file in files {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let reader = BufReader::new(File::open(&*file)?);
        let source = Decoder::new(reader)?;

        match repeat {
            true => stream_handle.play_raw(source.repeat_infinite().convert_samples())?,
            false => stream_handle.play_raw(source.convert_samples())?,
        }

        println!("now playing '{}'", file);
        thread::sleep(Duration::MAX);
    }

    Ok(())
}
