# zhalt

A simple rust program that provides abstracted IME control layer across several operating systems.

This program is heavily influenced by [this project](https://github.com/iuchim/zenhan). The majority of the Windows-specific process is a re-implementation of the aforementioned project in Rust.


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

For Windows users:

```
cargo install --path <project_root_dir>
```

...where the `<project_root_dir>` is the same directory as Cargo.toml is.

For Linux users:

```
cargo install --path <project_root_dir> --features <your_input_method_framework>
```

We are currently supporting the following IMFs (Input Method Frameworks):

- fcitx5
