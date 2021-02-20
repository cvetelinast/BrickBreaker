# BrickBreaker
A simple 2D Brick Breaker game

### Used technologies and best practices:
* Rust
* ggez
* REDUX design pattern

### How to run:
* debug mode:
$DEBUG=1 cargo run

* release mode:
$cargo run

* tests:
$cargo test

* tests with logs:
$cargo test -- --nocapture

### Input:
* left, right arrow from the keyboard - move the skateboard
* space - switch from "Next level" screen to "Game screen"; switch from "Game over" screen to "Next level" screen

### Database:
* The level and score information is stored in the file score.txt in the root of the project

### Strengths:
* The transition between the different screens is handled with the REDUX design pattern.
* Running in Debug mode shows the bounding boxes and is not affected by the ball touching the floor. 
* Easily configurable positioning of the bricks for different sizes of screens.
* Three (and easily extendable for more) states of the bricks which allows different behavior when a brick is touched by the ball.
* Read from a local file which stores the max score and the last reached level. Updates the result on game over and on winning of the level.
* Easily extendable for adding more screens (pause, user settings like user name).
* The sizes of the assets are relative - if some asset is changed with other size asset runtime, the collisions will work correctly. 