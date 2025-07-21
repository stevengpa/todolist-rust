use serde::Deserialize;
use uuid::Uuid;

#[derive(Deserialize, Debug)]
pub struct Task {
	id: Uuid,
	task: String,
	description: String,
}

pub trait MemDB {
	fn add(&mut self, task: Task);
	fn delete(&mut self, id: Uuid);
	fn todos(&self) -> &Vec<Task>;
}

pub struct MemDBImpl {
	todo: Vec<Task>,
}

impl MemDBImpl {
	pub fn new() -> Self {
		Self { todo: Vec::new() }
	}
}

impl MemDB for MemDBImpl {
	fn add(&mut self, task: Task) {
		self.todo.push(task);
	}

	fn delete(&mut self, id: Uuid) {
		self.todo.retain(|task| task.id != id)
	}

	fn todos(&self) -> &Vec<Task> {
		&self.todo
	}
}