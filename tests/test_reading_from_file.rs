use rand::{thread_rng, Rng};
use hangman::expression::Expression;

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
