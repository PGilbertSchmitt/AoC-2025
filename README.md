# Advent of Code

Simply put all inputs as text files in `src/<YEAR>/inputs/` and run `cargo test --release`. Some important notes:

- 2025 Day 6 needs to be pasted **_exactly_** as is without the whitespace being touched. The whitespace at the end of each line in that input is important

Some assertions or tests have been commented out. Usually, it's because the calculations are slow enough that it's not pleasant to test over and over, even in release mode. However, sometimes the answer is something that can't be checked easily, such as those solutions where something is drawn on the console.

- 2015 Day 4 Part 2 - takes ~1.3 seconds in release mode
- 2016 Day 8 Part 2 - spells text in the console
- 2016 Day 14 Part 2 - takes over a minute in release mode
