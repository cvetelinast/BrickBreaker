# BrickBreaker
A simple 2D Brick Breaker game

### Used technologies:
* Rust
* ggez

### How to run:
* debug mode:
$DEBUG=1 cargo run

* release mode:
$cargo run

### Input:
* left, right arrow from the keyboard

### Database:
* The level and score information is stored in the file score.txt in the root of the project

### Strengths:
* Running in Debug mode shows the bounding boxes and is not affected by the ball touching the floor.  
* Easily configurable positioning of the bricks for different sizes of screens.
* Three (and easily extendable for more) states of the bricks which allows different behavior when a brick is touched by the ball.
* Easily extendable for adding more screens (pause, user settings like user name, the screen before the start of the game).
