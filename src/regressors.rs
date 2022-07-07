
pub trait Regressor {
    fn predict(&self, new_data: &Vec<Vec<f64>>) -> Vec<f64>;
    fn fit(&mut self, data: &Vec<Vec<f64>>, targets: &Vec<f64>);
}

pub mod knn_regressor;
