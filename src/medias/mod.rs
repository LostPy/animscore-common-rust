use std::collections::HashMap;


pub mod coefficient;


pub trait MediaScore {

	fn get_coefficients(&self) -> &HashMap<String, coefficient::Coefficient> {
		unimplemented!();
	}

	fn get_mut_coefficients(&mut self) -> &mut HashMap<String, coefficient::Coefficient> {
		unimplemented!();
	}

	fn get_coeff_by_name(&self, name: String) -> Option<&coefficient::Coefficient> {
		self.get_coefficients().get(&name)
	}

	fn get_mut_coeff_by_name(&mut self, name: String) -> Option<&mut coefficient::Coefficient> {
		self.get_mut_coefficients().get_mut(&name)
	}

	fn add_coefficient(&mut self, name: String, value: u8) {
		self.get_mut_coefficients().insert(
			name.to_lowercase().to_string(),
			coefficient::Coefficient::new(name, value)
		);
	}

	fn remove_coefficient(&mut self, name: String) -> Option<(String, coefficient::Coefficient)> {
		self.get_mut_coefficients()
			.remove_entry(&name.to_lowercase().to_string())
	}

	fn set_coeff_value(&mut self, name: String, new_value: u8) {
		match self.get_mut_coeff_by_name(name) {
			Some(coeff) => {coeff.set_value(new_value);}
			_ => {}
		}
	}

	fn calculate(&self, values: &HashMap<String, u8>) -> u8 {
		calculate_score(self.get_coefficients(), values)
	}
}


/// Calcul a score (a ponderate mean) with coefficients and values
/// 
/// # Arguments
/// 
/// * `coefficients` - A [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) 
/// with the name (String) of the coefficient in key and the coefficient in value.
/// * `values` - A [HashMap](https://doc.rust-lang.org/std/collections/struct.HashMap.html) 
/// with the name (String) of the coefficient in key and the score / value associated in value.
fn calculate_score(coefficients: &HashMap<String, coefficient::Coefficient>, values: &HashMap<String, u8>) -> u8 {
	let mut sum: u16 = 0;
	let mut sum_coeff: u16 = 0;
	for (key, value) in values.iter() {
		match coefficients.get(&key.to_lowercase().to_string()) {
			Some(coeff) => {
				sum += coeff.apply(value.clone() as u16);
				sum_coeff += coeff.get_value() as u16;	
			},
			_ => {}
		}
	}
	(sum / sum_coeff) as u8
}


#[derive(Debug)]
pub struct AnimeScore {
	coefficients: HashMap<String, coefficient::Coefficient>,
}

impl AnimeScore {
	pub fn new() -> Self {
		let mut coefficients: HashMap<String, coefficient::Coefficient> = HashMap::new();
		for name in ["originality", "story", "drawings", "animation", "music", "assessment"].iter() {
			coefficients.insert(name.to_string(), coefficient::Coefficient::new(name.to_string(), 1));
		}
		Self {coefficients}
	}
}

impl MediaScore for AnimeScore {
	fn get_coefficients(&self) -> &HashMap<String, coefficient::Coefficient> {
	    &self.coefficients
	}

	fn get_mut_coefficients(&mut self) -> &mut HashMap<String, coefficient::Coefficient> {
	    &mut self.coefficients
	}
}

#[derive(Debug)]
pub struct MangaScore {
	coefficients: HashMap<String, coefficient::Coefficient>,
}

impl MangaScore {
	pub fn new() -> Self {
		let mut coefficients: HashMap<String, coefficient::Coefficient> = HashMap::new();
		for name in ["originality", "story", "drawings", "assessment"].iter() {
			coefficients.insert(name.to_string(), coefficient::Coefficient::new(name.to_string(), 1));
		}
		Self {coefficients}
	}
}

impl MediaScore for MangaScore {
	fn get_coefficients(&self) -> &HashMap<String, coefficient::Coefficient> {
	    &self.coefficients
	}

	fn get_mut_coefficients(&mut self) -> &mut HashMap<String, coefficient::Coefficient> {
	    &mut self.coefficients
	}
}

