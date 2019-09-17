use hangman::{common::*, expression::Expression, gui::GUI};
use std::env;
use std::io;

fn main() {
	let mut path_to_file = env::current_dir().unwrap();
	path_to_file.push("data/us_states.txt");
	let state : String = generate_random_line_from_file(&path_to_file);
	let mask : Vec<u32> = generate_random_mask_vector(state.len());

	let mut random_line = Expression::init(&state, &mask);
	let mut gui = GUI::init(random_line.get_len());

	gui.print(&(random_line.get_displayed_line()));

	loop {
		if random_line.check_if_solved() {
			println!("You win!\n");
			break;
		}

		let mut user_input = String::new();
		io::stdin().read_line(&mut user_input)
			.expect("Failed to read input!");
		let user_input : char = match user_input.trim().parse() {
			Ok(ch) => ch,
			Err(_) => continue,
		};

		if gui.update(&user_input, random_line.update_displayed_line(&user_input)) {
			gui.print(&(random_line.get_displayed_line()));
		} else {
			println!("You lost!\n");
			break;
		}
	}
}
