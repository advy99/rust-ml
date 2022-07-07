pub trait TargetTransformer {
    fn transform(&self, new_data: &Vec<String>) -> Vec<i32>;
    fn fit(&mut self, data: &Vec<String>);
}

pub mod label_encoder;
