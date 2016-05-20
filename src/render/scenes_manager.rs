use std::collections::HashMap;
use std::io::{
    Seek,
    SeekFrom,
};

use ::io::file::Stream;

use ::render::scene::Scene;

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
        let scene_count = s.read(&0u32) as usize;
        println!("Scene count is: {:?}", scene_count);
        for _ in 0..scene_count {
            let offset = s.read(&0u32) as u64;
            let name = s.read_string();
            println!("Scene name is: {:?}", name);
            self.name_index.insert(name, offset);
        }
    }

    pub fn get_scene(&self, name: &String, s: &mut Stream) -> Scene {
        s.reader.seek(SeekFrom::Start(self.name_index[name]));
        let mut scene = Scene::new();
        scene.read(s);
        scene
    }
}
