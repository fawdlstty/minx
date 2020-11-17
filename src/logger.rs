use std::fs::File;
use std::thread;
use std::time::Duration;



pub enum Level { trace, debug, info, warning, error, critical }

pub struct LogMsg {
	m_level: Level,
	m_content: String,
}

pub struct Logger {
	m_log_path: String,
	m_tx: String,
	m_thread: JoinHandle,
}

impl Logger {
	pub fn new (&mut self, _log_path: &String) {
		self.m_log_path = _log_path;
		let (_tx, _rx) = mpsc::channel ();
		self.m_tx = _tx;
		tx.send (String::from ("start program."));
		self.m_thread = thread::spawn (move || {
			while (true) {
				match _rx.recv () {
					Ok (_msg) => println ("recv {:?}", _msg),
					Err (_) break,
				}
			}
		});
	}

	pub fn write () {
		//
	}
}
