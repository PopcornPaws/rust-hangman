use rand::{thread_rng, Rng};
use std::fs;
use std::path::PathBuf;

pub fn generate_random_mask_vector(size_of_string : usize) -> Vec<i32> {
	let mut rng = thread_rng();
	let mut mask : Vec<i32> = Vec::new();

	for _ in 0..size_of_string / 2 {
		let mut random_num = rng.gen_range(0, size_of_string as i32);
		while mask.contains(&mut random_num) {
			random_num = rng.gen_range(0, size_of_string as i32);
		}
		mask.push(random_num);
	}

	mask
}

pub fn generate_random_line_from_file(file_name : &PathBuf) -> String {
	let contents = fs::read_to_string(file_name)
		.expect("Error while reading file!");

	let contents_as_vec : Vec<&str> = contents.split('\n').collect();

	let mut rng = thread_rng();
	let random_line_num : usize = rng.gen_range(0, contents_as_vec.len());

	String::from(contents_as_vec[random_line_num])
}

#[cfg(test)]
mod tests {
	use super::*;
	use std::env;

	#[test]
	fn test_random_mask_generation() {
		let exp = String::from("Just a random string");
		let exp_length = exp.len();

		let random_mask = generate_random_mask_vector(exp_length);

		assert_eq!(random_mask.len(), exp_length / 2);
	}

	#[test]
	fn test_random_line_generation() {
		let mut path_to_file = env::current_dir().unwrap();
		path_to_file.push("data/us_states.txt");

		let random_line : String = generate_random_line_from_file(&path_to_file);

		assert!(!random_line.starts_with("Q"));
		assert!(!random_line.starts_with("X"));
		assert!(!random_line.starts_with("Z"));
	}
}
