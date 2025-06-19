use std::time::Instant;

use clap::Parser;

#[derive(Parser)]
#[command(author, version, about)]
struct Args {
    limit: usize,
}

fn main() {
    let start_time: Instant = Instant::now();

    let args: Args = Args::parse();

    if args.limit < 2 {
        eprintln!("Please provide a limit greater than or equal to 2.");
        std::process::exit(1);
    }

    let limit: usize = args.limit;
    let mut is_prime: Vec<bool> = vec![true; limit + 1];
    is_prime[0] = false;
    is_prime[1] = false;

    for i in 2..=limit {
        if i * i > limit {
            break;
        }
        if is_prime[i] {
            let mut multiple = i * i;
            while multiple <= limit {
                is_prime[multiple] = false;
                multiple += i;
            }
        }
    }

    let prime_count: usize = is_prime.iter().filter(|&&x| x).count();
    println!("Number of primes up to {}: {}", limit, prime_count);

    let end_time: Instant = Instant::now();

    println!("Elapsed time: {:?}", end_time - start_time);
}
