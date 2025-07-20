use serde::{Serialize, Deserialize};
use validator::Validate;

#[derive(Debug, Validate, Serialize, Deserialize)]
pub struct NewToDoRequest {
	#[validate(length(min = 1, max = 100, message = "Task must be betwen 1 and 100 characters"))]
	pub task: String,
	#[validate(length(min = 2, max = 200, message = "Description must be between 2 and 200 characters"))]
	pub description: String,
}
