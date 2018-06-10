use std::time::Instant;

/// Executes a closure once and returns how long that closure took
/// as a duration.
pub fn time_it(name: &str, f: impl FnOnce()) {
    let start = Instant::now();
    f();
    println!("{:>20}: {:?}", name, start.elapsed());
}

fn main() {
    let iters = 100_000_000;
    assert_eq!(
        u8::from_str_radix(std::str::from_utf8(&[97u8, 98u8]).unwrap(), 16).unwrap(),
        (
            (char::from(97u8).to_digit(16).unwrap() << 4) +
             char::from(98u8).to_digit(16).unwrap()
        ) as u8
    );

    time_it("from str", || {
        for _ in 0..iters {
            u8::from_str_radix(std::str::from_utf8(&[97u8, 98u8]).unwrap(), 16).unwrap();
        }
    });

    time_it("from char", || {
        for _ in 0..iters {
            (
                (char::from(97u8).to_digit(16).unwrap() << 4) +
                 char::from(98u8).to_digit(16).unwrap()
            ) as u8;
        }
    });

    time_it("no str formation", || {
        for _ in 0..iters {
            u8::from_str_radix(&"ab", 16).unwrap();
        }
    });

    time_it("str formation", || {
        for _ in 0..iters {
            std::str::from_utf8(&[97u8, 98u8]).unwrap();
        }
    });
}
