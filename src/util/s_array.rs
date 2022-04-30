#[derive(PartialEq)]
pub struct SArray<T> {
    len: u32,
    size: Vec<u32>,
    dimension: u8,
    memory: Vec<Option<T>>,
}

impl<T: Copy> SArray<T> {

    pub fn new(dimension: u8, size: Vec<u32>) -> SArray<T> {
        if dimension as usize != size.len() {
            panic!("SArray: new not matched size");
        }
        for i in size.iter() {
            if *i == 0 {
                panic!("SArray: size cannot be zero");
            }
        }
        let mut len = 1;
        for i in 0..dimension {
            len *= size[i as usize];
        }
        let mut memory = Vec::<Option<T>>::with_capacity(len as usize);
        for _ in 0..len {
            memory.push(None);
        }
        SArray {
            len,
            size,
            dimension,
            memory,
        }
    }

    pub fn get_len(&self) -> u32 {
        self.len
    }

    pub fn get_size(&self) -> Vec<u32> {
        self.size.clone()
    }

    pub fn init_array(&mut self, value: T) {
        for index in 0..self.memory.len() {
            self.memory[index] = Some(value);
        }
    }

    pub fn set(&mut self, pos: Vec<u32>, value: T) {
        if self.dimension as usize != pos.len() {
            panic!("SArray: set not matched pos");
        }
        let off = self.get_off(&pos);
        self.memory[off as usize] = Some(value);
    }

    pub fn get(&self, pos: Vec<u32>) -> T {
        if self.dimension as usize != pos.len() {
            panic!("SArray: get not matched pos");
        }
        let off = self.get_off(&pos);
        if self.memory[off as usize].is_none() {
            panic!("SArray: get pos not exist");
        }
        self.memory[off as usize].unwrap()
    }

    fn get_off(&self, pos: &Vec<u32>) -> u32 {
        self.check_off(pos);
        let len = pos.len();
        let mut off = 0;
        for i in 0..len {
            let mut coefficient = 1;
            for j in i+1..len {
                coefficient *= self.size[j];
            }
            off += coefficient * pos[i];
        }
        off
    }

    fn check_off(&self, pos: &Vec<u32>) {
        for (i, p) in pos.iter().enumerate() {
            if *p > self.size[i] - 1 {
                panic!("SArray: not valid pos");
            }
        }
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn basics() {
        let mut arr = SArray::<u32>::new(3, vec![4, 5, 6]);
        arr.init_array(0);
        arr.set(vec![2, 1, 3], 1);
        assert_eq!(arr.get(vec![2, 1, 3]), 1);
        assert_eq!(arr.get_off(&vec![3, 2, 3]), 105);
        assert_eq!(arr.get(vec![0, 0, 0]), 0);
        assert_eq!(arr.get_off(&vec![0, 0, 0]), 0);
        assert_eq!(arr.get(vec![3, 4, 5]), 0);
        assert_eq!(arr.get_off(&vec![3, 4, 5]), 119);
    }
}