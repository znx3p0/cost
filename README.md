# Cost

Measure the cost of running a function.

This library exposes a function which takes a callback and returns the cost of
running that function.

```rust
let (cost, num) = cost(|| fibonacci(20));
println!("it cost {} instructions to compute {}", cost, num);
```
