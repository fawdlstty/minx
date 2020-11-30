use chrono::DateTime;
use chrono::prelude::*;
use serde::{Serialize,Deserialize};
use std::{collections::HashMap, fs::{File, OpenOptions}};
use std::io::Write;
use std::time::SystemTime;

//extern mod pub_trait;
//mod services;
use crate::ServiceModule;

use super::ServiceDepends4Thread;



#[derive(Serialize, Deserialize, Debug)]
pub enum Level { TRCE, DEBG, INFO, WARN, EROR, CRIT, }

#[derive(Serialize, Deserialize, Debug)]
pub struct LogMsg {
	m_module: String,
	m_level: Level,
	m_content: String,
}

// impl LogMsg {
// 	pub fn new (_module: String, _level: Level, _content: String) -> LogMsg {
// 		LogMsg {
// 			m_module: _module,
// 			m_level: _level,
// 			m_content: _content,
// 		}
// 	}
// }

pub struct Logger {
	m_thread: ServiceDepends4Thread,
}

impl ServiceModule for Logger {
	fn get_name (&self) -> &'static str {
		"logger"
	}
	//fn send (&mut self, content: String) -> bool {
	//	self.m_thread.send_str (content)
	//}
}

impl Logger {
	pub fn new (_param: &HashMap<String, String>) -> Logger {
		let mut _log_path = _param ["log_path"].to_string ();
		match _log_path.len () {
			0 => _log_path = String::from ("logs/"),
			_ => {
				let ch = _log_path.as_bytes () [_log_path.len () - 1];
				match ch as char {
					'/' => (),
					_ => _log_path.push_str ("/"),
				}
			},
		};
		Logger {
			m_thread: ServiceDepends4Thread::new (move |_msg| {
				let _msg: Result<LogMsg, serde_json::Error> = serde_json::from_str (_msg);
				match _msg {
					Ok (_msg) => {
						let _time = SystemTime::now ();
						let _time: DateTime<Local> = _time.into ();

						let _date = _time.format ("%Y%m%d").to_string ();
						//let _log_path = get_config_item ("logger", "log_path").unwrap ();
						let _file_path = format! ("{}{}.log", _log_path, _date);
						let mut _file = match OpenOptions::new ().append (true).open (_file_path.clone ()) {
							Ok (_file) => _file,
							Err (_) => File::create (_file_path.clone ()).unwrap (),
						};

						let _date = _time.format ("%Y%m%d-%H%M%S").to_string ();
						let _content = format! ("[{}][{:?}]  {}\n", _date, _msg.m_level, _msg.m_content);
						_file.write_all (_content.as_bytes ()).unwrap ();
						//println! ("recv {:?}", msg);
					},
					Err (_) => (),
				};
			}),
		}
	}
}

impl Drop for Logger {
	fn drop (&mut self) {
		//self.m_sender.clone ().send (LogMsg {
		//	m_level: Level::CRIT,
		//	m_content: String::from ("stop program.")
		//}).unwrap ();
		//match self.m_thread.take () {
		//	Some (_handle) => match _handle.join () { _ => (), },
		//	None => (),
		//}
		////self.m_thread.take ().unwrap ().join ().unwrap ();
	}
}
