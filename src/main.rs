use std::time::Instant;

pub fn time_it(name: &str, f: impl FnOnce()) {
    let start = Instant::now();
    f();
    println!("{:>20}: {:?}", name, start.elapsed());
}

fn main() {
    let iters = 100_000_000;
    // Build a byte array of valid hex characters. This way the results are deterministic
    // and not optimized out by the compiler.
    let mut bytes: Vec<(u8, u8)> = Vec::with_capacity(iters);

    for i in 0..iters {
        bytes.push(
            (
                b'0' + (i % 10) as u8,
                b'a' + (i % 6) as u8
            )
        );
    }

    // Just make sure that they are the same
    assert_eq!(
        u8::from_str_radix(std::str::from_utf8(&[97u8, 98u8]).unwrap(), 16).unwrap(),
        (
            (char::from(97u8).to_digit(16).unwrap() << 4) +
             char::from(98u8).to_digit(16).unwrap()
        ) as u8
    );

    time_it("from str", || {
        for (b1, b2) in bytes.iter() {
            u8::from_str_radix(std::str::from_utf8(&[*b1, *b2]).unwrap(), 16).unwrap();
        }
    });

    time_it("from str unchecked", || {
        for (b1, b2) in bytes.iter() {
            unsafe {
                u8::from_str_radix(std::str::from_utf8_unchecked(&[*b1, *b2]), 16).unwrap();
            }
        }
    });

    time_it("from char", || {
        for (b1, b2) in bytes.iter() {
            (
                (char::from(*b1).to_digit(16).unwrap() << 4) +
                 char::from(*b2).to_digit(16).unwrap()
            ) as u8;
        }
    });

    time_it("from_utf8", || {
        for (b1, b2) in bytes.iter() {
            std::str::from_utf8(&[*b1, *b2]).unwrap();
        }
    });

    time_it("from_utf8_unchecked", || {
        for (b1, b2) in bytes.iter() {
            unsafe { std::str::from_utf8_unchecked(&[*b1, *b2]); }
        }
    });
}
