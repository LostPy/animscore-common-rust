

#[derive(Debug)]
pub struct Coefficient {
	name: String,
	value: u8,
}


impl Coefficient {
	
	pub fn new(name: String, value: u8) -> Self {
		Self {name, value}
	}

	pub fn get_name(&self) -> &String {
		&self.name
	}

	pub fn get_value(&self) -> u8 {
		self.value
	}

	pub fn set_value(&mut self, new: u8) {
		self.value = new;
	}

	pub fn apply(&self, number: u16) -> u16 {
		number * self.value as u16
	}
}