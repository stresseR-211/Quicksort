use rand::seq::SliceRandom;
use std::collections::VecDeque;
pub type steps_deque = VecDeque<(usize, usize)>;
fn partition(a: &mut Vec<i32>, lo: usize, hi: usize, steps: &mut steps_deque) -> usize {
	let pivot = a[hi];
	let mut i = lo;
	for j in lo..hi {
		if a[j] <= pivot {
			if i != j {
				a.swap(i, j);
				steps.push_back((i, j));
			}
			i += 1;
		}
	}
	if i != hi {
		a.swap(i, hi);
		steps.push_back((i, hi));
	}
	i
}
fn _qsort(a: &mut Vec<i32>, lo: usize, hi: usize, steps: &mut steps_deque) -> () {
	if lo >= hi {
		return;
	}
	let p = partition(a, lo, hi, steps);
	if p > lo {
		_qsort(a, lo, p - 1, steps);
	}
	_qsort(a, p + 1, hi, steps);
}
pub fn quicksort(array: &mut Vec<i32>) -> steps_deque {
	let mut steps = steps_deque::new();
	if array.len() > 1 {
		let hi = array.len() - 1;
		_qsort(array, 0, hi, &mut steps);
	}
	println!("quicksorted: {:?}", array);
	steps
}
pub fn quicksort_void(array: &mut Vec<i32>) -> () {
	quicksort(array);
}
pub fn shuffle(array: &mut Vec<i32>) -> () {
	array.shuffle(&mut rand::thread_rng());
	println!("shuffled: {:?}", array);
}