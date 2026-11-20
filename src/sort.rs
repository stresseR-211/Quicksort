use rand::seq::SliceRandom;
use std::collections::VecDeque;
pub type StepsDeque = VecDeque<(usize, usize)>;
fn partition(a: &mut Vec<i32>, lo: usize, hi: usize, steps: &mut StepsDeque) -> usize {
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
fn qsort_steps(a: &mut Vec<i32>, lo: usize, hi: usize, steps: &mut StepsDeque) {
	if lo >= hi {
		return;
	}
	let p = partition(a, lo, hi, steps);
	if p > lo {
		qsort_steps(a, lo, p - 1, steps);
	}
	qsort_steps(a, p + 1, hi, steps);
}

pub fn quicksort(array: &Vec<i32>) -> StepsDeque {
	let mut steps = StepsDeque::new();
	if array.len() > 1 {
		let mut work = array.clone();
		let hi = work.len() - 1;
		qsort_steps(&mut work, 0, hi, &mut steps);
	}
	steps
}
pub fn shuffle(array: &Vec<i32>) -> StepsDeque {
	let mut steps = StepsDeque::new();
	let len = array.len();
	for i in (1..len).rev() {
		let j = rand::random_range(0..=i);
		if i != j {
			steps.push_back((i, j));
		}
	}
	steps
}