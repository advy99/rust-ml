use crate::regressors::Regressor;

pub struct KNNRegressor {
    k: i32,
    distance_function: fn(&Vec<f64>, &Vec<f64>) -> f64,
    data: Option<Vec<Vec<f64>>>,
    targets: Option<Vec<f64>>,
}

impl KNNRegressor {

    pub fn new(k_value: i32, new_distance_function: fn(&Vec<f64>, &Vec<f64>) -> f64) -> KNNRegressor {
        KNNRegressor {
            k: k_value,
            distance_function: new_distance_function,
			data: None,
			targets: None,
        }
    }

    fn compute_k_neighbors(&self, instance: &Vec<f64>) -> Vec<i32> {
        let mut neighbors: Vec<i32> = Vec::new();
        let mut distances = Vec::<(f64, i32)>::new();

        // Compute the distances between the instance and all the data
        for (index, point) in self.data.as_ref().unwrap().iter().enumerate() {
            let distance = (self.distance_function)(point, instance);
            distances.push((distance, index.try_into().unwrap() ));
        }

        distances.sort_by( |a, b| a.0.partial_cmp(&b.0).unwrap());

        // get the index of the k closest instances in training
        for i in 0..self.k {
			let index: usize = i.try_into().unwrap();
            neighbors.push( distances.get(index).unwrap().1 );
        }

        neighbors
    }

}


impl Classifier for KNNRegressor {

    fn fit(&mut self, data: &Vec<Vec<f64>>, targets: &Vec<f64>) {
        self.data = Some(data.clone());
        self.targets = Some(targets.clone());
    }


    fn predict(&self, new_data: &Vec<Vec<f64>>) -> Vec<f64> {
        let mut predictions = Vec::new();

        for instance in new_data {
            // compute the k neighbors
            let neighbors = self.compute_k_neighbors(instance);

			let mut prediction = 0.0;

			for neighbor in neighbors {
				prediction += self.target[neighbor];
			}

			prediction = prediction / (self.k as f64)


            predictions.push(prediction);

        }

        predictions

    }


}
