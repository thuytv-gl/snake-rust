use rand::Rng;
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

const BOARD_WIDTH: u16 = 40;
const BOARD_HEIGHT: u16 = BOARD_WIDTH;

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

#[derive(Clone, PartialEq, Debug)]
struct Point(u16, u16);

impl Board {
    fn new() -> Self {
        Board {
            top: 0,
            left: 0,
            width: BOARD_WIDTH,
            height: BOARD_HEIGHT,
        }
    }
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

struct Snake {
    body: Vec<Point>,
    direction: Direction,
}

impl Snake {
    fn new() -> Self {
        Snake {
            body: vec![Point(5, 5), Point(6, 5)],
            direction: Direction::East,
        }
    }

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

    fn advance(&mut self, fruit: &Fruit) -> Result<bool> {
        let Point(mut x, mut y) = self.body[0].clone();
        match &self.direction {
            Direction::North => y = y.wrapping_sub(1),
            Direction::South => y = y.wrapping_add(1),
            Direction::East => x = x.wrapping_add(1),
            Direction::West => x = x.wrapping_sub(1),
        };
        self.plug_head(Point(x, y));
        if Point(x, y) != fruit.coord {
            self.body.pop();
            return Ok(false);
        }
        Ok(true)
    }

    fn chop_head(&mut self) -> Point {
        self.body.remove(0)
    }

    fn plug_head(&mut self, new_head: Point) {
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

struct Fruit {
    coord: Point,
}

impl Fruit {
    fn new(board: &Board) -> Self {
        let mut rng = rand::thread_rng();
        let x: u16 =  rng.gen_range(1..board.right());
        let y: u16 =  rng.gen_range(1..board.bottom());
        Fruit {
            coord: Point(x, y)
        }
    }
    
    fn draw(&self, stdout: &mut Stdout) -> Result<()> {
        queue!(
            stdout,
            cursor::MoveTo(self.coord.0, self.coord.1),
            style::PrintStyledContent("█".red())
        )?;
        Ok(())
    }
}

struct Game {
    board: Board,
    snake: Snake,
    fruit: Fruit,
    stdout: Stdout,
}

impl Game {
    fn new() -> Self {
        let board = Board::new();
        Game {
            fruit: Fruit::new(&board),
            board,
            snake: Snake::new(),
            stdout: stdout(),
        }
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
        execute!(self.stdout, cursor::Show).unwrap();
        terminal::disable_raw_mode().unwrap();
    }

    fn quit(&mut self) {
        self.cleanup();
        std::process::exit(0);
    }

    fn assert_keyin(&mut self, key: Result<KeyEvent>) {
        if let Ok(evt) = key {
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
    }

    fn update_score(&mut self) -> Result<()> {
        let head = self.snake.chop_head();
        match head {
            _ if self.snake.body.iter().any(|p| *p == head) => self.quit(),
            _ if head == self.fruit.coord => self.fruit = Fruit::new(&self.board),
            Point(x, _) if x == 0 || x == self.board.right() => self.quit(),
            Point(_, y) if y == 0 || y == self.board.bottom() => self.quit(),
            _ => {},
        };

        self.snake.plug_head(head);

        let score = format!("Score: {}", self.snake.body.len()).magenta();
        queue!(
            self.stdout,
            cursor::MoveTo(self.board.right() + 4, 2),
            style::PrintStyledContent(score)
        )?;
        Ok(())
    }

    fn draw_help(&mut self) -> Result<()>{
        queue!(
            self.stdout,
            cursor::MoveTo(self.board.right() + 4, 4),
            style::PrintStyledContent("move: w,s,a,d".green())
        )?;
        Ok(())
    }

    fn play(&mut self) -> Result<()> {
        self.setup();
        let (tx, rx) = channel::<Result<KeyEvent>>();

        // waiting for keyboard events
        thread::spawn(move || {
            listent_keyboard_event(&tx);
        });

        loop {
            queue!(
                self.stdout,
                terminal::Clear(terminal::ClearType::All),
                cursor::Hide)?;

            if let Ok(key) = rx.try_recv() {
                self.assert_keyin(key);
            }
            self.draw_help()?;
            self.board.draw(&mut self.stdout)?;
            self.fruit.draw(&mut self.stdout)?;
            self.snake.draw(&mut self.stdout)?;
            self.snake.advance(&self.fruit)?;
            self.update_score()?;
            self.stdout.flush()?;
            thread::sleep(Duration::from_millis(100));
        }
    }
}

fn main() -> Result<()> {
    let mut game = Game::new();
    if game.play().is_err() {
        game.quit();
    }
    Ok(())
}

