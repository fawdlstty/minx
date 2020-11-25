use std::{sync::Arc, collections::HashMap};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use std::thread;
use std::thread::JoinHandle;

mod logger;
use crate::config::ModuleItem;

use self::logger::Logger;



pub enum ServiceCallback { GoOn, Quit, }

pub struct ServiceDepends4Thread {
	m_sender: mpsc::Sender<String>,
	m_thread: Option<JoinHandle<()>>,
	m_quit: Box<dyn FnOnce ()>,
}

impl ServiceDepends4Thread {
	pub fn new<Fc: FnOnce (String) -> ServiceCallback + Send + Copy + 'static, Fq: FnOnce () + Send + Copy + 'static> (on_callback: Fc, on_quit: Fq) -> ServiceDepends4Thread {
		let (_sender, _receiver) = mpsc::channel ();
		ServiceDepends4Thread {
			m_sender: _sender,
			m_thread: Some (thread::spawn (move || {
				loop {
					match _receiver.recv () {
						Ok (_msg) => match f (_msg) {
							ServiceCallback::GoOn => (),
							ServiceCallback::Quit => break,
						},
						Err (_) => break,
					}
				}
			})),
			m_quit: Box::new (on_quit),
		}
	}

	pub fn send (&mut self, content: String) -> bool {
		match self.m_sender.send (content) {
			Ok (_) => true,
			Err (_) => false,
		}
	}
}

impl Drop for ServiceDepends4Thread {
	fn drop (&mut self) {
		self.m_quit ();
		match self.m_thread.take () {
			Some (_handle) => match _handle.join () { _ => (), },
			None => (),
		}
		match self.m_thread.take () {
			Some (_thread) => match _thread.join () { Ok (_) => (), Err (_) => (), },
			None => (),
		};
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
