## Synopsis

Year 2016 solutions are all different projects per date. Days 1 and 2 are solved using Python, day 3 with C++ and all the rest using Rust. You need to build each day separately and each day has a different way of interacting with user. Some code reading might be needed.

## Installation

You should use Cargo for building each Rust project.

Build:

	cargo build --release

Or just run

	cargo run --release

Day 3 was done using Code::Blocks IDE but it can be compiled on command line with g++:

	g++ --std=c++11 main.cpp -o d3

(Solution for day 3 now only includes second phase)
