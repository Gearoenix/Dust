pub struct CpuEngine {}

impl CpuEngine {
    pub fn new() -> Self {
        CpuEngine {}
    }

    pub fn render(&self) -> Vec<u8> {
        let mut bitmap = vec![255; 128 * 128 * 4];
        for i in 0..128 {
            for j in 0..128 {
                if i * 128 > j * j {
                    bitmap[(i * 128 + j) * 4] = 100;
                    bitmap[(i * 128 + j) * 4 + 1] = 30;
                    bitmap[(i * 128 + j) * 4 + 2] = 130;
                }
            }
        }
        bitmap
    }
}
