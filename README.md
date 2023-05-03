# rust-lib-closure-test

Simple example to demonstrate that rustc optimizes an implementation of a generic function with a closure-array as argument inside a lib way better than when its called from outside the lib.

You can find the disassembled assembly of both functions in [01_lib_call.s](./01_lib_call.s) and [01_main_call.s](./01_main_call.s).

## How I got the assembly

```bash
cargo b --release
```

```bash
objdump -D target/release/rust-lib-closure-test > disassembly.s
```

In ```disassembly.s``` I searched for the function ```deserialize_struct```. There are two functions with this name. One is from the lib and the other one is from main.rs. The function called from inside the lib is the one called in ```deserialize_color_generic```. Comparing these two functions shows that the one from the lib is way better optimized.
