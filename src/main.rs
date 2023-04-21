use std::{thread, time};
use std::io::{stdout, Write, Stdout};
use crossterm::{
    execute, queue,
    style::{self, Stylize}, cursor, terminal, Result
};

struct Board {
    width: u16,
    height: u16,
    top: u16,
    left: u16,
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

enum Direction {
    North(u16),
    South(u16),
    East(u16),
    West(u16),
}

struct Snake {
    body: Vec<(u16, u16)>,
    heading: Direction,
}

impl Snake {
    fn draw(&self, out: &mut Stdout) -> Result<()> {
        for (x, y) in self.body.into_iter() {
            queue!(
                out,
                cursor::MoveTo(x,y),
                style::PrintStyledContent("█".magenta())
                )?;
        }

        Ok(())
    }

    fn advance() {
    }
}

fn game_loop(out: &mut Stdout, board: &Board) -> bool {
    execute!(out, terminal::Clear(terminal::ClearType::All)).unwrap();
    board.draw(out).unwrap();
    out.flush().unwrap();
    thread::sleep(time::Duration::from_millis(16));
    true
}

fn main() -> Result<()> {
    let board = Board {
        top: 0,
        left: 0,
        width: 40,
        height: 20,
    };
    let mut stdout = stdout();
    while game_loop(&mut stdout, &board) {};
    Ok(())
}
