use rand::rngs::StdRng;
use rand::SeedableRng;
use rand::prelude::SliceRandom;

pub fn train_test_split<T: std::clone::Clone, U: std::clone::Clone>(data: &Vec<Vec<T>>, target: &Vec<U>, test_size: f64, random_seed: Option<u64>) -> (Vec<Vec<T>>, Vec<Vec<T>>, Vec<U>, Vec<U>) {

	let mut x_train = Vec::<Vec::<T>>::new();
	let mut x_test = Vec::<Vec::<T>>::new();
	let mut y_train = Vec::<U>::new();
	let mut y_test = Vec::<U>::new();

	// use the specified seed, and if no seed, a random u64
	let mut rng = StdRng::seed_from_u64(random_seed.unwrap_or_else(|| rand::random::<u64>() ) );

	// a vector with all the index range
	let mut index: Vec<i32> = (0..data.len() as i32).collect();
	let test_len = ((data.len() as f64) * test_size).round() as usize;

	index.shuffle(&mut rng);

	for (pos, index) in index.iter().enumerate() {

		let usize_index: usize = *index as usize;

		if pos < test_len {
			x_test.push(data.get(usize_index).unwrap().to_vec());
			y_test.push(target.get(usize_index).unwrap().clone());
		} else {
			x_train.push(data.get(usize_index).unwrap().to_vec());
			y_train.push(target.get(usize_index).unwrap().clone());
		}
	}

	(x_train, x_test, y_train, y_test)

}
