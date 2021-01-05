use primes::is_prime;
use num::ToPrimitive;
use std::io;

fn main() {
    let mut strof = String::new();
    io::stdin().read_line(&mut strof).expect("Oh.");
    let numof: usize = strof.trim().parse().expect("Oh.");

    let test = findprimefactors(numof);
    println!("{:?}", test);
}

fn findprimefactors(n: usize) -> Vec<usize> {
    let mut vecofdividers: Vec<usize> = Vec::new();
    let halfof: f32 = (n as f32).sqrt();
    let uhalfof: u64 = num::ToPrimitive::to_u64(&halfof.floor()).unwrap();  
    for divider in 1..uhalfof {
        if n % divider as usize == 0 {
            vecofdividers.push(divider as usize);
        } else {
            continue;
        }
    }

    let mut vecofprimedividers: Vec<usize> = Vec::new();
    for (i, &item) in vecofdividers.iter().enumerate() {
        if is_prime(item as u64) == true {
            vecofprimedividers.push(item);
        } else {
            continue;
        }
    }

    return vecofprimedividers;
}