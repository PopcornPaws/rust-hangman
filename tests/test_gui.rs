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

#[test]
fn solve_expression() {
	let state = String::from("abcdefghi");
	let mask : Vec<u32> = vec![0,1,2,3,4,5,6,7,8];
	let mut random_line = Expression::init(&state, &mask);
	assert_eq!(random_line.get_displayed_line(), "_________");
	let mut gui = GUI::init(random_line.get_len());

	let input_string = String::from("abcdefghijklmno");
	let mut user_input_iter = input_string.chars();

	let mut user_input = user_input_iter.next().unwrap();

	while !random_line.check_if_solved() && 
		gui.update(&user_input, random_line.update_displayed_line(&user_input)) {
			gui.print(&(random_line.get_displayed_line()));
			user_input = user_input_iter.next().unwrap();

	}

	assert!(random_line.check_if_solved());
	assert_eq!(random_line.get_displayed_line(), "abcdefghi");
	assert_eq!(user_input_iter.next().unwrap(), 'k');
}
