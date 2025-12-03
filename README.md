# zhalt

A Simple CLI wrapper that provides a unified interface for various Input Methods across different environments.

This program is heavily influenced by [zenhan](https://github.com/iuchim/zenhan). Large part of the Windows-specific implementation is just a Rust re-implementation of the aforementioned project.


## Notice

This is just a hobby program, and furthermore, the interfaces will be subjected to breaking changes in the future.


## Support

This program is currently supporting the following targets:

- Windows
- Linux (fcitx5)


## Usage

To get the current state of the IME:

```
<executable>
```

To turn on the IME:

```
<executable> 1
```

To turn off the IME:

```
<executable> 0
```

To toggle the current state of the IME:

```
<executable> toggle
```


## Build

### Windows

```
cargo install --path <the_directory_where_Cargo.toml_exists>
```

### Linux

```
cargo install --path <the_directory_where_Cargo.toml_exists> --features <your_input_method_framework>
```

See also "Support" section to find out which Input Method Frameworks are supported.
