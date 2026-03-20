use std::io;

fn counting_sort <T:Copy + Default, F: Fn(&T) -> usize>(
		items: Vec<T>, 
		key: F, 
		max_value: usize
		) -> Vec<T>  {
		let mut count = vec![0; max_value + 1];
		
		for i in 0 .. items.len() {
			let cur_num: usize = key(&items[i]) as usize;
			
			count[cur_num] += 1;
		}
		let mut positions =  vec![0; max_value + 1];
		for i in 1 ..=max_value {
			positions[i] = positions[i - 1] + count[i - 1];
		}
		
		let mut output = vec![T::default(); items.len()];
		for i in 0..items.len() {
			let v = key(&items[i]) as usize;
			output[positions[v]] = items[i];
			positions[v] += 1;
		}
		
		return output;
		
}
	
	
fn main() {
    let mut s: String = String::new();
    io::stdin().read_line(&mut s);
    let n: usize = s.trim().parse().unwrap();
	let mut s: String = String::new();
    io::stdin().read_line(&mut s);
	s = s.trim().to_string();
    //println!("{}", n);
	
	
//	let n = 6;
//	let s = "banana".to_string();
	let mut ranks: Vec<usize> = vec![0; n];
	let bytes = s.as_bytes();
	// forming initial ranks
	for i in 0..n {
		ranks[i] = (bytes[i] - b'a') as usize;
	}
	
	let mut pairs: Vec<(i32, i32, usize)> = vec![(0,0, 0); n];
	
	let mut k: usize = 1;
	//forming initial pairs of ranks
	
	let mut first_pass = true;
	while k < n {
		let mut new_ranks = vec![0usize; n];
		if *ranks.iter().max().unwrap() == n - 1 { break; }
		for i in 0..n {
			pairs[i].0 = ranks[i] as i32;
			if i + k < n {
				pairs[i].1 = ranks[i + k] as i32;
			} else {
				pairs[i].1 = 0;
			}
			pairs[i].2 = i;
		
		}
		let max_value = if first_pass {25} else {n};
		first_pass = false;
		pairs = counting_sort(pairs, |p| p.1 as usize, max_value);
		pairs = counting_sort(pairs, |p| p.0 as usize, max_value);
		k *= 2;
		
		
		new_ranks[pairs[0].2] = 0;
		for i in 1..n {
			let prev_first = pairs[i - 1].0;
			let prev_second = pairs[i - 1].1;
			let curr_first = pairs[i].0;
			let curr_second = pairs[i].1;
			
			if prev_first == curr_first && prev_second == curr_second {
				new_ranks[pairs[i].2] = new_ranks[pairs[i - 1].2];
			} else {
				new_ranks[pairs[i].2] = new_ranks[pairs[i - 1].2] + 1;
			}
		}
		ranks = new_ranks;
	}
	
	let mut suffix_array = vec![0; n];
	
	for i in 0..n {
		suffix_array[ranks[i]] = i;
	}
	
	let mut l = 0;
	// constracting lcp array
	let mut lcp_array = vec![0; n];
	for i in 0..n {
		// i is index of suffix 
		let sa_index = ranks[i];
		if sa_index == 0 {
			continue;
		}
		
		let neighbor = suffix_array[sa_index - 1];
		
		while i + l < n && neighbor + l < n && bytes[i + l] == bytes[neighbor + l] {
			l += 1;
		}
		lcp_array[i] = l;
		
		if l > 0 {
			l -= 1;
		}
	}
		
			
		
	
//	println!("{:?}", ranks);
//	println!("{:?}", suffix_array);
//	println!("{:?}", lcp_array);
	let answer = lcp_array.iter().max().unwrap();
	println!("{:?}", answer);
	
	
		
}