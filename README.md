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
service2 = "cargo run --manifest-path {path2}"
service3 = "cargo run --manifest-path {path3}"

[run]
thread1 = ["service1", "service2"]
thread2 = ["service3"]
```

## Installation
1. From Cargo: ```cargo install moona```
2. [Releases](https://github.com/europeDreadlyDevil/Moona/releases/tag/moona-v0.1.0)

## Use
1. ```moona```
2. ```moona <PATH>```

## License
* [LICENSE-APACHE](LICENSE-APACHE)
* [LICENSE-MIT](LICENSE-MIT)
