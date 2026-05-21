//! A database resource accepts a stream of `DatabaseQuery` and responds with a stream of `DatabaseResponseItem`.

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Value {
	Real(f64),
	Integer(i64),
	Text(String),
	Blob(Vec<u8>),
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DatabaseQuery {
	request_id: u32,
	query: String,
	params: Vec<Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum DatabaseResponseItem {
	Definition {
		request_id: u32,
		rows_updated: u32,
		rows_selected: u32,
		error: String,
		column_names: Vec<String>,
	},
	Row {
		request_id: u32,
		values: Vec<Value>,
	},
}
