# Moona 🌙
**Moona** is build-util for easy configure run for single and multi service app

## Example

```toml
[vars]
path1 = "some/path"
path2 = "example/path"
path3 = "test/path"

[rules]
service1 = "cargo run --manifest-path {path1}"
service2 = "cargo run --manifest-path {paht2}"
service3 = "cargo run --manifest-path {paht3}"

[run]
thread1 = ["servicee1", "service2"]
thread2 = ["service3"]
```

## Installation
1. From Cargo: ```cargo install moona```
2. [Releases]()

## License
* [LICENSE-APACHE](LICENSE-APACHE)
* [LICENSE-MIT](LICENSE-MIT)