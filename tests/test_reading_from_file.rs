use hangman::expression::Expression;
use hangman::common::*;
use std::fs;
use std::env;

#[test]
fn read_from_dummy_file() {
	let file_contents = String::from("Alabama\nAlaska\nArkansas\nOhio\nWyoming");
	assert_eq!(file_contents.lines().count(), 5);
	
	let line_vector : Vec<&str> = file_contents.split('\n').collect();
	assert_eq!(line_vector.len(), 5);

	let line_num = 4;

	let state = String::from(line_vector[line_num]);
	assert_eq!(state, "Wyoming");

	let mask = vec![0,2,5,6];
	
	let mut random_line = Expression::init(&state, &mask);

	assert_eq!(random_line.get_displayed_line(), "_y_mi__");

	for c in (b'a'..=b'z').map(char::from) {
		if random_line.check_if_solved() {
			assert_eq!(c, 'x');
			break;
		}
		if random_line.update_displayed_line(&c) {
			println!("{}", c);
		}
	}

	assert_eq!(random_line.get_displayed_line(), "Wyoming");
}

#[test]
fn read_from_local_file() {
	let mut path_to_file = env::current_dir().unwrap();
	path_to_file.push("data/us_states.txt");
	let contents = fs::read_to_string(path_to_file)
		.expect("Error while reading file!");
	let contents_as_vec : Vec<&str> = contents.split('\n').collect();

	let line_num = 20;

	let state = String::from(contents_as_vec[line_num]);

	assert_eq!(state, "Massachusetts");

	let state_length = state.len();

	let mut mask : Vec<i32> = generate_random_mask_vector(state_length);

	if !mask.contains(&7) {
		mask.push(7) // add 'u' to the mask as well (i.e. delete from state name)
	}

	let mut random_line = Expression::init(&state, &mut mask);

	assert_ne!(random_line.get_displayed_line(), "Massachusetts");

	for c in (b'a'..=b'z').map(char::from) {
		if random_line.check_if_solved() {
			assert_eq!(c, 'v'); // 'u' character should be the last to find
			break;
		}
		random_line.update_displayed_line(&c);
	}

	assert_eq!(random_line.get_displayed_line(), "Massachusetts");
}

#[test]
fn read_random_line_from_local_file() {
	let mut path_to_file = env::current_dir().unwrap();
	path_to_file.push("data/us_states.txt");

	let state : String = generate_random_line_from_file(&path_to_file);

	let mask : Vec<i32> = generate_random_mask_vector(state.len());

	let random_line = Expression::init(&state, &mask);

	assert!(!random_line.get_line().starts_with("Q"));
	assert!(!random_line.get_line().starts_with("X"));
	assert!(!random_line.get_line().starts_with("Z"));
}
