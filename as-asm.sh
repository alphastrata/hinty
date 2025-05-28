#!/usr/bin/env bash

# Output all the is_fib versions' assembly to their own named file for comparison.

cargo asm --bin likely likely::is_fib > likely.s
cargo asm --bin unlikely unlikely::is_fib > unlikely.s
cargo asm --bin baseline baseline::is_fib > baseline.s
