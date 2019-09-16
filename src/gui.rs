use std::cmp;
// Graphical user interface in terminal
pub struct GUI {
	phase: u8,
	max_phase: u8,
	first_line: String,
	third_line: String,
	fourth_line: String,
	ws_buffer: Vec<String>,

}

impl GUI {
	pub fn init(expression_length : usize) -> Self {
		let max_guesses : u8 = 8;
		let line0 = String::from("Guess the expression:");
		let mut line2 = String::from("Wrong guesses: (0/x)");
		line2 = line2.replace('x', &(max_guesses.to_string()));
		let line3 = String::with_capacity(usize::from(4 * max_guesses));
		let mut buffer : Vec<String> = Vec::with_capacity(4); 
		for _ in 0..buffer.capacity() {
			buffer.push(String::new());
		}
		let mut max_length = 
			cmp::max(
				line0.len(), cmp::max(
					expression_length, cmp::max(
						line2.len(), line3.len()
					)
				)
			);		
		max_length += 2;
		for _ in 0..(max_length - line0.len()) {
			buffer[0].push(' ');
		}
		for _ in 0..(max_length - expression_length) {
			buffer[1].push(' ');
		}
		for _ in 0..(max_length - line2.len()) {
			buffer[2].push(' ');
		}
		for _ in 0..(max_length - line3.len()) {
			buffer[3].push(' ');
		}
		// return gui
		GUI {
			phase: 0,
			max_phase: max_guesses,
			first_line: line0,
			third_line: line2, 
			fourth_line: line3,
			ws_buffer: buffer  
		}
	}

	pub fn update(&mut self, user_guess : &char, is_correct : bool) -> bool {
		if self.phase < self.max_phase {
			if !is_correct {
				self.phase += 1;
				self.third_line = self.third_line.replacen(char::is_numeric, &((self.phase).to_string()), 1);
			}
			self.fourth_line.push(*user_guess);
			self.fourth_line.push(' ');
			self.ws_buffer[3].pop();
			self.ws_buffer[3].pop();

			true
		} else {
			false
		}
	}

	pub fn print(&self, displayed_expression : &String) {
		match self.phase {
			0 => {
				println!("{}{}\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			1 => {
				println!("{}{}\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			2 => {
				println!("{}{}|\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}|\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			3 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}|\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			4 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}|\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			5 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}| \\o/\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			6 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}| \\o/\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|   |\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			7 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}| \\o/\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|   |\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|  / \\\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			8 => {
				println!("{}{}|----\r\n", self.first_line, self.ws_buffer[0]);
				println!("{}{}|   x\r\n", displayed_expression, self.ws_buffer[1]);
				println!("{}{}|  /|\\\r\n", self.third_line, self.ws_buffer[2]);
				println!("{}{}|  / \\\r\n", self.fourth_line, self.ws_buffer[3]);
			},
			_ => println!("Something went wrong!"),
		}
	}

	pub fn get_first_line(&self) -> &String {
		&(self.first_line)
	}
	pub fn get_third_line(&self) -> &String {
		&(self.third_line)
	}
	pub fn get_fourth_line(&self) -> &String {
		&(self.fourth_line)
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn initialize_gui() {
		let length : usize = 12;
		let gui = GUI::init(length);
		assert_eq!(gui.get_first_line(), "Guess the expression:");
		assert_eq!(gui.get_third_line(), "Wrong guesses: (0/8)");
	}

	#[test]
	fn update_gui() {
		let exp = String::from("What's up?");
		let mut gui = GUI::init(exp.len());
		let user_input = 'a';
		let mut i : i32 = 0;
		gui.print(&exp);
		while gui.update(&user_input, false) { 
			gui.print(&exp);
			i += 1;
		}

		assert_eq!(i,8);
	}
}



