use chrono::DateTime;
use chrono::prelude::*;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::thread;
use std::thread::JoinHandle;
use std::time::SystemTime;
use std::sync::mpsc;

//extern mod pub_trait;
//mod pub_trait;
//use crate::pub_trait::*;



#[derive(Debug)]
pub enum Level { Trace, Debug, Info_, Warn_, Error, Critl }

pub struct LogMsg {
	m_level: Level,
	m_content: String,
}

pub struct Logger {
	m_sender: mpsc::Sender<LogMsg>,
	m_thread: JoinHandle<()>,
}

impl ServiceModule for Logger {
	async fn send (&mut self, content: &str) -> bool {
		//
	}
}

impl Logger {
	pub fn new (log_path: &String) -> Logger {
		let (_sender, _receiver) = mpsc::channel ();
		_sender.send (LogMsg {
			m_level: Level::Info_,
			m_content: String::from ("start program.")
		}).unwrap ();
		//self.m_tx = _tx;
		//tx.send (String::from ("start program."));
		let mut log_path = log_path.clone ();
		match log_path.len () {
			0 => log_path = String::from ("logs/"),
			_ => {
				let ch = log_path.as_bytes () [log_path.len () - 1];
				match ch as char {
					'/' => (),
					_ => log_path.push_str ("/"),
				}
			},
		};
		Logger {
			m_sender: _sender,
			m_thread: thread::spawn (move || {
				let mut _state = Level::Trace;
				loop {
					match _state {
						Level::Critl => break,
						_ => match _receiver.recv () {
							Ok (_msg) => {
								let _time = SystemTime::now ();
								let _time: DateTime<Local> = _time.into ();

								let _date = _time.format ("%Y%m%d").to_string ();
								let _file_path = format! ("{}{}.log", log_path, _date);
								let mut _file = match OpenOptions::new ().append (true).open (_file_path.clone ()) {
									Ok (_file) => _file,
									Err (_) => File::create (_file_path.clone ()).unwrap (),
								};

								let _date = _time.format ("%Y%m%d-%H%M%S").to_string ();
								let _content = format! ("[{}][{:?}]  {}\n", _date, _msg.m_level, _msg.m_content);
								_file.write_all (_content.as_bytes ()).unwrap ();
								_state = _msg.m_level;
								//println! ("recv {:?}", msg);
							},
							Err (_) => break,
						},
					}
				}
			})
		}
	}
}

impl Drop for Logger {
	fn drop (&mut self) {
		self.m_sender.clone ().send (LogMsg {
			m_level: Level::Critl,
			m_content: String::from ("stop program.")
		}).unwrap ();
		//self.m_thread.join ().unwrap ();
	}
}
