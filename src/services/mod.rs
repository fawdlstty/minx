use std::{sync::Arc, collections::HashMap};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

mod logger;
use crate::config::ModuleItem;

use self::logger::Logger;



pub struct ServiceDepends_Thread {
	m_sender: mpsc::Sender<String>,
	m_thread: Option<JoinHandle<()>>,
}

impl ServiceDepends_Thread {
	pub fn new<F: FnOnce (Receiver<String>) + Send + 'static> (f: F) -> ServiceDepends_Thread {
		let (_sender, _receiver) = mpsc::channel ();
		ServiceDepends_Thread {
			m_sender: _sender,
			m_thread: Some (thread::spawn (move || {
				f (_receiver);
			}))
		}
	}
}

impl Drop for ServiceDepends_Thread {
	fn drop (&mut self) {
		match self.m_thread.take () {
			Some (_handle) => match _handle.join () { _ => (), },
			None => (),
		}
		//self.m_thread.take ().unwrap ().join ().unwrap ();
	}
}



pub trait ServiceModule {
	fn get_name (&self) -> &'static str;
	fn send (&mut self, content: &str) -> bool;
}



pub struct ServiceManager {
	m_index: u32,
	m_modules: HashMap<(u32, &'static str), Box<dyn ServiceModule>>,
}

impl ServiceManager {
	pub fn new (_modules: &Vec<ModuleItem>) -> ServiceManager {
		let mut _sm = ServiceManager {
			m_index: 0,
			m_modules: HashMap::new (),
		};
		for _module_item in _modules {
			let _sm_item: Option<Box<dyn ServiceModule>> = match _module_item.m_type.as_str () {
				"built-in" => match _module_item.m_name.as_str () {
					"logger" => Some (Box::new (Logger::new (&_module_item.m_param))),
					_ => None,
				},
				_ => None,
			};
			match _sm_item {
				Some (_obj) => {
					_sm.m_index += 1;
					_sm.m_modules.insert ((_sm.m_index, _obj.get_name ()), _obj);
				},
				None => ()
			}
		}
		_sm
	}

	pub fn send_by_id (module_id: u32, content: &str) {
		//
	}

	pub fn send_by_name (module_name: &str, content: &str) {
		//
	}
}
