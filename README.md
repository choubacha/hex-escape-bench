# Benchmark str radix vs char radix


### debug
```
$ cargo run
   Compiling hex-escape-bench v0.1.0 (file:///Users/kchoubacha/repos/kbacha/hex-escape-bench)
    Finished dev [unoptimized + debuginfo] target(s) in 0.47 secs
     Running `target/debug/hex-escape-bench`
            from str: Duration { secs: 5, nanos: 108206998 }
           from char: Duration { secs: 7, nanos: 827446431 }
       str formation: Duration { secs: 3, nanos: 791812058 }
```

### release
```
$ cargo run --release
   Compiling hex-escape-bench v0.1.0 (file:///Users/kchoubacha/repos/kbacha/hex-escape-bench)
    Finished release [optimized] target(s) in 0.37 secs
     Running `target/release/hex-escape-bench`
            from str: Duration { secs: 1, nanos: 201551561 }
           from char: Duration { secs: 0, nanos: 105942603 }
       str formation: Duration { secs: 0, nanos: 684496518 }
```
