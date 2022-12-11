use bit_vec::BitVec;

type Num = u64;

pub fn prime_sieve(n: Num) -> Vec<Num> {
    assert!(n > 1);

    let mut a = BitVec::from_elem(n as usize, true);

    for i in 2..(1 + (n as f64).sqrt() as Num) {
        if a[i as usize] {
            let mut j: Num = i.pow(2) as Num;
            while j < n {
                a.set(j as usize, false);
                j += i;
            }
        }
    }

    let mut actual: Vec<Num> = vec![];
    for i in 2..a.len() {
        if a[i] {
            actual.push(i as Num);
        }
    }

    actual
}

#[allow(dead_code)]
fn prime_factors_naive(a: Num) -> Vec<Num> {
    let mut factors: Vec<Num> = vec![];
    let primes = calc_primes_to(a);

    let mut prime_iter = primes.iter();
    let mut running = a;
    let mut prime = prime_iter.next();

    while prime.is_some() {
        let prime_v = prime.unwrap();
        if running % prime_v == 0 {
            factors.push(*prime_v);
            running /= prime_v;
        } else {
            prime = prime_iter.next();
        }
    }

    factors
}

// pub fn find_prime_factors(n: Num) -> Vec<Num> {
//     if n <= 1 {
//         // Technically not correct as 1 is not prime or composite.
//         // This makes things easier to program though.
//         return vec![n];
//     } else if is_prime(n) {
//         return vec![n];
//     } else {
//         return prime_factors_sieve(n);
//     }
// }

#[allow(dead_code)]
fn prime_factors_sieve(n: Num) -> Vec<Num> {
    assert!(n > 1);

    let mut a = BitVec::from_elem(n as usize, true);

    let mut factors: Vec<Num> = vec![];
    let mut running = n;

    for i in 2..(1 + (n as f64).sqrt() as Num) {
        if running == 0 {
            break;
        }

        if a[i as usize] {
            let mut j: Num = i.pow(2) as Num;
            while j < n {
                a.set(j as usize, false);
                j += i;
            }

            while running % i == 0 {
                //println!("{}", running);
                factors.push(i);
                running /= i;
            }
        }
    }

    factors
}

#[allow(dead_code)]
pub fn calc_primes_to(a: Num) -> Vec<Num> {
    let mut primes: Vec<Num> = vec![2, 3];

    for x in 4..a {
        if is_prime(x) {
            primes.push(x);
        }
    }

    primes
}

#[allow(dead_code)]
pub fn is_prime(a: Num) -> bool {
    if a == 1 {
        return false;
    }
    if a == 2 || a == 3 {
        return true;
    }

    for x in (2..a).rev() {
        if a % x == 0 {
            return false;
        }
    }

    true
}
