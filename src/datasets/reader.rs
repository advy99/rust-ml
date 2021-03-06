use std::fs::*;
use std::io::{BufReader, BufRead};
use std::fmt::Debug;

pub fn read_x_y_from_csv<T: std::str::FromStr, U: std::str::FromStr>(file: &str, has_header: bool, comment_char: char) -> (Vec<Vec<T>>, Vec<U>)
where <T as std::str::FromStr>::Err: Debug, <U as std::str::FromStr>::Err: Debug
 {

	let mut data = Vec::new();
	let mut targets = Vec::new();

	let data_flow = File::open(file).expect( format!("Data file not found: {}", file).as_str() );
	let reader = BufReader::new(data_flow);

	let mut reader_lines = reader.lines();

	// if csv has header, skip first line
	if has_header{
		reader_lines.next();
	}

	for line in reader_lines {
		let unwrapped_line = line.unwrap();

		if !unwrapped_line.is_empty() && unwrapped_line.chars().nth(0) != Some(comment_char) {
			let total_columns = unwrapped_line.split(",").count();
			let line_iterator = unwrapped_line.split(",");
			let mut line_data = Vec::new();

			for (num_column, value) in line_iterator.enumerate() {
				if num_column == total_columns - 1 {
					targets.push( value.parse().unwrap() );
				} else {
					line_data.push(value.parse().unwrap());
				}
			}
			data.push(line_data);
		}

	}

	(data, targets)

}
