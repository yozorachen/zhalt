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
