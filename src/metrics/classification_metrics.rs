
pub fn accuracy_score(y_true: &Vec<i32>, y_pred: &Vec<i32>) -> f64 {
	let mut num_corrects = 0;

	for (true_value, pred_value) in y_true.iter().zip(y_pred.iter()) {
		if true_value == pred_value {
			num_corrects += 1;
		}
	}

	(num_corrects as f64) / (y_true.len() as f64)
}
