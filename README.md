# CREB: a command line application for searching patterns in file build with rust 🦀, inspired by the original <a href="https://www.gnu.org/software/grep/">grep</a> tool.

![Static Badge](https://img.shields.io/badge/cargo-1.74.1%20-blue)
![Static Badge](https://img.shields.io/badge/LICENSE-MIT-green)

## Usage

### First clone and cd to the project folder

```bash
    git clone  https://github.com/carlosdanielmaturano/creb.git
    cd creb
```

### And then run:

```bash
    cargo run <PATTERN> <FILE-PATH> <OPTIONS-FLAGS>
```

### For building an standalone binary, run:

```bash
    cargo build --release
    ./target/release/creb
```

### OPTIONS-FLAGS

<p>the options flags are used to give the user a bit more control of what the program should, do and that options included</p>

| command         | description                                     |
| --------------- | ----------------------------------------------- |
| -v or --invert  | invert the match output                         |
| -c or --color   | colorize the matching results                   |
| -n or --numbers | display the line number of the matching content |
