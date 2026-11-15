use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::ui::column_t;
use crate::ui::button_t;
use crate::ui::slider_t;
mod sort;
mod ui;
fn build_columns(array: &[i32]) -> Vec<ui::column> {
	array.iter().enumerate().map(|(idx, &v)| {
		let h = 42 * v.max(1) as u32;
		let x = 6 + 48 * idx as i32;
		ui::column::new(Rect::new(x, 560 - h as i32, 42, h))
	}).collect()
}
fn main() -> Result<(), String> {
	let mut array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let sdl2context = sdl2::init()?;
	let sysvid = sdl2context.video()?;
	let window = sysvid.window("QV", 800, 600).build().map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl2context.event_pump()?;
	let mut column_array = build_columns(&array);
	let ttfcontext = sdl2::ttf::init().map_err(|e| e.to_string())?;
	let font = ttfcontext.load_font("Arial.ttf", 40)?;
	let font2 = ttfcontext.load_font("Arial.ttf", 80)?;
	let texture_creator = canvas.texture_creator();
	let mut shuffle_button = ui::button::new(Rect::new(520, 400, 200, 70), "shuffle", sort::shuffle, &mut array);
	let mut qsort_button = ui::button::new(Rect::new(520, 270, 100, 100), "Q", sort::quicksort_void, &mut array);
	let mut slider = ui::slider::new(Rect::new(520, 60, 240, 8));
	let mut steps = sort::steps_deque::new();
	let mut last_step = Instant::now();
	while true {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
				Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
					steps = sort::quicksort(&mut array);
					last_step = Instant::now();
				}
				Event::KeyDown { keycode: Some(Keycode::S), .. } => {
					steps.clear();
					sort::shuffle(&mut array);
				}
				Event::MouseMotion { x, y, .. } => {
					shuffle_button.on_hover(shuffle_button.contains(x, y));
					qsort_button.on_hover(qsort_button.contains(x, y));
					slider.on_motion(x);
				}
				Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
					if shuffle_button.contains(x, y) { shuffle_button.on_click(); }
					if qsort_button.contains(x, y) { qsort_button.on_click(); }
					slider.on_click(x, y);
				}
				Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
					if shuffle_button.on_release(x, y) {
						steps.clear();
						sort::shuffle(&mut array);
					}
					if qsort_button.on_release(x, y) {
						steps = sort::quicksort(&mut array);
						last_step = Instant::now();
					}
					slider.on_release();
				}
				_ => {}
			}
		}
		canvas.set_draw_color(Color::RGB(64, 0, 64));
		canvas.clear();
		if slider.delay == 0 {
			while let Some((i, j)) = steps.pop_front() {
				array.swap(i, j);
			}
		} else if !steps.is_empty() && last_step.elapsed() >= Duration::from_millis(slider.delay) {
			if let Some((i, j)) = steps.pop_front() {
				array.swap(i, j);
			}
			last_step = Instant::now();
		}
		column_array = build_columns(&array);
		shuffle_button.draw(&mut canvas, &texture_creator, &font)?;
		qsort_button.draw(&mut canvas, &texture_creator, &font2)?;
		slider.draw(&mut canvas, &texture_creator, &font)?;
		for column in &column_array {
			column.draw(&mut canvas)?;
		}
		canvas.present();
		std::thread::sleep(std::time::Duration::from_millis(19));
	}
	Ok(())
}