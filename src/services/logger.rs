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

impl LogMsg {
	pub fn new (_module: String, _level: Level, _content: String) -> LogMsg {
		LogMsg {
			m_module: _module,
			m_level: _level,
			m_content: _content,
		}
	}
	fn write_log (&self, _log_path: &String) {
		//println! ("recv {:?}", msg);
		let _time = SystemTime::now ();
		let _time: DateTime<Local> = _time.into ();

		let _date = _time.format ("%Y%m%d").to_string ();
		let _file_path = format! ("{}{}.log", _log_path, _date);
		let mut _file = match OpenOptions::new ().append (true).open (_file_path.clone ()) {
			Ok (_file) => Some (_file),
			Err (_) => match File::create (_file_path.clone ()) {
			    Ok(_file) => Some (_file),
			    Err(_err) => {
					println! ("Create File Failed: {:?}", _err);
					None
				},
			},
		};

		let _date = _time.format ("%Y%m%d-%H%M%S").to_string ();
		let _content = format! ("[{}][{:?}][{}]  {}\n", _date, self.m_level, self.m_module, self.m_content);
		match _file {
			Some (mut _file) => match _file.write_all (_content.as_bytes ()) {
			    Ok(_) => (),
			    Err(_err) => {
					println! ("Write File Failed: {:?}", _err);
					println! ("{}", _content);
				},
			},
			None => println! ("{}", _content),
		}
	}
}

pub struct Logger {
	m_thread: ServiceDepends4Thread,
	//m_log_path: String,
}

impl ServiceModule for Logger {
	fn get_name (&self) -> &'static str {
		"logger"
	}
	fn send (&mut self, content: String) -> bool {
		self.m_thread.send (content)
	}
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
		let _log_path2 = _log_path.clone ();
		//let _log_path3 = _log_path.clone ();
		let mut _ret = Logger {
			m_thread: ServiceDepends4Thread::new (move |_msg: String| {
				let _msg: Result<LogMsg, serde_json::Error> = serde_json::from_str (&_msg [..]);
				match _msg {
					Ok (_msg) => _msg.write_log (&_log_path),
					Err (_) => (),
				};
			}),
			//m_log_path: _log_path3,
		};
		_ret.send (String::from ("logger"), Level::CRIT, String::from ("Program Start."));
		_ret
	}
	fn send (&mut self, _module: String, _level: Level, _content: String) -> bool {
		match serde_json::to_string (&LogMsg::new (_module, _level, _content)) {
			Ok (_str) => self.m_thread.send (_str),
			Err (_) => false,
		}
	}
}

impl Drop for Logger {
	fn drop (&mut self) {
		// TODO: 调用不到
		//LogMsg::new (String::from ("logger"), Level::CRIT, String::from ("Program Stop.")).write_log (&self.m_log_path);
	}
}
