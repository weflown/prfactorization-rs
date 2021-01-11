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

    let mut duplicate: u64 = 0;
    for (i, &item) in vecofprimedividers.iter().enumerate() {
        duplicate = duplicate * item;
    };

    if duplicate != x as u64 {
        let mut newvecofpd = vecofprimedividers.clone();
        let mut sumvec: u64 = 0;
        loop {
            for (i, &item) in vecofprimedividers.iter().enumerate() {
                sumvec = sumvec * item;
            };

            if sumvec == x as u64 {
                break;
            } else if sumvec != x as u64 {
                let vectoparse = sieveoferatosphene(x as u64);
                for (i, &item) in vectoparse.iter().enumerate() {
                    if x as u64 % (sumvec * item) == 0 && (sumvec * item) != x as u64 {
                        newvecofpd.push(item);
                        break;
                    } else if x as u64 % (sumvec * item) != 0 {
                        continue;
                    } else {
                        break;
                    }
                }
            };
        }

        return newvecofpd;
    } else {
        return vecofprimedividers;
    }
}
