use async_std::{io::prelude::WriteExt, fs::{File, OpenOptions}};
use async_trait::async_trait;
use chrono::DateTime;
use chrono::prelude::*;
use serde::{Serialize,Deserialize};
use std::collections::HashMap;
use std::time::SystemTime;

//extern mod pub_trait;
//mod services;
use crate::ServiceModule;



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
	async fn write_log (&self, _log_path: String) {
		//println! ("recv {:?}", msg);
		let _time = SystemTime::now ();
		let _time: DateTime<Local> = _time.into ();

		let _date = _time.format ("%Y%m%d").to_string ();
		let _log_path = format! ("{}{}.log", _log_path, _date);
		let mut _file = match OpenOptions::new ().append (true).open (_log_path.clone ()).await {
			Ok (_file) => Some (_file),
			Err (_) => match File::create (_log_path).await {
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
			Some (mut _file) => match _file.write_all (_content.as_bytes ()).await {
			    Ok(_) => (),
			    Err(_err) => {
					println! ("Write File Failed: {:?}", _err);
					println! ("{}", _content);
				},
			},
			None => println! ("{}", _content),
		}
	}
	pub fn to_string (&self) -> Option<String> {
		match serde_json::to_string (&self) {
			Ok (_str) => Some (_str),
			Err (_) => None,
		}
	}
}

pub struct Logger {
	m_log_path: String,
}

#[async_trait]
impl ServiceModule for Logger {
	fn get_name (&self) -> &'static str {
		"logger"
	}
	async fn async_send (&self, _msg: String) -> bool {
		let _msg: Result<LogMsg, serde_json::Error> = serde_json::from_str (&_msg [..]);
		match _msg {
			Ok (_msg) => {
				_msg.write_log (self.m_log_path.clone ()).await;
				true
			},
			Err (_) => false,
		}
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
		//_ret.send (String::from ("logger"), Level::CRIT, String::from ("Program Start."));
		Logger {
			m_log_path: _log_path,
		}
	}
	// fn send (&mut self, _module: String, _level: Level, _content: String) -> bool {
	// 	match serde_json::to_string (&LogMsg::new (_module, _level, _content)) {
	// 		Ok (_str) => self.m_thread.send (_str),
	// 		Err (_) => false,
	// 	}
	// }
}

impl Drop for Logger {
	fn drop (&mut self) {
		// TODO: 调用不到
		//LogMsg::new (String::from ("logger"), Level::CRIT, String::from ("Program Stop.")).write_log (&self.m_log_path);
	}
}
