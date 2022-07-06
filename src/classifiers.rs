
pub trait Classifier {
    fn predict(&self, new_data: &Vec<Vec<f64>>) -> Vec<i32>;
    fn fit(&self, data: &Vec<Vec<f64>>, targets: &Vec<i32>);
}

pub mod knn_classifier;
