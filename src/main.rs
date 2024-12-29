fn init_primes(data: &mut Vec<bool>) {
    for i in 2..data.len() {
        if !data[i] {
            continue;
        }
        for j in 2..data.len() {
            if i * j >= data.len() - 1 {
                break;
            }
            data[i * j] = false;
        }
    }
}

fn concat(a: usize, b: usize) -> usize {
    let mut n = 1;
    loop {
        if n > b {
            break;
        }
        n = n * 10;
    }
    a * n + b
}

fn test(trial: &Vec<usize>, primes: &Vec<bool>) -> bool {
    for i in 1..trial.len() {
        for j in 1..trial.len() {
            if i == j {
                continue;
            }
            let k = concat(trial[i], trial[j]);
            if ! primes[k] { 
                return false;
            }
        }
    }

    true
}

fn explore(start: usize, max: usize, primes: &Vec<bool>) {
    for i in start..max {
        if !primes[i] {
            continue;
        }
        for j in i..max {
            if !primes[j] {
                continue;
            }
            for k in j..max {
                if !primes[k] {
                    continue;
                }
                for l in k..max {
                    if !primes[l] {
                        continue;
                    }
                    for m in l..max {
                        if !primes[m] {
                            continue;
                        }
                        let trial = vec![i, j, k, l, m];
                        test(&trial, &primes);
                    }
                }
            }
        }
    }
}

fn main() {
    println!("Hello, world!");

    let mut data = Vec::with_capacity(10000);
    data.resize(10000, true);

    init_primes(&mut data);

    explore(1, 1000, &data);
}

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2), 12);
        assert_eq!(concat(5, 3), 53);
        assert_eq!(concat(3, 5), 35);
    }

    #[test]
    fn test_primes() { 
        let mut data = Vec::with_capacity(800);
        data.resize(800, true);
        init_primes(&mut data);

        assert_eq!( data[3], true);
        assert_eq!( data[7], true);
        assert_eq!( data[109], true); 
        assert_eq!( data[673], true);
        assert_eq!( data[674], false);
    }

    #[test]
    fn test_criterion(){
        let n: usize  = 673109 + 2; 
        let mut data = Vec::with_capacity(n);
        data.resize(n, true);
        init_primes(&mut data);
        let trial = vec![3,7,109, 673];
        assert_eq!( test(&trial, &data), true);

        let trial2 = vec![3,7,11,673];
        assert_eq!( test(&trial2, &data), false );

    }
}
