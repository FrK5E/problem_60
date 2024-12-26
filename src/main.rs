fn init_primes(data: &mut Vec<bool>) {
    for i in 2..data.len() {
        if !data[i] {
            continue;
        }
        for j in 2..data.len() {
            if i * j >= data.len()-1 {
                break;
            }
            data[i * j] = false;
        }
    }
}

fn concat( a: u32, b: u32 ) -> u32 { 
    let mut n=1; 
    loop { 
        if n>b { 
            break;
        }
        n = n*10;
    }
    a*n+b
}

fn main() {
    println!("Hello, world!");

    let mut data = Vec::with_capacity(10000);
    data.resize(10000, true);

    init_primes(&mut data);

    let mut count = 0;
    let mut it = 0;
    while count < 5 {
        if data[it] {
            println!("{it}");
            count += 1;
        }
        it += 1;
    }
}

// fn explore(start: u32, max: u32) {
//     for i in start..max {
//         for j in i..max {
//             for k in j..max {
//                 for l in k..max {
//                     for m in l..max {
//                         test(i, j, k, l, m);
//                     }
//                 }
//             }
//         }
//     }
// }

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_concat() {
        assert_eq!(concat(1, 2),12);
        assert_eq!(concat(5,3), 53); 
        assert_eq!(concat(3,5), 35);
    }
}