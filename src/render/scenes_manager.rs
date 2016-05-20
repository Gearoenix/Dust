use std::collections::HashMap;
use std::io::{
    Seek,
    SeekFrom,
};

use ::io::file::Stream;

pub struct ScenesManager {
    name_index: HashMap<String, u64>,
}

impl ScenesManager {
    pub fn new() -> ScenesManager {
        ScenesManager {
            name_index: HashMap::new(),
        }
    }

    pub fn read_table(&mut self, s: &mut Stream) {
        let offset = s.read(&0u32) as u64;
        let name = s.read_string();
        self.name_index.insert(name, offset);
    }

    pub fn get_scene(&self, name: &String, s: &mut Stream) {
        s.seek(SeekFrom::start(self.name_index[name]));

    }
}
