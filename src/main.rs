use std::env;
use std::time::Instant;

const START_NUMBER: i64 = 1;
fn main() {

    let args: Vec<String> = env::args().collect();
    let mut current: i64 = if args.len() > 1 {
        args[1].parse::<i64>().unwrap_or(START_NUMBER)
    } else {
        START_NUMBER
    };
    println!("Starting Syracuse from {} based on args {:?}", current, args);
    let mut biggest: (i64, i64) = (0, 0);
    let mut now = Instant::now();
    let mut computation = 0;
    loop {
        let steps = compute_steps(current);
        if steps > biggest.1 {
            biggest = (steps, current);
            println!("Steps {} from {}", biggest.0, biggest.1);
        }
        computation += 1;
        if computation >= 100_000_000 {
            println!("100M computations in {:?}. Now at {}", now.elapsed(), current);
            now = Instant::now();
            computation = 0;
        }
        current += 1;
    }
}

fn compute_steps(mut current: i64) -> i64 {
    let mut steps = 2;                                      // init at 2 as we stop at 4
    while current != 4 {
        if current & 1 == 0 {                               // % 2 adds about 50% overall
            current = current >> 1;                         // / 2 adds about 100-120% overall
        } else {
            current = (current * 3) + 1;
            // current = (current << 1) + current + 1;      // its a tadd slower
            // current = current + current + current + 1;   // about 20% slower
        }
        steps += 1;
    }
    steps
}
