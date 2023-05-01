# Learn rust by cloning the snake game
objectives:
- understand borrow checker
- vector, loop, thread, timer
- function and constants


## TODO:
- [x] draw a board with corssterm
- [x] snake can move and turn accordingly
- [ ] randomize the fruit and make it appear (not collide with the snake)
- [ ] snake can eat fruit and growth


## Dependencies
- [crossterm](https://docs.rs/crossterm/0.26.1/crossterm): UI backend for the game
- [rand](https://docs.rs/rand/latest/rand): randomizer

## Dev log
#### 02/05/2023
- The snake can move forward and turn with [w, s, a, d] keys
- Game can be exit with Ctrl+C now
    - Bug from last dev log is because all events are pipe into the game it self
    - Therefore we need to handle exit by our own
- BUG: after exit, terminal emulator having weird behaviours

#### 27/04/2023
- The game can listen to keyboard input event now
- BUG: cannot exit game with Ctl+C, maybe because of the thread spawn
