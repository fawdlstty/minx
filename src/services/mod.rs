use async_std::task;
use async_trait::async_trait;
use std::{collections::HashMap, future::Future, sync::{Arc, Mutex}};

use crate::config::ModuleItem;

use self::logger::{Level, LogMsg, Logger};


//use crate::config::ModuleItem;
mod logger;
//use self::logger::*;



#[async_trait]
pub trait ServiceModule: Send + Sync {
	// 获取模块名称
	fn get_name (&self) -> &'static str;
	// 进入入口
	async fn async_entry (&self);
	// 发送信息
	async fn async_send (&self, _msg: String) -> bool;
}



pub struct ServiceManager {
	m_modules_map: HashMap<&'static str, Arc<(dyn ServiceModule + 'static)>>,
}

impl ServiceManager {
	pub fn new (_modules: Vec<ModuleItem>) -> ServiceManager {
		let mut _ret = ServiceManager {
			m_modules_map: HashMap::new (),
		};
		for _module_item in _modules {
			match match _module_item.m_type.as_str () {
				"built-in" => match _module_item.m_name.as_str () {
					"logger" => Some (Arc::new (Logger::new (&_module_item.m_param))),
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
	}

	async fn async_entry (&self) {
		let mut _v: Option<Arc<dyn ServiceModule>> = None;
		let mut _vs = Vec::new ();
		for (_key, _value) in &self.m_modules_map {
			match &_v {
				Some (_v1) => {
					//let _value2 = Some (_value.clone ());
					// let _f: dyn FnOnce(dyn Future<Output=()>) = move || async {
					// 	match _value2.take () {
					// 		Some (_value2) => _value2.async_entry ().await,
					// 		None => (),
					// 	}
					// };
					// _vs.push (spawn (_f.call_once ()));
					//
					// let _value2 = Mutex::new (_value.clone ());
					// _vs.push (task::spawn ((move || async {
					// 	let _value3 = _value2.lock ();
					// 	match _value3 {
					// 	    Ok(_value3) => { _value3.async_entry ().await; () },
					// 	    Err(_) => (),
					// 	};
					// })()));
					//
					let _value2 = _value.clone ();
					_vs.push (task::spawn ((move || async {
						_value2.async_entry ().await;
					})()));
				},
				None => _v = Some (_value.clone ()),
			}
		}
		match &_v {
			Some (_v1) => _v1.async_entry ().await,
			None => (),
		};
		for _item in _vs {
			_item.await;
		}
	}

	pub async fn async_send (&self, _module_name: &str, _msg: String) -> bool {
		//self.m_thread.send (format! ("{}|{}", module_name, content));
		match self.m_modules_map.get (_module_name) {
		    Some(_module) => _module.async_send (_msg).await,
		    None => {
				let _err_str = format! ("cannot load module[{}], please check cfg file.", _module_name);
				println! ("{}", _err_str);
				false
			},
		}
	}

	async fn async_logger (&self, _module: &str, _level: Level, _content: &str) -> bool {
		let _msg = LogMsg::new (String::from (_module), _level, String::from (_content));
		match _msg.to_string () {
			Some (_str) => self.async_send ("logger", _str).await,
			None => {
				println! ("cannot serilize object LogMsg: [{:?}]", _msg);
				false
			}
		}
	}
	pub async fn async_logger_trace (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::TRCE, _content).await }
	pub async fn async_logger_debug (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::DEBG, _content).await }
	pub async fn async_logger_info (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::INFO, _content).await }
	pub async fn async_logger_warning (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::WARN, _content).await }
	pub async fn async_logger_error (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::EROR, _content).await }
	pub async fn async_logger_critical (&self, _module: &str, _content: &str) -> bool { self.async_logger (_module, Level::CRIT, _content).await }
}

impl Drop for ServiceManager {
	fn drop (&mut self) {
	}
}
