use crossterm::terminal::{enable_raw_mode, disable_raw_mode};
use crossterm::event::{read, Event, KeyCode};

use crossterm::event::poll;
use std::time::Duration;
use std::thread::sleep;
use rand::Rng;
use crossterm::execute;
use crossterm::terminal::Clear;
use crossterm::terminal::ClearType;
use crossterm::cursor::MoveTo;

struct Position {
	x: u8,
	y: u8,
}

#[derive(PartialEq)]
enum Direction {
	Up,
	Down,
	Left,
	Right,
}

struct Snake {
	body: Vec<Position>,
	direction: Direction,
}

const WIDTH: u8 = 20;
const HEIGHT: u8 = 20;

fn spawn_food(snake: &Snake) -> Position {
	let mut rng = rand::rng();
	loop {
		let food = Position {
			x: rng.random_range(0..WIDTH),
			y: rng.random_range(0..HEIGHT),
		};
		if !snake_occupies(&snake, &food) {
			return food
		}
	}
}

fn snake_occupies(snake: &Snake, position: &Position) -> bool {
	return snake.body.iter().any(
		|p| p.x == position.x && p.y == position.y
	);
}

fn move_snake(snake: &mut Snake, food: &mut Position, score: &mut u64) -> bool {
	let mut new_head: Position;

	match snake.direction {
		Direction::Up => {
			new_head = Position {
				x: snake.body[0].x,
				y: if snake.body[0].y == 0 {
					HEIGHT - 1
				} else {
					snake.body[0].y - 1
				}
			};
		}
		Direction:: Down => {
			new_head = Position {
				x: snake.body[0].x,
				y: (snake.body[0].y + 1) % HEIGHT,
			};
		}
		Direction::Left => {
			new_head = Position {
				x: if snake.body[0].x == 0 {
					WIDTH - 1
				} else {
					snake.body[0].x - 1
				},
				y: snake.body[0].y,
			};
		}
		Direction::Right => {
			new_head = Position {
				x: (snake.body[0].x + 1) % WIDTH,
				y: snake.body[0].y
			};
		}
	}

	if snake_occupies(&snake, &new_head) {
		return false;
	};

	if new_head.x != food.x || new_head.y != food.y {
		snake.body.pop();
	} else {
		*food = spawn_food(&snake);
		*score += 1;
	}

	// add the new head to the snake
	snake.body.insert(0, new_head);

	return true;	
}

fn render(snake: &Snake, food: &Position, game_over: &bool, score: &mut u64) {
	let mut grid = [["  "; WIDTH as usize]; HEIGHT as usize];

	grid[food.y as usize][food.x as usize] = "X ";

	for (i, item) in snake.body.iter().enumerate() {
		if i == 0 {
			match snake.direction {
				Direction::Up => grid[item.y as usize][item.x as usize] = "v ",
				Direction::Down => grid[item.y as usize][item.x as usize] = "^ ",
				Direction::Left => grid[item.y as usize][item.x as usize] = "> ",
				Direction::Right => grid[item.y as usize][item.x as usize] = "< ",
			}
		} else if i == snake.body.len() - 1 {
			grid[item.y as usize][item.x as usize] = "* ";
		} else {
			grid[item.y as usize][item.x as usize] = "# ";
		}
	}
	
	execute!(
 		std::io::stdout(),
		Clear(ClearType::All),
		MoveTo(0,0)
	).unwrap();
  
	if *game_over {
		print!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣠⡀⠀\r\n");
		print!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⢀⣤⣤⠀⠀⠀⢀⣴⣿⡶⠀⣾⣿⣿⡿⠟⠛⠁\r\n");
		print!("⠀⠀⠀⠀⠀⠀⣀⣀⣄⣀⠀⠀⠀⠀⣶⣶⣦⠀⠀⠀⠀⣼⣿⣿⡇⠀⣠⣿⣿⣿⠇⣸⣿⣿⣧⣤⠀⠀⠀\r\n");
		print!("⠀⠀⢀⣴⣾⣿⡿⠿⠿⠿⠇⠀⠀⣸⣿⣿⣿⡆⠀⠀⢰⣿⣿⣿⣷⣼⣿⣿⣿⡿⢀⣿⣿⡿⠟⠛⠁⠀⠀\r\n");
		print!("⠀⣴⣿⡿⠋⠁⠀⠀⠀⠀⠀⠀⢠⣿⣿⣹⣿⣿⣿⣿⣿⣿⡏⢻⣿⣿⢿⣿⣿⠃⣼⣿⣯⣤⣴⣶⣿⡤⠀\r\n");
		print!("⣼⣿⠏⠀⣀⣠⣤⣶⣾⣷⠄⣰⣿⣿⡿⠿⠻⣿⣯⣸⣿⡿⠀⠀⠀⠁⣾⣿⡏⢠⣿⣿⠿⠛⠋⠉⠀⠀⠀\r\n");
		print!("⣿⣿⠲⢿⣿⣿⣿⣿⡿⠋⢰⣿⣿⠋⠀⠀⠀⢻⣿⣿⣿⠇⠀⠀⠀⠀⠙⠛⠀⠀⠉⠁⠀⠀⠀⠀⠀⠀⠀\r\n");
		print!("⠹⢿⣷⣶⣿⣿⠿⠋⠀⠀⠈⠙⠃⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\r\n");
		print!("⠀⠀⠈⠉⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⣤⣤⣴⣶⣦⣤⡀⠀\r\n");
		print!("⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⣀⡀⠀⠀⠀⠀⠀⠀⠀⣠⡇⢰⣶⣶⣾⡿⠷⣿⣿⣿⡟⠛⣉⣿⣿⣿⠆\r\n");
		print!("⠀⠀⠀⠀⠀⠀⢀⣤⣶⣿⣿⡎⣿⣿⣦⠀⠀⠀⢀⣤⣾⠟⢀⣿⣿⡟⣁⠀⠀⣸⣿⣿⣤⣾⣿⡿⠛⠁⠀\r\n");
		print!("⠀⠀⠀⠀⣠⣾⣿⡿⠛⠉⢿⣦⠘⣿⣿⡆⠀⢠⣾⣿⠋⠀⣼⣿⣿⣿⠿⠷⢠⣿⣿⣿⠿⢻⣿⣧⠀⠀⠀\r\n");
		print!("⠀⠀⠀⣴⣿⣿⠋⠀⠀⠀⢸⣿⣇⢹⣿⣷⣰⣿⣿⠃⠀⢠⣿⣿⢃⣀⣤⣤⣾⣿⡟⠀⠀⠀⢻⣿⣆⠀⠀\r\n");
		print!("⠀⠀⠀⣿⣿⡇⠀⠀⢀⣴⣿⣿⡟⠀⣿⣿⣿⣿⠃⠀⠀⣾⣿⣿⡿⠿⠛⢛⣿⡟⠀⠀⠀⠀⠀⠻⠿⠀⠀\r\n");
		print!("⠀⠀⠀⠹⣿⣿⣶⣾⣿⣿⣿⠟⠁⠀⠸⢿⣿⠇⠀⠀⠀⠛⠛⠁⠀⠀⠀⠀⠀⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀\r\n");
		print!("⠀⠀⠀⠀⠈⠙⠛⠛⠛⠋⠁⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀⠀\r\n");
	} else {
		println!("+{}+\r", "-".repeat((WIDTH * 2) as usize));
		for row in &grid {
			print!("|");
			for cell in row {
				print!("{}", cell);
			}
			println!("|\r");
		}
		println!("+{}+\r", "-".repeat((WIDTH * 2) as usize));
		println!("Score: {}", score);
	}


}

fn main() {
	enable_raw_mode().unwrap();

	let mut snake = Snake {
		body: vec![Position { x: 10, y: 10 }],
		direction: Direction::Right,
	};

	let mut running = true;
	let mut game_over = false;
	let mut initial_food = spawn_food(&snake);
	let mut score: u64 = 0;

	while running {
		if poll(Duration::from_millis(0)).unwrap() {
				if let Event::Key(key_event) = read().unwrap() {
						match key_event.code {
								KeyCode::Char('q') => { running = false } // quit,
								KeyCode::Char('w') => { if snake.direction != Direction::Down {
									snake.direction = Direction::Up
								}}
								KeyCode::Char('a') => { if snake.direction != Direction::Right {
									snake.direction = Direction::Left
								}}
								KeyCode::Char('s') => { if snake.direction != Direction::Up {
									snake.direction = Direction::Down
								}}
								KeyCode::Char('d') => { if snake.direction != Direction::Left {
									snake.direction = Direction::Right
								}}
								_ => {}
						}
				}
		}

		// move snake
		match move_snake(&mut snake, &mut initial_food, &mut score) {
			false => { running = false; game_over = true }
			true => {}
		};
		// render
		render(&snake, &initial_food, &game_over, &mut score);

		// sleep
		sleep(
			Duration::from_millis(200 - score)
		);
	}

	disable_raw_mode().unwrap();
}
