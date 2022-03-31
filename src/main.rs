use clap::Parser;
use mouse_rs::{
    types::{keys::Keys, Point},
    Mouse,
};
use std::{str::FromStr, thread, time::Duration};

#[derive(Debug)]
enum Pattern {
    BackAndForth,
    Compass,
}

impl FromStr for Pattern {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "back-and-forth" => Ok(Pattern::BackAndForth),
            "compass" => Ok(Pattern::Compass),
            _ => Err(format!("Unknown pattern: {}", s)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Action {
    None,
    Left,
    Right,
    Up,
    Down,
}

impl Pattern {
    fn next_action(&self, prev_action: Action) -> Action {
        match self {
            Pattern::BackAndForth => match prev_action {
                Action::None => Action::Left,
                Action::Left => Action::Right,
                Action::Right => Action::Left,
                _ => Action::None,
            },
            Pattern::Compass => match prev_action {
                Action::None => Action::Left,
                Action::Left => Action::Up,
                Action::Right => Action::Down,
                Action::Up => Action::Right,
                Action::Down => Action::Left,
                _ => Action::None,
            },
        }
    }
}

struct MousePoint {
    x: i32,
    y: i32,
}

impl Into<MousePoint> for Point {
    fn into(self) -> MousePoint {
        MousePoint {
            x: self
                .x
                .try_into()
                .expect(format!("Error converting Point.x to i32: {}", self.x).as_str()),
            y: self
                .y
                .try_into()
                .expect(format!("Error converting Point.y to i32: {}", self.y).as_str()),
        }
    }
}

/// Simple program to automate your mouse
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The number of seconds between each mouse movement
    #[clap(short, long)]
    interval: Option<u64>,
    /// The amount to move the mouse in each direction
    #[clap(short, long)]
    step: Option<i32>,
    /// Whether to click the mouse after moving
    #[clap(short, long)]
    click: Option<bool>,
    /// The movement pattern to use. Valid values are "back-and-forth" and "compass"
    #[clap(short, long)]
    pattern: Option<Pattern>,
}

struct Cleanup;

impl Drop for Cleanup {
    fn drop(&mut self) {
        log::debug!("Workmate clocking out");
    }
}

fn main() {
    let _ = Cleanup;

    log::debug!("Workmate clocking in");

    let args = Args::parse();

    let interval = args.interval.unwrap_or(60);
    let step = args.step.unwrap_or(5);
    let click = args.click.unwrap_or(false);
    let pattern = args.pattern.unwrap_or(Pattern::BackAndForth);

    let mouse = Mouse::new();

    let mut prev_action: Action = Action::None;

    loop {
        let action = pattern.next_action(prev_action);
        let curr_position: MousePoint = mouse.get_position().unwrap_or(Point { x: 0, y: 0 }).into();

        let move_result = match action {
            Action::Left => mouse.move_to(curr_position.x - step, curr_position.y),
            Action::Right => mouse.move_to(curr_position.x + step, curr_position.y),
            Action::Up => mouse.move_to(curr_position.x, curr_position.y - step),
            Action::Down => mouse.move_to(curr_position.x, curr_position.y + step),
            Action::None => {
                log::debug!("No action");
                Ok(())
            }
        };

        if let Err(err) = move_result {
            log::error!("Error moving mouse: {}", err);
        }

        if click {
            // TODO: configurable?
            if let Err(err) = mouse.click(&Keys::LEFT) {
                log::error!("Error clicking mouse: {}", err);
            }
        }

        prev_action = action;

        thread::sleep(Duration::from_secs(interval));
    }
}
