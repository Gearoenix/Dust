use std::sync::{Arc, RwLock};
use super::kernel::Kernel;
use num_cpus;

pub struct Data {
    pub view_port_dimension: (u32, u32),
}

pub struct CpuEngine {
    pub data: Arc<RwLock<Data>>,
    pub kernels: Vec<Kernel>,
}

impl CpuEngine {
    pub fn new(data: Data) -> Self {
        let data = Arc::new(RwLock::new(data));
        let mut kernels = Vec::new();
        for i in 0..num_cpus::get() {
            kernels.push(Kernel::new(&data, i as u32));
        }
        CpuEngine {
            data: data,
            kernels: kernels,
        }
    }

    pub fn render(&self) -> Vec<u8> {
        for k in &self.kernels {
            k.render();
        }
        let data = self.data.read().unwrap();
        let mut bitmap =
            vec![255u8; (data.view_port_dimension.0 * data.view_port_dimension.1 * 4) as usize];
        let threads_count = num_cpus::get() as u32;
        let mut bitmap_index = 0;
        let mut starting_row = 0;
        for i in 0..self.kernels.len() {
            let ending_row = (data.view_port_dimension.1 * (i as u32 + 1)) / threads_count;
            let pixels = self.kernels[i].receive();
            let mut pixels_index = 0;
            for _ in starting_row..ending_row {
                for _ in 0..data.view_port_dimension.0 {
                    for _ in 0..4 {
                        bitmap[bitmap_index] = pixels[pixels_index];
                        pixels_index += 1;
                        bitmap_index += 1;
                    }
                }
            }
            starting_row = ending_row;
        }
        bitmap
    }
}
