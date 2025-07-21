use serde::Deserialize;

use envy;

#[derive(Deserialize, Debug)]
pub struct Config {
	pub app_host: String,
	pub app_port: u16,
	pub database_host: String,
	pub database_port: u16,
	pub database_user: String,
	pub database_pwd: String,
	pub database_name: String,
}
