use std::thread::spawn;
use std::sync::{Arc, RwLock};
use std::sync::mpsc::{channel, Receiver, Sender};
use super::engine::Data;
use num_cpus;

pub struct Kernel {
    run_signal: Sender<bool>,
    result_receiver: Receiver<Vec<u8>>,
}

impl Kernel {
    pub fn new(data: &Arc<RwLock<Data>>, index: u32) -> Self {
        let data = data.clone();
        let (run_signal, run_signal_receiver) = channel();
        let (result_signal, result_receiver) = channel();
        let threads_count = num_cpus::get() as u32;
        let _ = spawn(move || {
            Kernel::run(
                data,
                index,
                threads_count,
                run_signal_receiver,
                result_signal,
            );
        });
        Kernel {
            run_signal: run_signal,
            result_receiver: result_receiver,
        }
    }

    pub fn render(&self) {
        self.run_signal.send(true).unwrap();
    }

    pub fn receive(&self) -> Vec<u8> {
        self.result_receiver.recv().unwrap()
    }

    fn run(
        data: Arc<RwLock<Data>>,
        index: u32,
        threads_count: u32,
        run_signal_receiver: Receiver<bool>,
        result_signal: Sender<Vec<u8>>,
    ) {
        while run_signal_receiver.recv().unwrap() {
            let data = data.read().unwrap();
            let starting_row = (data.view_port_dimension.1 * index) / threads_count;
            let ending_row = (data.view_port_dimension.1 * (index + 1)) / threads_count;
            let mut bitmap = vec![
                255u8;
                ((ending_row - starting_row) * data.view_port_dimension.0 * 4)
                    as usize
            ];
            let mut bitmap_index = 0;
            for i in starting_row..ending_row {
                for j in 0..data.view_port_dimension.0 {
                    let x = (j as f64 / data.view_port_dimension.0 as f64 - 0.5) * 5.0;
                    let y = (i as f64 / data.view_port_dimension.1 as f64 - 0.5) * 5.0;
                    let ray = data.cameras[0].get_ray(x, y);
                    let r = data.triangles[0].intersect(&ray, 90000.0, &data.vertices);
                    if let Some(_) = r {
                        bitmap[bitmap_index] = 100;
                        bitmap[bitmap_index + 1] = 30;
                        bitmap[bitmap_index + 2] = 130;
                    }
                    bitmap_index += 4;
                }
            }
            result_signal.send(bitmap).unwrap();
        }
        result_signal.send(Vec::new()).unwrap();
    }
}

impl Drop for Kernel {
    fn drop(&mut self) {
        self.run_signal.send(false).unwrap();
        let _ = self.result_receiver.recv().unwrap();
    }
}
