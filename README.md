# Benchmark str radix vs char radix


### debug
```
$ cargo run
   Compiling hex-escape-bench v0.1.0 (file:///Users/kchoubacha/repos/kbacha/hex-escape-bench)
    Finished dev [unoptimized + debuginfo] target(s) in 0.65 secs
     Running `target/debug/hex-escape-bench`
            from str: Duration { secs: 5, nanos: 181579738 }
  from str unchecked: Duration { secs: 4, nanos: 503736649 }
           from char: Duration { secs: 7, nanos: 802062246 }
           from_utf8: Duration { secs: 3, nanos: 800076037 }
 from_utf8_unchecked: Duration { secs: 3, nanos: 129759622 }
```

### release
```
$ cargo run --release
   Compiling hex-escape-bench v0.1.0 (file:///Users/kchoubacha/repos/kbacha/hex-escape-bench)
    Finished release [optimized] target(s) in 0.41 secs
     Running `target/release/hex-escape-bench`
            from str: Duration { secs: 1, nanos: 185271576 }
  from str unchecked: Duration { secs: 0, nanos: 545586102 }
           from char: Duration { secs: 0, nanos: 107972942 }
           from_utf8: Duration { secs: 0, nanos: 686227808 }
 from_utf8_unchecked: Duration { secs: 0, nanos: 28 }
```
