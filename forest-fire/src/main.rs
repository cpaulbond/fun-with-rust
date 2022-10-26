use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;
use termion::async_stdin;
use termion::event::{Key, Event, MouseEvent};
use termion::input::{TermRead, MouseTerminal};
use termion::raw::IntoRawMode;
use termion::raw::IntoRawMode;
use termion::terminal_size;
use termion::{color, style};

use std::io::{Write, stdout, stdin};

use serde::Deserialize;
// use std::env;
// use std::fmt;
// use std::fs::File;
// use std::io::BufReader;
use std::{thread, time};

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


fn term_default() -> Config {
    let (c, r) = terminal_size().unwrap();

    Config {
        rows: r as usize - 10,
        cols: c as usize - 10,
        steps: 10,
        wait: 0.2,
        skip: 0,
        start_fire: 0.001,
        grow_tree: 0.01,
        new_tree: 0.5
    }
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

fn output<W: Write>(stdout: &mut W, w: &World) {
    write!(stdout, "{}x\n", termion::cursor::Goto(1, 1)).unwrap();
    // for r in 0..w.rows() {
    //     for c in 0..w.cols() {
    //         let v = w.get(r, c);
    //         if v == State::Fire {
    //             write!(stdout, "{}{}{}", color::Fg(color::Red), v.as_string(), style::Reset).unwrap();
    //         } else {
    //             write!(stdout, "{}", v.as_string()).unwrap();
    //         }
    //     }
    //     write!(stdout, "\n").unwrap();
    // }
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
    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let stdin = stdin();
    let stdin = stdin.lock();

    let config: Config = term_default();
    let wait = time::Duration::from_secs_f64(config.wait);

    write!(stdout, "{}", termion::clear::All).unwrap();
    stdout.flush().unwrap();

    // let mut bytes = stdin.bytes();
    // loop {
    //     let mut world = generate(World::new(config.rows, config.cols), config.new_tree);
    //         for step in 1..config.steps {
    //         if (config.skip <= 0) || (config.skip > 0 && step % config.skip == 0) {
    //             write!(stdout, "{}step {}\n", termion::cursor::Goto(1, 1), step).unwrap();
    //             output(&mut stdout, &world);
    //             stdout.flush().unwrap();
    //             thread::sleep(wait);
    //         }
    //         world = advance(&world, config.grow_tree, config.start_fire);
    //     }

    //     let b = bytes.next().unwrap().unwrap();
    //     match b {
    //         // Quit
    //         b'q' => return,
    //         _ => (),
    //     }

    //     stdout.flush().unwrap();
    // }

    let mut world = generate(World::new(config.rows, config.cols), config.new_tree);
    for c in stdin.events() {
        let evt = c.unwrap();
        match evt {
            Event::Key(Key::Char('q')) => break,
            Event::Mouse(me) => {
                match me {
                    MouseEvent::Press(_, x, y) => {
                        write!(stdout, "{}x", termion::cursor::Goto(x, y)).unwrap();
                    }
                    _ => (),
                }
            }
            _ => {}
        }

        output(&mut stdout, &world);
        stdout.flush().unwrap();
        world = advance(&world, config.grow_tree, config.start_fire);
    }

    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
           "{}{}",
           termion::clear::All,
           termion::cursor::Goto(1, 1))
            .unwrap();

    loop {
        write!(stdout, "{}", termion::clear::CurrentLine).unwrap();

        let b = stdin.next();
        write!(stdout, "\r{:?}", b).unwrap();
        if let Some(Ok(b'q')) = b {
            break;
        }

        stdout.flush().unwrap();

        thread::sleep(Duration::from_millis(50));
        stdout.write_all(b"# ").unwrap();
        stdout.flush().unwrap();
        thread::sleep(Duration::from_millis(50));
        stdout.write_all(b"\r #").unwrap();
        write!(stdout, "{}", termion::cursor::Goto(1, 1)).unwrap();
        stdout.flush().unwrap();
    }
}
