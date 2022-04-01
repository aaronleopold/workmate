# workmate

A silly CLI tool to move your mouse around in simple, pre-defined patterns.

## Usage

Here are some example commands:

```bash
# show the help menu
cargo run --release -- --help

# run with all default settings
cargo run --release

# specify a pattern and interval (in seconds)
cargo run --release -- --pattern "back-and-forth" --interval 5

# left mouse click
cargo run --release -- --interval 5 --click
```

## Future work

This was just a quick and fun little project for me and I don't plan on doing anything else with it. Might look into making more patterns, like finding windows and focusing them on interval or something but who knows.
