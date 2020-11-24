use std::collections::HashMap;



pub trait ServiceModule {
	fn get_name (&mut self) -> &String;
	async fn send (&mut self, content: &str) -> bool;
}

pub struct ServiceManager {
	m_modules: HashMap<u32, Rc<ServiceModule>>,
}

impl ServiceManager {
	pub fn new (&mut self) {
		//
	}
}
