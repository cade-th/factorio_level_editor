use std::fs::File;
use std::io::{self, Write};

const WORLD_SIZE: usize = 32;
#[derive(Copy, Clone, Debug)]
pub enum Blocks {
    GRASS = 0,
    STONE = 1,
    PLAYER = 2,
}

impl Blocks {
    fn to_u8(self) -> u8 {
        self as u8
    }
}

pub struct World {
    pub data: [[Blocks; WORLD_SIZE]; WORLD_SIZE],
}

impl World {
    pub fn new() -> Self {
        World {
            data: [[Blocks::STONE; WORLD_SIZE]; WORLD_SIZE],
        }
    }

    pub fn data_to_file(&self, file_name: &str) -> io::Result<()> {
        let mut file = File::create(file_name)?;

        for row in &self.data {
            for &block in row {
                file.write_all(&[block.to_u8()])?;
            }
        }

        println!("world data saved to {}", file_name);
        Ok(())
    }
}
