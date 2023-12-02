use std::fmt::{self, Display};

#[derive(Debug)]
pub struct Game {
    pub id: i32,
    pub cubes: Vec<SetCubes>,
}

#[derive(Debug)]
pub struct SetCubes {
    pub red: i32,
    pub blue: i32,
    pub green: i32,
}

impl Game {
    pub fn parse(line: &str) -> Option<Self> {
        //Game 100: n blue, n green; 5 blue, 12 green; 16 green, 1 red, 1 blue; 2 blue, 1 green; 1 red, 3 blue, 18 green; 3 green, 1 red, 3 blue
        // println!("{}", line);

        let mut line = line.strip_prefix("Game ")?;
        let mut game = Game {
            id: 0,
            cubes: Vec::new(),
        };

        let index = line.find(':')?;
        let id_str = line.get(0..index)?;
        match i32::from_str_radix(id_str, 10) {
            Ok(v) => game.id = v,
            _ => return None,
        }

        line = line.get((index + 1)..)?;

        for part in line.split(';') {
            let p = SetCubes::parse(part);
            game.cubes.push(p);
        }

        // println!("Game {}{}", game.id, line);
        // println!("{}", game);
        // println!("---");

        Some(game)
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let c: Vec<String> = self.cubes.iter().map(|c| c.to_string()).collect();

        write!(f, "Game {}: {}", self.id, c.join("; "))
    }
}

impl SetCubes {
    pub fn new() -> Self {
        SetCubes {
            red: 0,
            blue: 0,
            green: 0,
        }
    }

    pub fn parse(data: &str) -> Self {
        let mut cube = SetCubes::new();

        for p in data.split(',') {
            if p.ends_with("blue") {
                cube.blue += parse_i32(p.strip_suffix("blue"));
            } else if p.ends_with("green") {
                cube.green += parse_i32(p.strip_suffix("green"));
            } else if p.ends_with("red") {
                cube.red = parse_i32(p.strip_suffix("red"));
            }
        }

        cube
    }
}

impl Display for SetCubes {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} red, {} blue, {} green",
            self.red, self.blue, self.green
        )
    }
}

#[inline]
fn parse_i32(data: Option<&str>) -> i32 {
    return match data {
        None => 0,
        Some(v) => {
            let v = v.trim();

            return match i32::from_str_radix(v, 10) {
                Err(_) => 0,
                Ok(v32) => {
                    assert!(v == v32.to_string());
                    v32
                }
            };
        }
    };
}

impl PartialEq for SetCubes {
    fn eq(&self, other: &Self) -> bool {
        self.red == other.red && self.blue == other.blue && self.green == other.green
    }
}
