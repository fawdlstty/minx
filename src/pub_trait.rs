pub trait ServerModule {
	async fn send (&mut self, content: &str) -> bool;
}
