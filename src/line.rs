// data struct containing randomly selected line and the displayed line
pub struct Line<'a> {
	random_line : &'a String,
	displayed_line : String,
}

impl<'a> Line<'a> {
	pub fn init(line : &'a String, mask : &Vec<i32>) -> Self {
		let mut line = Line {
			random_line : line,
			displayed_line : line.clone(),
		};
		line.mask_display(mask);

		line
	}

	fn mask_display(&mut self, mask : &Vec<i32>) {
		// iterate through line characters and replace given indices
		self.displayed_line = self.displayed_line
			.char_indices()
			.map(|(i, x)| if mask.contains(&(i as i32)) { '_' } else { x })
			.collect()
	}

	pub fn get_len(&self) -> usize {
		self.random_line.len()
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
		let mask : Vec<i32> = vec![0,3,6];
		let mut line = Line::init(&state, &mask);

		assert_eq!(line.get_len(), state.len());
		assert_eq!(line.get_line(), &state);
		line.get_displayed_line().push('u');
		assert_eq!(line.get_displayed_line(), &String::from("_la_am_u"));
	}

	#[test]
	fn create_random_line_struct() {
		let state = String::from("Alabama");
		let mut mask : Vec<i32> = Vec::new();

		let line_length = state.len();
		let mut rng = thread_rng();

		for _ in 0..line_length / 2 {
			let mut random_num  = rng.gen_range(0, line_length as i32);
			while mask.contains(&mut random_num) {
				random_num = rng.gen_range(0, line_length as i32);
			}
			mask.push(random_num); 
		}
		assert_eq!(mask.len(), 3);

		let mut line = Line::init(&state, &mask);
		let mut displayed_line_iterator = line.get_displayed_line().char_indices();

		for i in 0..line_length {
			if mask.contains(&(i as i32)) {
				assert_eq!(Some((i, '_')), displayed_line_iterator.next())
			} else {
				assert_ne!(Some((i, '_')), displayed_line_iterator.next())
			}
		}

		assert_eq!(None, displayed_line_iterator.next());

		println!("{}", line.get_displayed_line());
	}
}




