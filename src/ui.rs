pub struct column {
	rect: sdl2::rect::Rect,
	blocksize: i8,
	leftpad: i8,
	coloured: bool,
};
pub trait column_t {
	fn move(&mut self, to: i8) -> ();
	fn switch_colour(&mut self) -> ();
};
impl column_t for column {
	fn new(rect: sdl2::rect::Rect) -> Self {
		Self {
			rect,
			blocksize: 42,
			leftpad: 6,
			coloured: false,
		}
	}
	fn move(&mut self, to: i8) -> () {
		let offset: i8 = self.width + self.leftpad;
		self.rect.x = offset * to;
	}
	fn switch_colour() -> () {
		self.coloured = !self.coloured;
	}
}
pub struct button {
	rect: sdl2::rect::Rect,
	text: sdl2::ttf::Text,
	hovered: bool,
	held: bool,
	func: fn() -> (),
};
pub trait button_t {
	fn resize(&mut self, scale: f16) -> ();
	fn on_hover(&mut self) -> ();
	fn on_click(&mut self) -> ();
	fn on_release(&mut self) -> ();
	fn switch_colour(&mut self) -> ();
};
impl button_t for button {
	
	fn resize(&mut self, scale: f16) {}
	fn on_hover(&mut self) {
		switch
	}
	fn on_click(&mut self) {
		self.resize(1.2);
	}
	fn on_release(&mut self) {
		(self.func)();
		self.resize(1.0);
	}
	fn switch_colour(&mut self) -> () {}
}