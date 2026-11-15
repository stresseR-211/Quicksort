use sdl2::pixels::Color;
use sdl2::rect::Rect;
use sdl2::render::{Canvas, TextureCreator};
use sdl2::video::{Window, WindowContext};
use sdl2::ttf::Font;
pub struct column {
	leftpad: i32,
	rect: Rect,
	colour: Color,
}
pub trait column_t {
	fn new(rect: Rect) -> Self;
	fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String>;
	fn move_to(&mut self, to: i32) -> ();
	fn switch_colour(&mut self) -> ();
}
impl column_t for column {
	fn new(rect: Rect) -> Self {
		Self {
			leftpad: 6,
			rect,
			colour: Color::RGB(255, 255, 255),
		}
	}
	fn draw(&self, canvas: &mut Canvas<Window>) -> Result<(), String> {
		canvas.set_draw_color(self.colour);
		canvas.fill_rect(self.rect)
	}
	fn move_to(&mut self, to: i32) -> () {
		let offset = self.rect.width() as i32 + self.leftpad;
		self.rect.set_x(offset * to);
	}
	fn switch_colour(&mut self) -> () {
		self.colour = if self.colour == Color::RGB(255, 255, 255) {
			Color::RGB(255, 255, 255)
		} else {
			Color::RGB(255, 0, 0)
		};
	}
}
pub struct button {
	base_rect: Rect,
	rect: Rect,
	label: String,
	held: bool,
	func: fn(&mut Vec<i32>),
	arg: *mut Vec<i32>,
	colour: Color,
}
pub trait button_t {
	fn new(rect: Rect, label: &str, func: fn(&mut Vec<i32>), arg: &mut Vec<i32>) -> Self;
	fn draw(&self, canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String>;
	fn contains(&self, x: i32, y: i32) -> bool;
	fn resize(&mut self, scale: f32) -> ();
	fn on_hover(&mut self, hovered: bool) -> ();
	fn on_click(&mut self) -> ();
	fn on_release(&mut self, x: i32, y: i32) -> bool;
	fn switch_colour(&mut self, random_bool: bool) -> ();
}
impl button_t for button {
	fn new(rect: Rect, label: &str, func: fn(&mut Vec<i32>), arg: &mut Vec<i32>) -> Self {
		Self {
			base_rect: rect,
			rect,
			label: label.to_string(),
			held: false,
			func,
			arg,
			colour: Color::RGB(4, 252, 17),
		}
	}
	fn draw(&self, canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String> {
		canvas.set_draw_color(self.colour);
		canvas.fill_rect(self.rect)?;
		let surface = font.render(&self.label).blended(Color::RGB(255, 255, 255)).map_err(|e| e.to_string())?;
		let texture = tc.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
		let q = texture.query();
		let target = Rect::from_center(self.rect.center(), q.width, q.height);
		canvas.copy(&texture, None, Some(target))
	}
	fn contains(&self, x: i32, y: i32) -> bool {
		self.rect.contains_point((x, y))
	}
	fn resize(&mut self, scale: f32) -> () {
		let cx = self.base_rect.x() + self.base_rect.width() as i32 / 2;
		let cy = self.base_rect.y() + self.base_rect.height() as i32 / 2;
		let new_w = (self.base_rect.width() as f32 * scale) as u32;
		let new_h = (self.base_rect.height() as f32 * scale) as u32;
		self.rect = Rect::new(cx - new_w as i32 / 2, cy - new_h as i32 / 2, new_w, new_h);
	}
	fn on_hover(&mut self, hovered: bool) -> () {
		self.switch_colour(hovered);
	}
	fn on_click(&mut self) -> () {
		self.held = true;
		self.resize(1.2);
	}
	fn on_release(&mut self, x: i32, y: i32) -> bool {
		let fire = self.held && self.contains(x, y);
		if self.held {
			self.held = false;
			self.resize(1.0);
		}
		if fire {
			unsafe {
				(self.func)(&mut *self.arg);
			}
		}
		fire
	}
	fn switch_colour(&mut self, random_bool: bool) -> () {
		self.colour = if random_bool {
			Color::RGB(4, 252, 17)
		} else {
			Color::RGB(127, 255, 255)
		};
	}
}