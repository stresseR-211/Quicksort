struct column {
	height: i8;
	width: i8;
	blocksize: i16;
	coloured: bool;
};
trait column_t {
	fn create(height: i8) -> column
	fn move(to: i16) -> ()
	fn switch_colour() -> ()
};
struct button {
	height: i16;
	width: i16;
	hovered: bool;
	held: bool;
	func: fn() -> ();
};
impl button_t {
	fn resize()
	fn onHover() -> ()
	fn onClick() -> ()
	fn onRelease() -> ()
};