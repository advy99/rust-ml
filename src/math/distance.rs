pub fn minkowski_distance(point_a: &Vec<f64>, point_b: &Vec<f64>, p: i32) -> f64 {
	let mut distance: f64 = 0.0;

	let iterator = point_a.iter().zip(point_b.iter());

	for (point_a_value, point_b_value) in iterator {
		let abs_difference = (point_a_value - point_b_value).abs();
		distance = distance + abs_difference.powi(p);
	}

	distance.powf( 1.0 / (p as f64))

}


pub fn euclidean_distance(point_a: &Vec<f64>, point_b: &Vec<f64>) -> f64 {
	minkowski_distance(point_a, point_b, 2)
}

pub fn manhattan_distance(point_a: &Vec<f64>, point_b: &Vec<f64>) -> f64 {
	minkowski_distance(point_a, point_b, 1)
}
