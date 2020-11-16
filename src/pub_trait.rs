pub trait ServerModule {
	fn send (&mut self, &str) -> Future<bool>;
}
