use serde::Deserialize;
use std::env;
use std::fmt;
use std::fs::File;
use std::io::BufReader;
use std::{thread, time};
use termion::{cursor, clear, color, style};

#[derive(Deserialize, Debug)]
struct Config {
    rows: usize,
    cols: usize,
    steps: usize,
    wait: f64,
    skip: usize,
    start_fire: f64,
    grow_tree: f64,
    new_tree: f64,
}

#[derive(PartialEq, Copy, Clone, Debug)]
enum State {
    Empty,
    Grass,
    Tree,
    Fire,
}

impl State {
    fn as_string(self) -> String {
        match self {
            State::Empty => " ".to_string(),
            State::Grass => ".".to_string(),
            State::Tree => "t".to_string(),
            State::Fire => "F".to_string(),
        }
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

struct World(Vec<Vec<State>>);

impl World {
    fn new(rows: usize, cols: usize) -> World {
        World(vec![vec![State::Empty; cols]; rows])
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn get(&self, r: usize, c: usize) -> State {
        self.0[r][c]
    }

    fn set(&mut self, r: usize, c: usize, v: State) {
        self.0[r][c] = v
    }
}

fn output(w: &World) {
    for r in 0..w.rows() {
        for c in 0..w.cols() {
            let v = w.get(r, c);
            if v == State::Fire {
                print!("{}{}{}", color::Fg(color::Red), v.as_string(), style::Reset);
            } else {
                print!("{}", v);
            }
        }
        println!();
    }
}

fn generate(mut w: World, new_tree: f64) -> World {
    for r in 0..w.rows() {
        for c in 0..w.cols() {
            let p = rand::random::<f64>();
            w.set(
                r,
                c,
                if p <= new_tree {
                    State::Tree
                } else {
                    State::Grass
                },
            )
        }
    }
    w
}

fn neighbors_burning(w: &World, row: usize, col: usize) -> bool {
    let delta = [
        (1, 1),
        (1, 0),
        (1, -1),
        (0, 1),
        (0, -1),
        (-1, 1),
        (-1, 0),
        (-1, -1),
    ];
    for (dr, dc) in delta {
        let r = dr + row as isize;
        let c = dc + col as isize;
        if r >= 0 && c >= 0 && r < w.rows() as isize && c < w.cols() as isize {
            // println!("DBG: {:3},{:3} {:3},{:3} {}", dr, dc, r, c, w.get(r as usize, c as usize));
            if w.get(r as usize, c as usize) == State::Fire {
                // println!("DBG: {:3},{:3} {:3},{:e} {} FIRE", dr, dc, r, c, w.get(r as usize, c as usize));
                return true;
            }
        }
    }
    false
}

fn advance(w: &World, grow_tree: f64, start_fire: f64) -> World {
    let mut a = World::new(w.rows(), w.cols());

    for r in 0..w.rows() {
        for c in 0..w.cols() {
            let p = rand::random::<f64>();
            let n = match w.get(r, c) {
                State::Fire => State::Grass,
                State::Grass => {
                    if p <= grow_tree {
                        State::Tree
                    } else {
                        State::Grass
                    }
                }
                State::Tree => {
                    if neighbors_burning(w, r, c) || p <= start_fire {
                        State::Fire
                    } else {
                        State::Tree
                    }
                }
                State::Empty => State::Empty,
            };
            // println!("DBG: {:03} {:03} {} -> {}", r, c, w.get(r, c), n);
            a.set(r, c, n);
        }
    }

    a
}

fn main() {
    let args: Vec<String> = env::args().skip(1).collect();
    assert_eq!(args.len(), 1);
    let reader = BufReader::new(File::open(&args[0]).unwrap());
    let config: Config = serde_json::from_reader(reader).unwrap();
    let wait = time::Duration::from_secs_f64(config.wait);

    let mut world = generate(World::new(config.rows, config.cols), config.new_tree);
    print!("{}", clear::All);
    for step in 1..config.steps {
        if (config.skip <= 0) || (config.skip > 0 && step % config.skip == 0) {
            print!("{}", cursor::Goto(1, 1));
            println!("* {}", step);
            output(&world);
            thread::sleep(wait);
        }
        world = advance(&world, config.grow_tree, config.start_fire);
    }
}
