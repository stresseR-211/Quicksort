use sdl2::pixels::Color;
use sdl2::rect::{Point, Rect};
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
	fn new(rect: Rect, label: &str, func: fn(&mut Vec<i32>) -> (), arg: &mut Vec<i32>) -> Self;
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
pub enum anchor {
	instant,
	quick,
	normal,
	slow,
}
impl anchor {
	pub fn name(&self) -> &'static str {
		match self {
			anchor::instant => "inst",
			anchor::quick => "qck",
			anchor::normal => "norm",
			anchor::slow => "slow",
		}
	}
	pub fn delay_ms(&self) -> u64 {
		match self {
			anchor::instant => 0,
			anchor::quick => 200,
			anchor::normal => 800,
			anchor::slow => 1500,
		}
	}
}
fn draw_text(canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, font: &Font, text: &str, centre: Point, colour: Color) -> Result<(), String> {
	let surface = font.render(text).blended(colour).map_err(|e| e.to_string())?;
	let texture = tc.create_texture_from_surface(&surface).map_err(|e| e.to_string())?;
	let q = texture.query();
	canvas.copy(&texture, None, Some(Rect::from_center(centre, q.width, q.height)))
}
const HEAD_W: u32 = 20;
const HEAD_H: u32 = 40;
pub struct slider {
	path: Rect,
	head: Rect,
	label_rect: Rect,
	label: String,
	held: bool,
	colour: Color,
	anchors: [anchor; 4],
	pub delay: u64,
}
pub trait slider_t {
	fn new(path: Rect) -> Self;
	fn anchor_x(&self, i: usize) -> i32;
	fn nearest(&self) -> usize;
	fn change_delay(&mut self, i: usize) -> ();
	fn switch_head_colour(&mut self) -> ();
	fn move_slider(&mut self, x: i32) -> ();
	fn snap(&mut self) -> ();
	fn on_click(&mut self, x: i32, y: i32) -> ();
	fn on_motion(&mut self, x: i32) -> ();
	fn on_release(&mut self) -> ();
	fn draw(&self, canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String>;
}
impl slider_t for slider {
	fn new(path: Rect) -> Self {
		let label_rect = Rect::new(path.x(), path.y() - 60, path.width(), 30);
		let mut s = Self {
			path,
			head: Rect::new(0, 0, HEAD_W, HEAD_H),
			label_rect,
			label: String::new(),
			held: false,
			colour: Color::RGB(40, 40, 40),
			anchors: [anchor::instant, anchor::quick, anchor::normal, anchor::slow],
			delay: 0,
		};
		let cy = s.path.center().y();
		s.head.center_on((s.anchor_x(0), cy));
		s.change_delay(0);
		s
	}
	fn anchor_x(&self, i: usize) -> i32 {
		self.path.x() + self.path.width() as i32 * i as i32 / (self.anchors.len() as i32 - 1)
	}
	fn nearest(&self) -> usize {
		let cx = self.head.center().x();
		(0..self.anchors.len()).min_by_key(|&i| (self.anchor_x(i) - cx).abs()).unwrap()
	}
	fn change_delay(&mut self, i: usize) -> () {
		self.delay = self.anchors[i].delay_ms();
		self.label = self.anchors[i].name().to_string();
	}
	fn switch_head_colour(&mut self) -> () {
		self.colour = if self.held {
			Color::RGB(12, 78, 0)
		} else {
			Color::RGB(40, 40, 40)
		};
	}
	fn move_slider(&mut self, x: i32) -> () {
		let x = x.clamp(self.path.x(), self.path.right());
		self.head.center_on((x, self.path.center().y()));
		self.label = self.anchors[self.nearest()].name().to_string();
	}
	fn snap(&mut self) -> () {
		let i = self.nearest();
		self.head.center_on((self.anchor_x(i), self.path.center().y()));
		self.change_delay(i);
	}
	fn on_click(&mut self, x: i32, y: i32) -> () {
		if self.head.contains_point((x, y)) {
			self.held = true;
			self.switch_head_colour();
		}
	}
	fn on_motion(&mut self, x: i32) -> () {
		if self.held {
			self.move_slider(x);
		}
	}
	fn on_release(&mut self) -> () {
		if self.held {
			self.held = false;
			self.snap();
			self.switch_head_colour();
		}
	}
	fn draw(&self, canvas: &mut Canvas<Window>, tc: &TextureCreator<WindowContext>, font: &Font) -> Result<(), String> {
		let cy = self.path.center().y();
		canvas.set_draw_color(Color::RGB(90, 90, 90));
		canvas.fill_rect(self.path)?;
		for i in 0..self.anchors.len() {
			canvas.fill_rect(Rect::from_center((self.anchor_x(i), cy), 4, 20))?;
		}
		canvas.set_draw_color(self.colour);
		canvas.fill_rect(self.head)?;
		draw_text(canvas, tc, font, &self.label, self.label_rect.center(), Color::RGB(255, 255, 255))
	}
}