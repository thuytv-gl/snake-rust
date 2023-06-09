# Learn rust by cloning the snake game
## Objectives:
- understand borrow checker
- vector, loop, thread, timer
- function and constants

## TODO:
- [x] draw a board with corssterm
- [x] snake can move and turn accordingly
- [x] snake can eat fruit and growth
- [x] randomize the fruit (not collide with the snake)
- [x] game end when: snake collide itself, bump to wall
- [x] show score on screen

## Demo
[![rust-snake](https://asciinema.org/a/Jz1mHsvgeFpy7oepWNRMyu6Xi.png)](https://asciinema.org/a/Jz1mHsvgeFpy7oepWNRMyu6Xi)


## Dependencies
- [crossterm](https://docs.rs/crossterm/0.26.1/crossterm): UI backend for the game
- [rand](https://docs.rs/rand/latest/rand): randomizer

## Dev log
#### 05/05/2023
- Game on
- Fixed all the bugs
- Don't have to care about overflow bc game snake die when it touch the wall anyway
- Score looks nice
- Maybe draw a nice box around the score
- BUG: game render a bit lagging on [Alacritty](https://github.com/alacritty/alacritty)

#### 04/05/2023
- Fruit spawn randomly now
- BUG:
    - overflow when snake go to -x or -y coordinate
    - fruit spawn random so sometime it hard to know when it in the wall
#### 03/05/2023
- I refactor the code, it is easier to read now, and hope that performance be the same
- Fixed weird terminal behavior after exit the game
    - caused by [terminal::enable_raw_mode](//docs.rs/crossterm/latest/crossterm/terminal/fn.enable_raw_mode.html)
- BUG: I noticed there is a bug when the game start, the snake shrink one bit 

#### 02/05/2023
- The snake can move forward and turn with [w, s, a, d] keys
- Game can be exit with Ctrl+C now
    - Bug from last dev log is because all events are pipe into the game it self
    - Therefore we need to handle exit by our own
- BUG: after exit, terminal emulator having weird behaviours

#### 27/04/2023
- The game can listen to keyboard input event now
- BUG: cannot exit game with Ctl+C, maybe because of the thread spawn

