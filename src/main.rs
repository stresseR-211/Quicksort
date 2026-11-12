fn partition(a: &mut Vec<i32>, lo: usize, hi: usize) -> usize {
	let pivot = a[hi];
	let mut i = lo;
	for j in lo..hi {
		if a[j] <= pivot { 
			a.swap(i, j);
			i = i + 1;
		}
	}
	a.swap(i, hi);
	return i;
}
fn _qsort(a: &mut Vec<i32>, lo: usize, hi: usize) {
	if hi <= lo {
		return;
	}
	let pivot = partition(a, lo, hi);
	_qsort(a, lo, pivot - 1);
	_qsort(a, pivot + 1, hi);
}
fn quicksort(array: &mut Vec<i32>) -> () {
	_qsort(array, 0, array.len()-1);
}
fn main() -> Result<(), i32> {
	let mut array: Vec<i32> = [7,3,0,9,8,1,4,5,2,6].to_vec(); 
	println!("Array before sorting: {:?}", array);
	quicksort(&mut array);
	println!("Array after sorting: {:?}", array);
	Ok(())
}