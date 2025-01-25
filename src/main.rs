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

fn test_trial_vector(trial: &[usize], primes: &Vec<bool>) -> bool {
    for i in trial {
        for j in trial {
            if i == j {
                continue;
            }
            let k = concat(*i, *j);
            if !primes[k] {
                return false;
            }
        }
    }

    true
}

pub fn clone_with_extra<T: Clone>(x: &[T], y: &T) -> Vec<T> {
    x.iter().chain([y]).cloned().collect()
}

fn explore(trial: &[usize], primes: &Vec<bool>) {
    if trial.len()==5 { 
        return;
    }

    let start = trial[trial.len() - 1] + 1;

    for i in start..2000 {
        if !primes[i] {
            continue;
        }
        let mut trial2: [usize; 5] = [0, 0, 0, 0, 0];
        for i in 0..trial.len() {
            trial2[i] = trial[i];
        }
        trial2[trial.len()] = i;
        let n = trial.len() + 1;

        if concat(trial2[n - 1], trial2[n - 2]) >= primes.len() {
            break;
        }
        let flag = test_trial_vector(&trial2[0..n], &primes);
        if flag {
            println!("found ");
            for i in trial2 {
                print!(" {} ", i)
            }
            explore(&trial2, primes);
        }
    }
}

fn main() {
    let n = 100000000;
    let mut primes = Vec::with_capacity(n);
    primes.resize(n, true);

    init_primes(&mut primes);

    for i in 3..1000 {
        if !primes[i] { 
            continue;
        }
        let trial: [usize; 2] = [i, i + 1];
        explore(&trial[0..2], &primes);
    }
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

        assert_eq!(data[3], true);
        assert_eq!(data[7], true);
        assert_eq!(data[109], true);
        assert_eq!(data[673], true);
        assert_eq!(data[674], false);
    }

    #[test]
    fn test_criterion() {
        let n: usize = 673109 + 2;
        let mut data = Vec::with_capacity(n);
        data.resize(n, true);
        init_primes(&mut data);
        let trial = vec![3, 7, 109, 673];
        assert_eq!(test_trial_vector(&trial, &data), true);

        let trial2 = vec![3, 7, 11, 673];
        assert_eq!(test_trial_vector(&trial2, &data), false);
    }

    #[test]
    fn test_criterion2() {
        let n: usize = 673109 + 2;
        let mut data = Vec::with_capacity(n);
        data.resize(n, true);
        init_primes(&mut data);
        let trial = vec![3, 7];
        assert_eq!(test_trial_vector(&trial, &data), true);
    }
}
