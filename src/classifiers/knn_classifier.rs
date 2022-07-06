use std::collections::HashMap;
use crate::classifiers::Classifier;

pub struct KNNClassifier {
    k: i32,
    distance_function: fn(&Vec<f64>, &Vec<f64>) -> f64,
    data: Option<Vec<Vec<f64>>>,
    targets: Option<Vec<i32>>,
}

impl KNNClassifier {

    pub fn new(k_value: i32, new_distance_function: fn(&Vec<f64>, &Vec<f64>) -> f64) -> KNNClassifier {
        KNNClassifier {
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
        for (index, point) in self.data.iter().enumerate() {
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


impl Classifier for KNNClassifier {

    fn fit(&self, data: &Vec<Vec<f64>>, targets: &Vec<i32>) {
        self.data = Some(data.clone());
        self.targets = Some(targets.clone());
    }


    fn predict(&self, new_data: &Vec<Vec<f64>>) -> Vec<i32> {
        let mut predictions = Vec::new();

        for instance in new_data {
            // compute the k neighbors
            let neighbors = self.compute_k_neighbors(instance);

            // count the class of the neighbors
            let mut count = HashMap::new();

            for neighbor in neighbors {
				let neighbor_index: usize = neighbor.try_into().unwrap();
                let neighbor_class = self.targets.unwrap().get(neighbor_index);
                let counter = count.entry(neighbor_class).or_insert(0);
                *counter += 1;
            }

            // predict the class
            let prediction: i32 = count.iter() // iterate over the HashMap
                                       .max_by( |a, b| a.1.cmp(&b.1) ) // get the max value using
                                                                       // the value
                                       .map( |k, _v| k ); // map the result to only get the
                                                          // predicted class

            predictions.push(prediction);

        }

        predictions

    }


}
