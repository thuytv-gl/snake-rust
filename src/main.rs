use std::sync::mpsc::{channel, Sender};
use std::thread;
use std::time::Duration;
use std::io::{stdout, Write, Stdout};
use crossterm::{
    execute, queue,
    style::{self, Stylize},
    cursor,
    terminal,
    Result,
    event:: {self, Event, KeyEvent, KeyCode, KeyModifiers},
};

struct Board {
    width: u16,
    height: u16,
    top: u16,
    left: u16,
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

#[derive(Clone)]
struct Point(u16, u16);

struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

impl Board {
    fn right(&self) -> u16 {
        self.width - 1
    }
    fn bottom(&self) -> u16 {
        self.height - 1
    }


    fn is_wall(&self, x: u16, y: u16) -> bool {
        (x == self.left || x == self.right())
            || (y == self.top || y == self.bottom())
    }

    fn draw(&self, out: &mut Stdout) -> Result<()> {
        for y in 0..self.height {
            for x in 0..self.width {
                if self.is_wall(x, y) {
                    queue!(
                        out,
                        cursor::MoveTo(x,y),
                        style::PrintStyledContent("█".magenta())
                        )?;
                }
            }
        }
        Ok(())
    }
}

impl Snake {
    fn draw(&self, out: &mut Stdout) -> Result<()> {
        for Point(x, y) in &self.body {
            queue!(
                out,
                cursor::MoveTo(*x,*y),
                style::PrintStyledContent("█".magenta())
                )?;
        }

        Ok(())
    }

    fn advance(&mut self) {
        self.body.pop();
        let mut new_head = self.body[0].clone();
        match &self.direction {
            Direction::North => new_head.1 -= 1,
            Direction::South => new_head.1 += 1,
            Direction::East => new_head.0 += 1,
            Direction::West => new_head.0 -= 1,
        }

        self.body.insert(0, new_head);
    }

    fn turn(&mut self, dir: Direction) {
        match (&self.direction, &dir) {
            // prevent snake go backward
            (Direction::North, Direction::South)
                | (Direction::South, Direction::North)
                | (Direction::West, Direction::East)
                | (Direction::East, Direction::West) => {},
            _ => self.direction = dir,
        }
    }
}

fn listent_keyboard_event(tx: &Sender<Result<KeyEvent>>) {
    loop {
        if event::poll(Duration::from_millis(1000)).unwrap() {
            if let Ok(Event::Key(e)) = event::read() {
                tx.send(Ok(e)).unwrap();
            }
        }
    }
}

struct Game {
    board: Board,
    snake: Snake,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board {
                top: 0,
                left: 0,
                width: 40,
                height: 20,
            },
            snake: Snake {
                body: vec![Point(5, 5), Point(6, 5), Point(7, 5), Point(8, 5)],
                direction: Direction::East,
            },
        }
    }

    fn assert_keyin(&mut self, key: Result<KeyEvent>) {
        if let Ok(evt) = key {
            match evt {
                KeyEvent {
                    modifiers: KeyModifiers::CONTROL,
                    code: KeyCode::Char('c'),
                    ..
                } => self.quit(),
                KeyEvent {
                    code: KeyCode::Char('w'),
                    ..
                } => self.snake.turn(Direction::North),
                KeyEvent {
                    code: KeyCode::Char('s'),
                    ..
                } => self.snake.turn(Direction::South),
                KeyEvent {
                    code: KeyCode::Char('a'),
                    ..
                } => self.snake.turn(Direction::West),
                KeyEvent {
                    code: KeyCode::Char('d'),
                    ..
                } => self.snake.turn(Direction::East),
                _ => {},
            };
        }
    }

    fn quit(&self) {
        std::process::exit(0);
    }

    fn play(&mut self) {
        terminal::enable_raw_mode().unwrap();
        let mut stdout = stdout();
        let (tx, rx) = channel::<Result<KeyEvent>>();

        // waiting for keyboard events
        thread::spawn(move || {
            listent_keyboard_event(&tx);
        });

        loop {
            execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::Hide).unwrap();
            if let Ok(key) = rx.try_recv() {
                self.assert_keyin(key);
            }

            self.board.draw(&mut stdout).unwrap();
            self.snake.draw(&mut stdout).unwrap();
            stdout.flush().unwrap();
            self.snake.advance();
            thread::sleep(Duration::from_millis(150));
        }
    }
}

fn main() -> Result<()> {
    let mut game = Game::new();
    game.play();
    Ok(())
}

