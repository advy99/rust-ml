use std::collections::HashMap;

use crate::transformers::TargetTransformer;

pub struct LabelEncoder {
    num_unique_values: i32,
    transformation: HashMap<String, i32>,
}

impl LabelEncoder {
	pub fn new() -> LabelEncoder {
		LabelEncoder {
			num_unique_values: 0,
			transformation: HashMap::new(),
		}
	}

}


impl TargetTransformer for LabelEncoder {
	fn transform(&self, new_data: &Vec<String>) -> Vec<i32> {
		let mut result = Vec::<i32>::new();

		for data in new_data {
			result.push( *self.transformation.get(data).unwrap() );
		}

		result
	}

    fn fit(&mut self, data: &Vec<String>) {
		self.num_unique_values = 0;
		self.transformation = HashMap::new();

		for element in data {
			if !self.transformation.contains_key(element) {
				self.transformation.entry(String::from(element) ).or_insert(self.num_unique_values);
				self.num_unique_values += 1;
			}
		}

	}
}
