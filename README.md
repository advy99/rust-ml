# rust-ml

Implementations of multiple machine learning methos in Rust. Similar to Python [scikit-learn](https://scikit-learn.org/).

I mantain this project in my free time, so maybe this is not updated regularly.

## Structure

This repo is structured inside the `src` folder. Inside this folder you can find a Rust file for every module. All the modules are for generic things, such as `classifiers`, `metrics`, and so on. Every module has a submodule folder, where all the specifics things are implemented, for example, inside the `classifiers` module is the `knn_classifier` submodule, wichs implements `KNNClassifier`, or the `metrics` module, who inside has the `classification_metrics` and `regression_metrics` submodules.


## Contributing

Feel free to submit a PR to the repo! The only requirement is to repect the structure of the project.

## Progress

So far is implemented:

- Classification: 
  * [x] KNNClassifier
  * [ ] DecisionTreeClassifier
  * [ ] RandomForestClassifier
  * [ ] LogisticRegressionClassifier
  * [ ] SVMClassifier
  
  
- Regression:
  * [x] KNNRegressor
  * [ ] LinearRegressor
  * [ ] SVMRegressor

- Metrics:
  * [x] Accuracy
  * [x] RMSE
  * [x] MSE
  * [x] MAE
  * [ ] F1-score
  * [ ] Precision
  * [ ] Recall
  
  
- Others:
  * [x] LabelEncoder
  * [x] train_test_split
  * [x] CSV reader
  * [ ] OneHotEncoder
  * [ ] Z-Score Scaler
  * [ ] MinMaxScaler 
