use std::io::{stdout, Write};
use std::time::{Duration, Instant};
use std::thread;
use rand::Rng;
use crossterm::{
    cursor,
    event::{self, KeyCode, KeyEvent},
    execute,
    terminal::{self, ClearType},
};

const SCREEN_WIDTH: u16 = 40;
const SCREEN_HEIGHT: u16 = 20;

fn main() {
    // Initialize the screen
    let mut stdout = stdout();
    execute!(stdout, terminal::EnterAlternateScreen).unwrap();
    execute!(stdout, cursor::Hide).unwrap();
    terminal::enable_raw_mode().unwrap();

    // Initialize the Snake
    let mut snake: Vec<(u16, u16)> = vec![
        (SCREEN_HEIGHT / 2, SCREEN_WIDTH / 4),
        (SCREEN_HEIGHT / 2, SCREEN_WIDTH / 4 - 1),
        (SCREEN_HEIGHT / 2, SCREEN_WIDTH / 4 - 2),
    ];

    // Initialize the food
    let mut rng = rand::thread_rng();
    let mut food = (rng.gen_range(1..SCREEN_HEIGHT - 1), rng.gen_range(1..SCREEN_WIDTH - 1));

    // Initialize the score
    let mut score = 0;

    // Initialize the direction
    let mut direction = KeyCode::Right;

    // Initialize the game loop
    let mut last_update_time = Instant::now();
    let update_interval = Duration::from_millis(100);
    loop {
        // Update the direction based on user input
        if event::poll(Duration::from_millis(0)).unwrap() {
            if let event::Event::Key(key_event) = event::read().unwrap() {
                direction = match key_event.code {
                    KeyCode::Left if direction != KeyCode::Right => KeyCode::Left,
                    KeyCode::Right if direction != KeyCode::Left => KeyCode::Right,
                    KeyCode::Up if direction != KeyCode::Down => KeyCode::Up,
                    KeyCode::Down if direction != KeyCode::Up => KeyCode::Down,
                    _ => direction,
                };
            }
        }

        // Move the Snake
        if last_update_time.elapsed() >= update_interval {
            let head = snake[0];
            let new_head = match direction {
                KeyCode::Right => (head.0, head.1 + 1),
                KeyCode::Left => (head.0, head.1 - 1),
                KeyCode::Up => (head.0 - 1, head.1),
                KeyCode::Down => (head.0 + 1, head.1),
                _ => head,
            };
            snake.insert(0, new_head);

            // Check if the Snake has collided with the wall or itself
            if (
                new_head.0 == 0 || new_head.0 == SCREEN_HEIGHT - 1 ||
                new_head.1 == 0 || new_head.1 == SCREEN_WIDTH - 1 ||
                snake[1..].contains(&new_head)
            ) {
                break;
            }

            // Check if the Snake has eaten the food
            if new_head == food {
                score += 1;
                food = loop {
                    let new_food = (rng.gen_range(1..SCREEN_HEIGHT - 1), rng.gen_range(1..SCREEN_WIDTH - 1));
                    if !snake.contains(&new_food) {
                        break new_food;
                    }
                };
            } else {
                let tail = snake.pop().unwrap();
            }

            last_update_time = Instant::now();
        }

        // Draw the game
        execute!(stdout, cursor::MoveTo(0, 0)).unwrap();
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                if (y == 0 || y == SCREEN_HEIGHT - 1) || (x == 0 || x == SCREEN_WIDTH - 1) {
                    print!("#");
                } else if snake.contains(&(y, x)) {
                    print!("O");
                } else if (y, x) == food {
                    print!("*");
                } else {
                    print!(" ");
                }
            }
            println!("");
        }
        println!("Score: {}", score);

        thread::sleep(Duration::from_millis(10));
    }

    // Clean up the screen
    execute!(stdout, terminal::LeaveAlternateScreen).unwrap();
    execute!(stdout, cursor::Show).unwrap();
}
