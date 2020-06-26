use super::User;
use crate::environment::Environment;
use crate::error::Error;
use actix::{prelude::*, Actor, Addr};
use log::*;
use std::collections::HashMap;
use std::fs::File;
use std::path::PathBuf;
use std::sync::mpsc::{channel, RecvError};

/// Starts the user manager actor in a new system thread
/// and returns the addr for the actor if successful
pub fn start_user_manager(env: &Environment) -> Result<Addr<UserManager>, RecvError> {
	info!("Starting user manager");

	let data_path = env.data_path.clone();
	let (sender, receiver) = channel();
	std::thread::spawn(move || {
		let mut user_manager = UserManager::new(data_path);
		if let Err(err) = user_manager.load() {
			error!("Error loading users: {:?}", err);
		}
		let user_manager_system = System::new("User Manager");
		let user_manager_addr = user_manager.start();
		if sender.send(user_manager_addr).is_err() {
			error!("Error sending user manager address to main thread");
		}
		if user_manager_system.run().is_err() {
			error!("User Manager Actix System encountered an error");
		}
	});
	receiver.recv()
}

pub struct UserManager {
	users: HashMap<String, User>,
	data_path: PathBuf,
}

impl Actor for UserManager {
	type Context = Context<Self>;
}

impl UserManager {
	fn new(data_path: PathBuf) -> UserManager {
		UserManager {
			users: HashMap::new(),
			data_path,
		}
	}

	/// Load all users from data/users.json
	fn load(&mut self) -> Result<(), Error> {
		let mut path = self.data_path.clone();
		path.push("users.json");
		let file = File::open(&path)?;
		let mut users: HashMap<String, User> = serde_json::from_reader(std::io::BufReader::new(file))?;
		for (address, user) in users.iter_mut() {
			user.master_address = address.clone()
		}

		self.users = users;
		info!("Loaded {} users", self.users.len());
		// TODO: merge changes with unsaved
		// TODO: watch file for changes

		Ok(())
	}

	/// Create new user
	/// Return: User
	fn create(&mut self) {
		let user = User::new();
		self.users.insert(user.master_address.clone(), user);
	}

	/// Get user based on master_address
	/// Return: User or None
	fn get(&self, master_address: &str) -> Option<User> {
		self.users.get(master_address).cloned()
	}

	/// Save all users in memory to file
	fn save(&mut self) -> Result<(), Error> {
		let mut path = self.data_path.clone();
		path.push("users.json");
		let file = File::open(path)?;
		serde_json::to_writer_pretty(file, &self.users)?;

		Ok(())
	}
}

pub struct UserRequest {
	pub address: String,
}

impl Message for UserRequest {
	type Result = Option<User>;
}

impl Handler<UserRequest> for UserManager {
	type Result = Option<User>;

	fn handle(&mut self, msg: UserRequest, ctx: &mut Self::Context) -> Self::Result {
		self.get(&msg.address)
	}
}
