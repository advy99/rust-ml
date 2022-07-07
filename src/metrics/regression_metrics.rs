pub fn mean_absolute_error(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
	let mut error = 0.0;

	for (true_value, pred_value) in y_true.iter().zip(y_pred.iter()) {
		error += (true_value - pred_value).abs();
	}

	(error as f64) / (y_true.len() as f64)
}

pub fn mean_squared_error(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
	let mut error = 0.0;

	for (true_value, pred_value) in y_true.iter().zip(y_pred.iter()) {
		error += (true_value - pred_value).powi(2);
	}

	(error as f64) / (y_true.len() as f64)

}

pub fn root_mean_squared_error(y_true: &Vec<f64>, y_pred: &Vec<f64>) -> f64 {
	mean_squared_error(y_true, y_pred).sqrt()
}
