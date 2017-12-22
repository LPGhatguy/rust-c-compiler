#!/bin/sh
cargo run > out.s
gcc -m32 out.s -o out
./out

echo "Return code: $?"

rm out.s
rm out