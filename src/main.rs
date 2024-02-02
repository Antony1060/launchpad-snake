use std::cmp::min;
use std::sync::{Arc, Mutex};
use std::thread::sleep;
use std::time::{Duration, Instant};

use launchy::launchpad_mini_mk3::Button::GridButton;
use launchy::mini_mk3::{Message, RgbColor};
use launchy::{InputDevice, OutputDevice};
use lazy_static::lazy_static;

use crate::models::direction;
use crate::models::direction::Direction;
use crate::models::snake::Snake;
use crate::models::vector2i::Vector2i;
use crate::utils::colors::gen_rainbow_colors_launchpad;
use crate::utils::launchpad::RgbUpdate;

mod models;
mod utils;

const PAD_HEIGHT: u8 = 9;
const PAD_WIDTH: u8 = 9;

const MS_PER_TICK: u128 = 200;

lazy_static! {
    static ref APPLE_COLOR: RgbColor = RgbColor::new(100, 0, 0);
}

fn main() -> anyhow::Result<()> {
    let mut output = launchy::mini_mk3::Output::guess()?;

    let center = Vector2i {
        x: (PAD_WIDTH / 2) - 1,
        y: PAD_HEIGHT / 2,
    };

    let snake = Snake::new_at(center);
    let snake = Arc::new(Mutex::new(snake));
    let snake_input = snake.clone();

    let _input = launchy::mini_mk3::Input::guess(move |msg| {
        if let Message::Press {
            button: GridButton { x, y },
        } = msg
        {
            let pair = (x, y);
            let mut snake = snake_input.lock().expect("mutex should lock properly");

            if pair == direction::RESET_KEY {
                snake.reset();
                drop(snake);
                return;
            }

            let new_direction = match pair {
                direction::UP_KEY => Direction::Up,
                direction::DOWN_KEY => Direction::Down,
                direction::LEFT_KEY => Direction::Left,
                direction::RIGHT_KEY => Direction::Right,
                _ => return,
            };

            snake.change_direction(new_direction);

            drop(snake);
        }
    })?;

    let rainbow = gen_rainbow_colors_launchpad();

    loop {
        let start = Instant::now();

        let mut snake = snake.lock().expect("mutex should lock properly");

        snake.tick();

        output.clear()?;

        let updates = snake
            .clone()
            .into_iter()
            .enumerate()
            .map(|(index, part)| {
                (
                    part,
                    // start somewhere not at beginning, then offset
                    rainbow[((index * 20) + 400) % rainbow.len()],
                )
            })
            .collect::<Vec<_>>();

        let apple = snake.get_apple();

        utils::launchpad::light_rbg(
            &mut output,
            &[&[(apple, *APPLE_COLOR)] as &[RgbUpdate], &updates].concat(),
        )?;

        drop(snake);

        let dur = start.elapsed();
        sleep(Duration::from_millis(
            (MS_PER_TICK - min(dur.as_millis(), MS_PER_TICK)) as u64,
        ))
    }
}
