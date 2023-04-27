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
    event:: {self, Event, poll, KeyEvent, KeyCode, KeyEventKind},
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
        for mut p in &mut self.body {
            match &self.direction {
                Direction::North => p.1 = 1,
                Direction::South => p.1 -= 1,
                Direction::East => p.0 += 1,
                Direction::West => p.0 -= 1,
            }
        }
    }

    fn turn(&mut self, dir: Direction) {
        match (&self.direction, &dir) {
            (Direction::North, Direction::South) => {},
            (Direction::East, Direction::West) => {},
            _ => self.direction = dir,
        }
    }
}

fn read_char(tx: &Sender<Result<char>>) {
    loop {
        if poll(Duration::from_millis(1000)).unwrap() {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) = event::read()
            {
                tx.send(Ok(c));
            }
        }
    }
}

fn game_loop() {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();
    let board = Board {
        top: 0,
        left: 0,
        width: 40,
        height: 20,
    };
    let mut snake = Snake {
        body: vec![Point(5, 5), Point(6, 5)],
        direction: Direction::East,
    };
    let (tx, rx) = channel::<Result<char>>();

    // waiting for keyboard events
    thread::spawn(move || {
        read_char(&tx);
    });

    loop {
        // execute!(stdout, terminal::Clear(terminal::ClearType::All), cursor::Hide).unwrap();
        let next = rx.try_recv();
        if next.is_ok() {
            println!("{:?}", next);
        }

        board.draw(&mut stdout).unwrap();
        snake.draw(&mut stdout).unwrap();
        stdout.flush().unwrap();
        snake.advance();
        thread::sleep(Duration::from_millis(300));
    }
}

fn main() -> Result<()> {
    game_loop();
    Ok(())
}
