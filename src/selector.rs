pub struct Selector<'a, T> {
	pub data: Option<&'a T>,
}

impl<'a, T> Selector<'a, T> {
	pub fn select<R>(&self, handler: impl FnOnce(&'a T) -> R) -> Option<R> {
		self.data.map(handler)
	}

	pub async fn select_async<R>(&self, handler: impl AsyncFnOnce(&'a T) -> R) -> Option<R> {
		if let Some(data) = self.data {
			Some(handler(data).await)
		} else {
			None
		}
	}
}
