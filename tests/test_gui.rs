// Test all with GUI
use hangman::{common::*, expression::Expression, gui::GUI};
use std::env;

#[test]
fn generate_random_line_and_print() {
	let mut path_to_file = env::current_dir().unwrap();
	path_to_file.push("data/us_states.txt");
	let state : String = generate_random_line_from_file(&path_to_file);
	let mask : Vec<u32> = generate_random_mask_vector(state.len());

	let mut random_line = Expression::init(&state, &mask);
	let mut gui = GUI::init(random_line.get_len());

	gui.print(&(random_line.get_displayed_line()));
	let mut user_input = ':';

	while !random_line.check_if_solved() && 
		gui.update(&user_input, random_line.update_displayed_line(&user_input)) {
			gui.print(&(random_line.get_displayed_line()));
			user_input = '.';
		}

	assert!(!random_line.check_if_solved());
	assert!(!gui.update(&user_input, true));
}

	


