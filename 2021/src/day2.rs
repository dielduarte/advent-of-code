use std::fs;

#[derive(Default)]
struct Submarine {
  horizontal: i32,
  depth: i32,
  aim: i32
}

impl Submarine {
  fn sum(&self) -> i32 {
    self.depth * self.horizontal  
  }
}

pub fn main() {
  let file = fs::read_to_string("src/input-day2.txt")
      .unwrap();
  
  let position = file.lines().fold(
    Submarine::default(),
    |mut acc, item| {
      let items = item.split(' ').collect::<Vec<&str>>();
      let value = items[1].parse::<i32>().unwrap();

      match items[0] {
        "forward" => {
          acc.horizontal += value;
          acc.depth += acc.aim * value;
        },
        "up" => {
          acc.aim -= value
        },
        "down" => {
          acc.aim += value
        },
        i => panic!("unhandled input {}", i)
      }

      acc
    }
  );

  println!("{}", position.sum())
}
