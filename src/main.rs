use sdl2;
mod sort;
fn main() -> Result<(), String> {
	let mut array: Vec<i32> = [0,1,2,3,4,5,6,7,8,9].to_vec(); 
	let sdl2context = sdl2::init()?;
	let sysvid = sdl2context.video()?;
	let window = sysvid.window("QV", 800, 600).position_centered().build().map_err(|e| e.to_string())?;
	let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
	let mut event_pump = sdl2context.event_pump()?;
	while true {
		for event in event_pump.poll_iter() {
			match event {
				sdl2::event::Event::Quit { .. } | sdl2::event::Event::KeyDown { keycode: Some(sdl2::keyboard::Keycode::Escape), .. } => return Ok(()),
				_ => {}
			}
		}
		canvas.set_draw_color(sdl2::pixels::Color::RGB(64, 0, 64));
		canvas.clear();
		canvas.present();
		std::thread::sleep(std::time::Duration::from_millis(10));
	}
	sort::shuffle(&mut array);
	sort::quicksort(&mut array);
	Ok(())
}
