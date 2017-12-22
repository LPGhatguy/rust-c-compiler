#!/bin/sh
cargo run > out.s
gcc -m32 out.s -o out
./out

rm out.s
rm out