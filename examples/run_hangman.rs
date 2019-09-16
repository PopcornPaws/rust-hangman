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

	//add loop
	let mut user_input = String::new();
	io::stdin().read_line(&mut user_input)
		.expect("Failed to read input");

	let user_input : char = user_input.trim().parse()
		.expect("Please enter a character!");

	while !random_line.check_if_solved() &&
		gui.update(&user_input, random_line.update_displayed_line(&user_input)) {
			gui.print(&(random_line.get_displayed_line()));
			let mut user_input = String::new();
			io::stdin().read_line(&mut user_input)
				.expect("Failed to read input");
			let user_input : char = user_input.trim().parse()
				.expect("Please enter a character!");
	}
	if random_line.check_if_solved() {
		println!("You win!\n");
	} else if gui.update(&user_input, true) {
		println!("You lost!\n")
	}
}
