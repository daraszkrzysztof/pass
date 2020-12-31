use std::collections::HashMap;
use std::error::Error;
use serde::Serialize;
use serde::Deserialize;
use std::fs::File;

/// Maps username to passwords
#[derive(Clone, Default, Serialize, Deserialize)]
pub struct Database {
    pub records: HashMap<String, Vec<u8>>,
}

impl Database {

    const PATH: &'static str = "users.db";

    pub fn load_or_create() -> Result<Self, Box<dyn Error>> {
        Ok(match File::open(Self::PATH) {
            //Ok(f) => serde_json::from_reader(f)?,
            Ok(f) => bincode::deserialize_from(snap::read::FrameDecoder::new(f))?,
            _ => Default::default(),
        })
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let f = File::create(Self::PATH)?;
        //Ok(serde_json::to_writer_pretty(f, self)?)
        Ok(bincode::serialize_into(snap::write::FrameEncoder::new(f), self)?)
    }

    pub fn with<F, T>(f: F) -> Result<T, Box<dyn Error>>
        where F: FnOnce(&mut Self) -> Result<T, Box<dyn Error>> {
        let mut db = Self::load_or_create()?;
        let res = f(&mut db);
        db.save()?;
        res
    }
}
