use brick_breaker::assets::Assets;
use brick_breaker::event_handler_wrapper::{EventHandlerWrapper, SCORE_FILE_NAME};
use ggez::conf::{Conf, WindowMode};
use ggez::event::{self};
use ggez::input;
use ggez::timer;
use ggez::{graphics, Context, ContextBuilder, GameError, GameResult};
use std::fs::File;
use std::io::{self, BufRead};

fn main() -> GameResult {
    let conf = Conf::new().window_mode(WindowMode {
        width: 800.0,
        height: 600.0,
        ..Default::default()
    });

    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("brick_breaker", "cveti")
        .default_conf(conf.clone())
        .build()
        .expect("aieee, could not create ggez context!");

    graphics::set_window_title(&ctx, "Brick breaker");

    let main_state = MainState::new(&mut ctx, conf).unwrap();
    event::run(ctx, event_loop, main_state);
}

struct MainState {
    event_handler_wrapper: EventHandlerWrapper,
}

impl MainState {
    pub fn new(_ctx: &mut Context, conf: Conf) -> GameResult<MainState> {
        match read_lines(String::from(SCORE_FILE_NAME)) {
            Ok(lines) => {
                let x = lines.map(|x| x.unwrap()).collect::<Vec<String>>();
                if x.len() != 2 {
                    let message = format!("There was an error with the file {}.", SCORE_FILE_NAME)
                        .to_string();
                    return Err(GameError::ResourceLoadError(message));
                }
                return Self::initialize_main_state(_ctx, conf, x);
            }
            _ => {
                let message = format!("The file {} was not found.", SCORE_FILE_NAME).to_string();
                return Err(GameError::ResourceLoadError(message));
            }
        };
    }

    pub fn initialize_main_state(
        ctx: &mut Context,
        conf: Conf,
        lines: Vec<String>,
    ) -> GameResult<MainState> {
        let assets = Assets::new(ctx)?;
        let level = lines[0].parse::<i32>().unwrap();
        let max_score = lines[1].parse::<usize>().unwrap();

        let e = EventHandlerWrapper::new(conf, assets, level, max_score);

        return Ok(MainState {
            event_handler_wrapper: e,
        });
    }
}

// The output is wrapped in a Result to allow matching on errors
// Returns an Iterator to the Reader of the lines of the file.
pub fn read_lines(filename: String) -> io::Result<io::Lines<io::BufReader<File>>> {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        self.event_handler_wrapper.update(ctx)
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        self.event_handler_wrapper.key_down_event(keycode);
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keycode: event::KeyCode,
        _keymod: input::keyboard::KeyMods,
    ) {
        self.event_handler_wrapper.key_up_event(keycode);
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx, graphics::BLACK);
        match self.event_handler_wrapper.draw(ctx) {
            Ok(_) => {}
            _ => {
                return Err(GameError::EventLoopError(String::from(
                    "An error with drawing occurred.",
                )));
            }
        }
        graphics::present(ctx)?;

        // And yield the timeslice
        // This tells the OS that we're done using the CPU but it should
        // get back to this program as soon as it can.
        // This ideally prevents the game from using 100% CPU all the time
        // even if vsync is off.
        // The actual behavior can be a little platform-specific.
        timer::yield_now();

        Ok(())
    }
}
