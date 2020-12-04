use async_trait::async_trait;
use std::collections::HashMap;


use crate::config::ModuleItem;
mod logger;
use self::logger::*;



#[async_trait]
pub trait ServiceModule {
	fn get_name (&self) -> &'static str;
	async fn async_send (&self, _msg: String) -> bool;
}



pub struct ServiceManager {
	m_modules_map: HashMap<&'static str, Box<(dyn ServiceModule + 'static)>>,
}

//_ret.send (&LogMsg::new (Level::INFO, String::from ("start program.")));
impl ServiceManager {
	pub async fn new (_modules: &Vec<ModuleItem>) -> ServiceManager {
		let mut _ret = ServiceManager {
			m_modules_map: HashMap::new (),
		};
		for _module_item in _modules {
			match match _module_item.m_type.as_str () {
				"built-in" => match _module_item.m_name.as_str () {
					"logger" => Some (Box::new (Logger::new (&_module_item.m_param))),
					_ => None,
				},
				_ => None,
			} {
				Some (_module) => {
					_ret.m_modules_map.insert (_module.get_name (), _module);
					()
				},
				None => println! ("load module[{}] failed.", _module_item.m_type),
			};
		}
		_ret
		// let mut _sm = ServiceManager {
		// 	m_thread: ServiceDepends4Thread::new (move |_msg: String| {
		// 		let _success = match _msg.find ('|') {
		// 			Some (_size) => {
		// 				match _modules_map.get (&_msg[.._size]) {
		// 					Some (mut _sm) => {
		// 						let _content = (&_msg[_size+1..]).to_string ();
		// 						match _sm.lock () {
		// 						    Ok(mut _sm1) => _sm1.send (_content),
		// 						    Err(_) => false
		// 						}
		// 					},
		// 					None => false,
		// 				}
		// 			},
		// 			None => false,
		// 		};
		// 	}),
		// };
	}

	pub async fn async_send (&self, _module_name: &str, _msg: String) -> bool {
		//self.m_thread.send (format! ("{}|{}", module_name, content));
		match self.m_modules_map.get (_module_name) {
		    Some(_module) => _module.async_send (_msg).await,
		    None => {
				let _err_str = format! ("cannot load module[{}], please check cfg file.", _module_name);
				if _module_name == "logger" {
					println! ("{}", _err_str);
				} else {
					self.async_send ("logger", _err_str);
				}
				false
			},
		}
	}

	async fn async_logger (&self, _module: String, _level: Level, _content: String) -> bool {
		let _msg = LogMsg::new (_module, _level, _content);
		match _msg.to_string () {
			Some (_str) => self.async_send ("logger", _str).await,
			None => {
				println! ("cannot serilize object LogMsg: [{:?}]", _msg);
				false
			}
		}
	}
	pub async fn async_logger_trace (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::TRCE, _content).await }
	pub async fn async_logger_debug (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::DEBG, _content).await }
	pub async fn async_logger_info (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::INFO, _content).await }
	pub async fn async_logger_warning (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::WARN, _content).await }
	pub async fn async_logger_error (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::EROR, _content).await }
	pub async fn async_logger_critical (&self, _module: String, _content: String) -> bool { self.async_logger (_module, Level::CRIT, _content).await }
}

impl Drop for ServiceManager {
	fn drop (&mut self) {
	}
}
