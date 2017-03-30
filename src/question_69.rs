fn is_prime(int: usize) -> bool {
	let max = (int as f64).sqrt() as usize + 1;

	for i in 2..max {
		if int % i == 0 { return false; }
	}
	true
}


fn primes_below_N(N: usize) -> Vec<f64> {
	let mut primes = vec![2 as f64];
	for i in 1..(N/2 + 1) {
		let n = 2*i + 1;
		if is_prime(n) { primes.push(n as f64); }
	}

	primes
}


fn totient(primes: &Vec<f64>, n: f64) -> f64 {
	let mut totient = n as f64;

	for &prime in primes {
		if prime > n { break; }
		if n % prime == 0 as f64 { 
			totient = totient * (1.0 - 1.0/prime); 
		}
	}

	totient
}


fn main() {
	let N = 1000000;
	let primes = primes_below_N(N);

	let mut cur_max_ratio = 0 as f64;
	let mut cur_max_n = 0;
	for n in 1..N {
		if n % 10000 == 0 { println!("{:?}", n); }
		let n_f64 = n as f64;
		let totient = totient(&primes, n_f64);

		let ratio = n_f64 / totient;
		if ratio > cur_max_ratio {
			cur_max_ratio = ratio;
			cur_max_n = n;
		}
	}

    println!("{:?}", cur_max_n);
}
