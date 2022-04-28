// Disk I/O Simulator

pub struct FakeDisk {
    pub size: u32,
    pub data: Vec<[u8; 4096]>,
}

impl FakeDisk {
    pub fn new(size: u32) -> FakeDisk {
        let mut data = vec![];
        if size % 128 != 0 {
            panic!()
        }
        for _ in 0..size {
            data.push([0; 4096]);
        }
        FakeDisk {
            size,
            data,
        }
    }

    pub fn fake_disk_read(&self, block_no: u32) -> [[u8; 4096]; 128] {
        let mut data = vec![];
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for index in start_index..end_index {
            data.push(self.data[index as usize]);
        }
        data.try_into().unwrap()
    }
    
    pub fn fake_disk_write(&mut self, address: u32, data: [u8; 4096]) {
        self.data[address as usize] = data;
    }
    
    pub fn fake_disk_erase(&mut self, block_no: u32) {
        let start_index = block_no * 128;
        let end_index = (block_no + 1) * 128;
        for index in start_index..end_index {
            self.data[index as usize] = [0; 4096];
        }
    }
}