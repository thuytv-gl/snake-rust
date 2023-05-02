# Learn rust by cloning the snake game
objectives:
- understand borrow checker
- vector, loop, thread, timer
- function and constants


## TODO:
- [x] draw a board with corssterm
- [x] snake can move and turn accordingly
- [x] snake can eat fruit and growth
- [ ] randomize the fruit (not collide with the snake)
- [ ] game end when: snake collide itself, bump to wall
- [ ] show score on screen


## Dependencies
- [crossterm](https://docs.rs/crossterm/0.26.1/crossterm): UI backend for the game
- [rand](https://docs.rs/rand/latest/rand): randomizer

## Dev log
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
