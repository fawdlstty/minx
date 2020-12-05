use async_trait::async_trait;
use std::{future::Future, collections::HashMap};

use crate::config::ModuleItem;

use self::logger::{Level, LogMsg, Logger};


//use crate::config::ModuleItem;
mod logger;
//use self::logger::*;



#[async_trait]
pub trait ServiceModule {
	// 获取模块名称
	fn get_name (&self) -> &'static str;
	// 判断是否包括入口
	fn is_entry (&self) -> bool;
	// 进入入口
	async fn async_entry (&self);
	// 发送信息
	async fn async_send (&self, _msg: String) -> bool;
}



pub struct ServiceManager {
	m_modules_map: HashMap<&'static str, Box<(dyn ServiceModule + 'static)>>,
}

impl ServiceManager {
	pub fn new (_modules: Vec<ModuleItem>) -> ServiceManager {
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
