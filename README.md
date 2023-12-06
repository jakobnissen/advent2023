### Install
* Clone this repo
* Using Rust's `cargo`, run `cargo build --release`
* The binary can be found in `target/release/advent2023`

### Solving days
* Make sure you've downloaded the data first (see below)
* To solve days `x`, `y` and `z`, run: `advent2023 solve data 1 2 3`
* Alternatively, to run all implemented days, run `advent2023 solve data --all`

Example:
```shell
$ advent2023 solve data 1 2 3
Day 01 [102.88µs]:
  Part 1: 57346
  Part 2: 57345

Day 02 [56.67µs]:
  Part 1: 2716
  Part 2: 72227

Day 03 [278.12µs]:
  Part 1: 517021
  Part 2: 81296995
  
```

### Downloading data
* Login on [Advent of Code's website](https://adventofcode.com/2023)
* Obtain a session code identifying you to the AoC server. To do this, using Firefox:
	- In your browser, right click the page and press "inspect"
	- In the "Network" tab, press the "Reload" button
	- Click the HTML document
	- Under "Headers", in "Request headers", find your cookie.
	- Part of the cookie has the string `session=[long hexadecimal code];`. The hexadecimal part of this is your code.
* Set an environmental variable called `ADVENTOFCODE_SESSION` to your session code
* To download data of days `x`, `y`, and `z` to directory `data`, run: `ADVENTOFCODE_SESSION=abcdef[...]3d2f advent2023 download data x y z`

Example:
```shell
$ ADVENTOFCODE_SESSION=9f5d642957086d6ab635fe1a1ccfdc2db09379dfcb9d8d0f07553fcc0528d9aae1355b1a84d384119823136e7aa411fc1412e950048a97efeca7d948d291c65d advent2023 download data 1 2 3
Downloading day 01
Downloading day 02
Downloading day 03
```

To download all released days, you can run `ADVENTOFCODE_SESSION=[...] advent2023 download data --all`
