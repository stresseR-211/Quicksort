mod sort;
fn main() -> Result<(), i32> {
	let mut array: Vec<i32> = [7,3,0,9,8,1,4,5,2,6].to_vec(); 
	println!("Array before sorting: {:?}", array);
	sort::quicksort(&mut array);
	println!("Array after sorting: {:?}", array);
	Ok(())
}