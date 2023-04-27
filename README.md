# Learn rust by cloning the snake game
objectives:
- understand borrow checker
- vector, loop, thread, timer
- function and constants


## TODO:
- [ ] draw a board with corssterm
- [ ] snake can move and turn accordingly
- [ ] randomize the fruit and make it appear (not collide with the snake)
- [ ] snake can eat fruit and growth


## Dependencies
- [crossterm](https://docs.rs/crossterm/0.26.1/crossterm): UI backend for the game
- [rand](https://docs.rs/rand/latest/rand): randomizer

## Dev log
#### 27/04/2023
- The game can listen to keyboard input event now
- BUG: cannot exit game with Ctl+C, maybe because of the thread spawn
