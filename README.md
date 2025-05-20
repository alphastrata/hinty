# hinty

See the [blog post](https://jeremyfwebb.ninja/2025/2025-MAY-20) for more info.

I was trying to answer the question, _does `std::hint::likely()` or `unlikely()` **really** do anything?_

```sh
cargo build --release
./gen_data.sh 10000000 # 10 million
cargo bench
./run_perf.sh
```

 
