use sdl2::*;
mod sort;
fn main() -> Result<(), String> {
	let mut array: Vec<i32> = vec![0,1,2,3,4,5,6,7,8,9]; 
	let sdl2context = sdl2::init()?;
	let sysvid = sdl2context.video()?;
	let window = sysvid.window("QV", 800, 600).position_centered().build().map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl2context.event_pump()?;
	let shuffle_button = ui::button::new([24, 24, 300, 100], sort::shuffle, &mut array);
	let qsort_button = ui::button::new([0, 0, 100, 100], sort::quicksort, &mut array);
	let mut column_array: Vec<ui::column>;
	for i in array {
		column_array.push(ui::column::new([6 + 42 * i, 400, 42, 42 * i]));
	}
	while true {
		canvas.set_draw_color(pixels::Color::RGB(64, 0, 64));
		for event in event_pump.poll_iter() {
			match event {
				event::Event::Quit { .. } | event::Event::KeyDown { keycode: Some(keyboard::Keycode::Escape), .. } => return Ok(()),
				event::Event::KeyDown { keycode: Some(keyboard::Keycode::Q), .. } => sort::quicksort(&mut array),
				event::Event::KeyDown { keycode: Some(keyboard::Keycode::S), .. } => sort::shuffle(&mut array),
				_ => {}
			}
		}
		canvas.clear();
		canvas.present();
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
	Ok(())
}
