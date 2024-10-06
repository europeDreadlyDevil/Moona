# Moona 🌙
**Moona** is build-util for easy configure run for single and multi service app

## Example

```toml
[vars]
path1 = "some/path"
path2 = "example/path"
path3 = "test/path"

[rules]
build1 = "cargo build --manifest-path {path1}"
service1 = "cargo run --manifest-path {path1}"
build2 = "cargo build --manifest-path {path1}"
service2 = "cargo run --manifest-path {paht2}"
service3 = "cargo run --manifest-path {paht3}"

[run]
build_thread1 = ['build1']
build_thread2 = ['build2']
thread1 = ["servicee1", "service2"]
thread2 = ["service3"]

[order]
order = [['build_thread1', 'build_thread2'], ['thread1', 'thread2']]
```

## Installation
1. From Cargo: ```cargo install moona```
2. [Releases](https://github.com/europeDreadlyDevil/Moona/releases/tag/moona-v0.1.0)

## Use
1. ```moona```
2. ```moona <PATH>```
3. ```moona -r <RULE>```

## License
* [LICENSE-APACHE](LICENSE-APACHE)
* [LICENSE-MIT](LICENSE-MIT)
