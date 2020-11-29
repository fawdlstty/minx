use std::{collections::HashMap, sync::Arc, rc::Rc};
use core::ops::FnOnce;
use std::time::Duration;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::mpsc;
use std::thread;
use std::thread::JoinHandle;

mod logger;
use serde::Serialize;

use crate::config::ModuleItem;

use self::logger::*;



pub struct ServiceDepends4Thread {
	m_sender: Option<mpsc::Sender<String>>,
	m_expect_run: Arc<bool>,
	m_thread: Option<JoinHandle<()>>,
}

impl ServiceDepends4Thread {
	pub fn new<T: Fn (&str) + Send + Copy + Sync> (on_callback: Arc<T>) -> ServiceDepends4Thread {
		let (_sender, _receiver): (Sender<String>, Receiver<String>) = mpsc::channel ();
		let mut _sd4t = ServiceDepends4Thread {
			m_sender: Some (_sender),
			m_expect_run: Arc::new (true),
			m_thread: None,
		};
		let _expect_run_weak = Arc::downgrade (&_sd4t.m_expect_run);
		_sd4t.m_thread = Some (thread::spawn (move || {
			loop {
				match _receiver.recv_timeout (Duration::from_millis (10)) {
					Ok (_msg) => {
						let _msg = &_msg [..];
						match _msg {
							"" => break,
							_ => {
								on_callback (_msg);
								()
							}
						}
					},
					Err (_) => break,
				}
			}
		}));
		_sd4t
	}

	pub fn send<T: Serialize> (&mut self, obj: &T) -> bool {
		match serde_json::to_string (obj) {
			Ok (_content) => self.send_str (_content),
			Err (_) => false,
		}
	}

	pub fn send_str (&mut self, content: String) -> bool {
		match self.m_sender.send (content) {
			Ok (_) => true,
			Err (_) => false,
		}
	}
}

impl Drop for ServiceDepends4Thread {
	fn drop (&mut self) {
		self.send_str (String::from (""));
		self.m_expect_run = false;
		// match self.m_thread.take () {
		// 	Some (_thread) => match _thread.join () { _ => (), },
		// 	None => (),
		// }
	}
}



pub trait ServiceModule {
	fn get_name (&self) -> &'static str;
	//fn send (&mut self, content: String) -> bool;
}



pub struct ServiceManager {
	m_modules: HashMap<&'static str, Box<dyn ServiceModule>>,
}

//_ret.send (&LogMsg::new (Level::INFO, String::from ("start program.")));
impl ServiceManager {
	pub fn new (_modules: &Vec<ModuleItem>) -> ServiceManager {
		let mut _sm = ServiceManager {
			m_modules: HashMap::new (),
		};
		for _module_item in _modules {
			let _sm_item: Option<Box<dyn ServiceModule>> = match _module_item.m_type.as_str () {
				"built-in" => match _module_item.m_name.as_str () {
					"logger" => Some (Box::new (Logger::new (/*&_module_item.m_param*/))),
					_ => None,
				},
				_ => None,
			};
			match _sm_item {
				Some (_obj) => {
					_sm.m_modules.insert (_obj.get_name (), _obj);
				},
				None => ()
			}
		}
		_sm
	}

	pub fn send (&mut self, module_name: &str, content: &str) {
		//
	}
}
