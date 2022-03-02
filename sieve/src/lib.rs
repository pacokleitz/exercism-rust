pub fn primes_up_to(upper_bound: u64) -> Vec<u64> {
    let mut is_prime = vec![true; (upper_bound + 1) as usize];
    let mut multiplier;

    for n in 2..=upper_bound {
        if is_prime[n as usize] {
            multiplier = 2;
            while n * multiplier <= upper_bound {
                is_prime[(n * multiplier) as usize] = false;
                multiplier += 1;
            }
        }
    }

    is_prime
        .iter()
        .enumerate()
        .filter_map(|(n, is_prime)| if *is_prime { Some(n as u64) } else { None })
        .skip(2)
        .collect::<Vec<u64>>()
}
