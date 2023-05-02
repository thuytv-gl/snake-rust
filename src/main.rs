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

#[derive(Clone, PartialEq)]
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

    fn advance(&mut self, fruit: &Point) {
        let mut new_head = self.body[0].clone();
        match &self.direction {
            Direction::North => new_head.1 -= 1,
            Direction::South => new_head.1 += 1,
            Direction::East => new_head.0 += 1,
            Direction::West => new_head.0 -= 1,
        }

        if new_head != *fruit {
            self.body.pop();
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
    fruit: Point,
    stdout: Stdout,
}

impl Game {
    fn new() -> Self {
        Game {
            board: Board {
                top: 0,
                left: 0,
                width: 40,
                height: 40,
            },
            snake: Snake {
                body: vec![Point(5, 5), Point(6, 5), Point(7, 5), Point(8, 5)],
                direction: Direction::East,
            },
            fruit: Point(7, 7),
            stdout: stdout(),
        }
    }

    fn spawn_fruit(&mut self) {
        queue!(
            self.stdout,
            cursor::MoveTo(7, 7),
            style::PrintStyledContent("█".magenta())
        ).unwrap();
    }

    fn handle_modin(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('c')
                |KeyCode::Char('q') => self.quit(),
            _ => {},
        }
    }

    fn handle_keyin(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('w') => self.snake.turn(Direction::North),
            KeyCode::Char('s') => self.snake.turn(Direction::South),
            KeyCode::Char('a') => self.snake.turn(Direction::West),
            KeyCode::Char('d') => self.snake.turn(Direction::East),
            _ => {},
        }
    }

    fn setup(&self) {
        terminal::enable_raw_mode().unwrap();
    }

    fn cleanup(&mut self) {
        execute!(self.stdout, terminal::Clear(terminal::ClearType::All), cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn quit(&mut self) {
        self.cleanup();
        std::process::exit(0);
    }

    fn play(&mut self) {
        self.setup();
        self.spawn_fruit();
        let (tx, rx) = channel::<Result<KeyEvent>>();

        // waiting for keyboard events
        thread::spawn(move || {
            listent_keyboard_event(&tx);
        });

        loop {
            execute!(self.stdout, terminal::Clear(terminal::ClearType::All), cursor::Hide).unwrap();
            if let Ok(Ok(evt)) = rx.try_recv() {
                match evt {
                    KeyEvent {
                        modifiers: KeyModifiers::CONTROL,
                        code: c,
                        ..
                    } => self.handle_modin(c),
                    KeyEvent {
                        code: c,
                        ..
                    } => self.handle_keyin(c),
                };
            }


            queue!(
                self.stdout,
                cursor::MoveTo(self.fruit.0, self.fruit.1),
                style::PrintStyledContent("█".magenta())
            ).unwrap();
            self.board.draw(&mut self.stdout).unwrap();
            self.snake.draw(&mut self.stdout).unwrap();
            self.stdout.flush().unwrap();
            self.snake.advance(&self.fruit);
            thread::sleep(Duration::from_millis(150));
        }
    }
}

fn main() -> Result<()> {
    let mut game = Game::new();
    game.play();
    Ok(())
}

