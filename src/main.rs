pub mod classifiers;
pub mod math;
pub mod datasets;
pub mod transformers;
pub mod metrics;

use std::env;

use math::distance::euclidean_distance;

use classifiers::knn_classifier::KNNClassifier;
use classifiers::Classifier;

use transformers::label_encoder::LabelEncoder;
use transformers::TargetTransformer;

use metrics::classification_metrics::*;

use datasets::reader::*;
use datasets::split::train_test_split;

fn main() {

	let args: Vec<String> = env::args().collect();

	if args.len() != 3 {
		panic!("ERROR: This program must be launched with two arguments.\n \t {} <data_file> <test_percentage>", &args[0]);
	}

	let data_file = &args[1];
	let test_percentage: f64 = args[2].parse::<f64>().unwrap();

	println!("{:?}", args);

    let mut knn_classifier = KNNClassifier::new(3, euclidean_distance);
	let mut label_encoder = LabelEncoder::new();

	let (x_iris, y_iris) = read_x_y_from_csv(data_file,  true, '#');

	label_encoder.fit(&y_iris);

	let y_iris_int = label_encoder.transform(&y_iris);


	let (x_train_iris, x_test_iris, y_train_iris, y_test_iris) = train_test_split(&x_iris, &y_iris_int, test_percentage, None);

	println!("Dataset len: {}", x_iris.len());
	println!("Train len: {}", x_train_iris.len());
	println!("Test len: {}", x_test_iris.len());
	println!("Percentage in test data: {}", test_percentage);


	knn_classifier.fit(&x_train_iris, &y_train_iris);

	let test_predictions = knn_classifier.predict(&x_test_iris);
	let test_accuracy = accuracy_score(&y_test_iris, &test_predictions);

	let train_predictions = knn_classifier.predict(&x_train_iris);
	let train_accuracy = accuracy_score(&y_train_iris, &train_predictions);

	println!("Train accuracy: {}", train_accuracy);
	println!("Test accuracy: {}", test_accuracy);

}
