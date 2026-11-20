#![allow(warnings)]
use std::time::{Duration, Instant};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::mouse::MouseButton;
use sdl2::pixels::Color;
use sdl2::rect::Rect;
use crate::ui::{column_t, button_t, slider_t};
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
	let array: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
	let mut display_array = array.clone();
	let sdl2context = sdl2::init()?;
	let sysvid = sdl2context.video()?;
	let window = sysvid.window("QV", 800, 600).position_centered().build().map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl2context.event_pump()?;
	let ttfcontext = sdl2::ttf::init().map_err(|e| e.to_string())?;
	let font = ttfcontext.load_font("Arial.ttf", 40)?;
	let font2 = ttfcontext.load_font("Arial.ttf", 80)?;
	let texture_creator = canvas.texture_creator();
	let mut column_array = build_columns(&display_array);
	let mut shuffle_button = ui::button::new(Rect::new(520, 400, 200, 70), "shuffle");
	let mut qsort_button = ui::button::new(Rect::new(520, 270, 100, 100), "Q");
	let mut slider = ui::slider::new(Rect::new(520, 100, 240, 8));
	let mut steps = sort::StepsDeque::new();
	let mut last_step = Instant::now();
	let mut sorting_active = false;
	while (true) {
		for event in event_pump.poll_iter() {
			match event {
				Event::Quit { .. } | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => return Ok(()),
				Event::KeyDown { keycode: Some(Keycode::Q), .. } => {
					if !sorting_active || steps.is_empty() {
						steps.clear();
						steps = sort::quicksort(&display_array);
						last_step = Instant::now();
						sorting_active = !steps.is_empty();
					}
				}
				Event::KeyDown { keycode: Some(Keycode::S), .. } => {
					if !sorting_active || steps.is_empty() {
						steps.clear();
						steps = sort::shuffle(&display_array);
						last_step = Instant::now();
						sorting_active = !steps.is_empty();
					}
				}
				Event::MouseMotion { x, y, .. } => {
					shuffle_button.on_hover(shuffle_button.contains(x, y));
					qsort_button.on_hover(qsort_button.contains(x, y));
					slider.on_motion(x);
				}
				Event::MouseButtonDown { mouse_btn: MouseButton::Left, x, y, .. } => {
					if shuffle_button.contains(x, y) { 
						shuffle_button.on_click(); 
					}
					if qsort_button.contains(x, y) { 
						qsort_button.on_click(); 
					}
					slider.on_click(x, y);
				}
				Event::MouseButtonUp { mouse_btn: MouseButton::Left, x, y, .. } => {
					if shuffle_button.on_release(x, y) {
						if !sorting_active || steps.is_empty() {
							steps.clear();
							steps = sort::shuffle(&display_array);
							last_step = Instant::now();
							sorting_active = !steps.is_empty();
						}
					}
					if qsort_button.on_release(x, y) {
						if !sorting_active || steps.is_empty() {
							steps.clear();
							steps = sort::quicksort(&display_array);
							last_step = Instant::now();
							sorting_active = !steps.is_empty();
						}
					}
					slider.on_release();
				}
				_ => {}
			}
		}
		canvas.set_draw_color(Color::RGB(64, 0, 64));
		canvas.clear();
		if !steps.is_empty() {
			if slider.delay == 0 {
				while let Some((i, j)) = steps.pop_front() {
					display_array.swap(i, j);
				}
				sorting_active = false;
			} else if last_step.elapsed() >= Duration::from_millis(slider.delay) {
				if let Some((i, j)) = steps.pop_front() {
					display_array.swap(i, j);
				}
				last_step = Instant::now();
				if steps.is_empty() {
					sorting_active = false;
				}
			}
		} else {
			sorting_active = false;
		}
		column_array = build_columns(&display_array);
		shuffle_button.draw(&mut canvas, &texture_creator, &font)?;
		qsort_button.draw(&mut canvas, &texture_creator, &font2)?;
		slider.draw(&mut canvas, &texture_creator, &font)?;
		for column in &column_array {
			column.draw(&mut canvas)?;
		}
		canvas.present();
		std::thread::sleep(std::time::Duration::from_millis(21));
	}
	Ok(())
}