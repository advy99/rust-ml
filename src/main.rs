pub mod classifiers;
pub mod math;
pub mod datasets;
pub mod transformers;
pub mod metrics;

use math::distance::euclidean_distance;
use classifiers::knn_classifier::KNNClassifier;
use classifiers::Classifier;
use transformers::label_encoder::LabelEncoder;
use datasets::reader::*;
use transformers::TargetTransformer;
use metrics::classification_metrics::*;

fn main() {

    let mut knn_classifier = KNNClassifier::new(3, euclidean_distance);
	let mut label_encoder = LabelEncoder::new();

	let (x_iris, y_iris) = read_x_y_from_csv("data_examples/iris.csv",  true, '#');

	label_encoder.fit(&y_iris);

	let y_iris_int = label_encoder.transform(&y_iris);

	knn_classifier.fit(&x_iris, &y_iris_int);

	let predictions = knn_classifier.predict(&x_iris);

	let accuracy = accuracy_score(&y_iris_int, &predictions);

	println!("Train accuracy: {}", accuracy);

}
