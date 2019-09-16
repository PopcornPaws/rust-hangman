// data struct containing randomly selected line and the displayed line
pub struct Expression<'a> {
	random_line : &'a String,
	displayed_line : String,
	random_line_length : usize,
}

impl<'a> Expression<'a> {
	pub fn init(exp : &'a String, mask : &Vec<u32>) -> Self {
		let mut expression = Expression {
			random_line : exp,
			displayed_line : exp.clone(),
			random_line_length : exp.len(),
		};
		expression.mask_display(mask);

		expression
	}

	fn mask_display(&mut self, mask : &Vec<u32>) {
		// iterate through line characters and replace given indices
		self.displayed_line = self.displayed_line
			.char_indices()
			.map(|(i, x)| if mask.contains(&(i as u32)) { '_' } else { x })
			.collect()
	}

	pub fn update_displayed_line(&mut self, input : &char) -> bool {
		let lowercase_input = input.to_lowercase().to_string();
		let uppercase_input = lowercase_input.to_uppercase();
		if self.random_line.contains(&lowercase_input) || self.random_line.contains(&uppercase_input) {
			self.displayed_line = self.displayed_line
				.char_indices()
				.map(|(i, x)| 
					 if self.random_line.get(i..i + 1) == Some(&lowercase_input) && x == '_' { 
					 	 lowercase_input.clone() 
					 } else if self.random_line.get(i..i + 1) == Some(&uppercase_input) && x == '_' {
					 	 uppercase_input.clone()
					 } else { 
					 	 x.to_string() 
					 }
				)
				.collect();
			true
		} else {
			false
		}
	}
	
	pub fn check_if_solved(&self) -> bool {
		!self.displayed_line.contains('_')
	}

	pub fn get_len(&self) -> usize {
		self.random_line_length
	}

	pub fn get_line(&self) -> &String {
		self.random_line
	}
	
	pub fn get_displayed_line(&mut self) -> &mut String {
		&mut self.displayed_line
	}
}

#[cfg(test)]
mod tests {
	use rand::{thread_rng, Rng};
	use super::*;

	#[test]
	fn create_line_struct() {
		let state = String::from("Alabama");
		let mask : Vec<u32> = vec![0,3,6];
		let mut exp = Expression::init(&state, &mask);

		assert_eq!(exp.get_len(), state.len());
		assert_eq!(exp.get_line(), &state);
		exp.get_displayed_line().push('u');
		assert_eq!(exp.get_displayed_line(), &String::from("_la_am_u"));
	}

	#[test]
	fn create_random_line_struct() {
		let state = String::from("Alabama");
		let mut mask : Vec<u32> = Vec::new();

		let line_length = state.len();
		let mut rng = thread_rng();

		for _ in 0..line_length / 2 {
			let mut random_num  = rng.gen_range(0, line_length as u32);
			while mask.contains(&mut random_num) {
				random_num = rng.gen_range(0, line_length as u32);
			}
			mask.push(random_num); 
		}
		assert_eq!(mask.len(), 3);

		let mut exp = Expression::init(&state, &mask);
		let mut displayed_line_iterator = exp.get_displayed_line().char_indices();

		for i in 0..line_length {
			if mask.contains(&(i as u32)) {
				assert_eq!(Some((i, '_')), displayed_line_iterator.next())
			} else {
				assert_ne!(Some((i, '_')), displayed_line_iterator.next())
			}
		}

		assert_eq!(None, displayed_line_iterator.next());

		println!("{}", exp.get_displayed_line());
	}

	#[test]
	fn update_struct() {
		let state = String::from("AlaBAMa");
		let mask = vec![0,1,2,3,4,5,6]; 

		let mut exp_1 = Expression::init(&state, &mask);
		let mut exp_2 = Expression::init(&state, &mask);

		assert_eq!(exp_1.get_displayed_line(), "_______");
		assert_eq!(exp_2.get_displayed_line(), "_______");


		let mut input = 'A';
		assert!(exp_1.update_displayed_line(&mut input));

		input = 'a';
		assert!(exp_2.update_displayed_line(&mut input));

		assert_eq!(exp_1.get_displayed_line(), "A_a_A_a");
		assert_eq!(exp_2.get_displayed_line(), "A_a_A_a");

		input = 'k';

		assert!(!exp_1.update_displayed_line(&mut input));
		assert!(!exp_2.update_displayed_line(&mut input));

		assert!(!exp_1.check_if_solved());
		assert!(!exp_2.check_if_solved());

		input = 'b';
		assert!(exp_1.update_displayed_line(&mut input));
		input = 'B';
		assert!(exp_2.update_displayed_line(&mut input));
		assert_eq!(exp_1.get_displayed_line(), "A_aBA_a");
		assert_eq!(exp_2.get_displayed_line(), "A_aBA_a");

		input = 'm';
		assert!(exp_1.update_displayed_line(&mut input));
		input = 'M';
		assert!(exp_2.update_displayed_line(&mut input));
		assert_eq!(exp_1.get_displayed_line(), "A_aBAMa");
		assert_eq!(exp_2.get_displayed_line(), "A_aBAMa");

		input = 'L';
		assert!(exp_1.update_displayed_line(&mut input));
		input = 'l';
		assert!(exp_2.update_displayed_line(&mut input));
		assert_eq!(exp_1.get_displayed_line(), "AlaBAMa");
		assert_eq!(exp_2.get_displayed_line(), "AlaBAMa");

		assert!(exp_1.check_if_solved());
		assert!(exp_2.check_if_solved());
	}
}
