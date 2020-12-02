use std::collections::HashMap;
use std::sync::Arc;
use std::sync::mpsc;
use std::sync::mpsc::{Sender, Receiver};
use std::sync::Mutex;
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

use crate::config::ModuleItem;
mod logger;
use self::logger::*;



pub struct ServiceDepends4Thread {
	m_sender: Mutex<mpsc::Sender<Option<String>>>,
	m_thread: Option<JoinHandle<()>>,
}

impl ServiceDepends4Thread {
	pub fn new<Tc: Fn (String) + Send + Sync + 'static, Tq: Fn () + Send + Sync + 'static> (on_callback: Tc, on_quit: Tq) -> ServiceDepends4Thread {
		let (_sender, _receiver): (Sender<Option<String>>, Receiver<Option<String>>) = mpsc::channel ();
		let mut _sd4t = ServiceDepends4Thread {
			m_sender: Mutex::new (_sender),
			m_thread: None,
		};
		_sd4t.m_thread = Some (thread::spawn (move || {
			loop {
				match _receiver.recv_timeout (Duration::from_millis (10)) {
					Ok (_msg) => match _msg {
						Some (_msg) => on_callback (_msg),
						None => break,
					},
					Err (_) => (),
				}
			}
			on_quit ();
		}));
		_sd4t
	}

	//pub fn send<T: Serialize> (&mut self, obj: &T) -> bool {
	//	match serde_json::to_string (obj) {
	//		Ok (_content) => self.send_str (_content),
	//		Err (_) => false,
	//	}
	//}

	pub fn send (&mut self, content: String) -> bool {
		match self.m_sender.lock () {
		    Ok(_sender) => match _sender.send (Some (content)) {
				Ok (_) => true,
				Err (_) => false,
			},
		    Err(_) => false
		}
	}
}

impl Drop for ServiceDepends4Thread {
	fn drop (&mut self) {
		//self.send (None);
		match self.m_thread.take () {
			Some (_thread) => match _thread.join () { _ => (), },
			None => (),
		}
	}
}



pub trait ServiceModule: Send + Sync {
	fn get_name (&self) -> &'static str;
	fn send (&mut self, content: String) -> bool;
}



pub struct ServiceManager {
	m_thread: ServiceDepends4Thread,
	m_modules: Arc<HashMap<&'static str, Arc<Mutex<(dyn ServiceModule + 'static)>>>>,
}

//_ret.send (&LogMsg::new (Level::INFO, String::from ("start program.")));
impl ServiceManager {
	pub fn new (_modules: &Vec<ModuleItem>) -> ServiceManager {
		let mut _modules_map: HashMap<&'static str, Arc<Mutex<(dyn ServiceModule + 'static)>>> = HashMap::new ();
		for _module_item in _modules {
			let _sm_item: Option<Arc<Mutex<(dyn ServiceModule + 'static)>>> = match _module_item.m_type.as_str () {
				"built-in" => match _module_item.m_name.as_str () {
					"logger" => Some (Arc::new (Mutex::new (Logger::new (&_module_item.m_param)))),
					_ => None,
				},
				_ => None,
			};
			match _sm_item {
				Some (_obj) => {
					//_modules_map.insert (_obj.get_mut ().unwrap ().get_name (), _obj);
					match _obj.lock () {
					    Ok(_obj1) => {
							_modules_map.insert (_obj1.get_name (), _obj.clone ());
							()
						},
					    Err(_) => (),
					};
				},
				None => ()
			}
		}
		let _modules_map = Arc::new (_modules_map);
		let _modules_map_weak = Arc::downgrade (&_modules_map);
		let _modules_map_weak2 = _modules_map_weak.clone ();
		let mut _sm = ServiceManager {
			m_thread: ServiceDepends4Thread::new (move |_msg: String| {
				let _modules_map = _modules_map_weak.upgrade ().unwrap ();
				let _success = match _msg.find ('|') {
					Some (_size) => {
						//println! ("str1:[{}], str2:[{}]", &_msg[.._size], &_msg[_size+1..]);
						match _modules_map.get (&_msg[.._size]) {
							//Some (mut _sm) => _sm.send ((&_msg[_size+1..]).to_string ()),
							Some (mut _sm) => {
								let _content = (&_msg[_size+1..]).to_string ();
								match _sm.lock () {
								    Ok(mut _sm1) => _sm1.send (_content),
								    Err(_) => false
								}
							},
							None => false,
						}
					},
					None => false,
				};
			}, move || {}),
			m_modules: _modules_map,
		};
		_sm
	}

	pub fn send (&mut self, module_name: &str, content: &str) {
		self.m_thread.send (format! ("{}|{}", module_name, content));
	}
}

impl Drop for ServiceManager {
	fn drop (&mut self) {
		//self.m_thread.send (None);
	}
}
