use std::thread;
use std::time::Duration;

pub struct World {
    x_size: i32,
    y_size: i32,
    random_seed: i32,
    updating_tiles: Vec<Block>,
    all_tiles: Vec<Block>,
}


pub struct Block {
    x: i32,
    y: i32,
    block_type: BlockType,
    previous_type: BlockType,
}

pub enum BlockType {
    SnakeHead,
    SnakeTail,
    SnakeBody,
    Apple,
    Empty,
}


impl Block {


    pub fn print_tile(&self) {
        match self.block_type {
            BlockType::Apple => print!("A"),
            BlockType::Empty => print!("0"),
            _ => print!("X")
        }


    }


    pub fn new(x: i32, y:i32, block_type: BlockType) -> Self {
        Self {
            x,
            y,
            block_type,
            previous_type: BlockType::Empty,
        }
    }


}

impl World {

    pub fn update(&mut self) {
        for _i in 0..self.x_size {
            for _j in  0..self.y_size {
                match self.all_tiles.get(_i * self.y_size + _j) {
                    Some(block) => block.print_tile(),
                    None => println!("bad"),
                }
            }
        }
    }


    pub fn new(x_size: i32, y_size: i32, random_seed: i32) -> Self {
        Self {
            x_size,
            y_size,
            random_seed,
            updating_tiles: Vec::new(),
            all_tiles: Vec::new(),
        }
    }




}


fn main() {
    let x_size = 10;
    let y_size = 10;
    let seed = 4;
    let mut new_world: World = World::new(x_size, y_size, seed);
    for _i in 0..x_size {
        for _j in  0..y_size {
            if _j % 3 == 0 {
                new_world.all_tiles.push(Block::new(_i, _j, BlockType::Apple));
            } else {
                new_world.all_tiles.push(Block::new(_i, _j, BlockType::Empty));
            }
        }
    }

    loop {
        new_world.update();

        thread::sleep(Duration::from_millis(200));
    }
}
