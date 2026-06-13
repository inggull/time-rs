# time-rs

간단한 시간 관련 Rust 라이브러리입니다.

## Example

```rust
fn main() {
    // UTC
    println!("1970-01-01 00:00:00 UTC was {:?} ago!", time::now());

    // UTC+9
    println!("Current system time: {}", time::Date::now(9));
}
```
