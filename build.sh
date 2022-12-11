#!/usr/bin/env bash

# build
 cargo build --release --target x86_64-pc-windows-gnu
 cargo build --release --target x86_64-unknown-linux-gnu

# zip
rm bin.zip
zip -r bin bin

# move
 cp ./target/x86_64-pc-windows-gnu/release/pttypter.exe bin/
 cp ./target/x86_64-unknown-linux-gnu/release/pttypter bin/

# clean
rm ./docs/**/*.aux &>/dev/null
rm ./docs/**/*.fdb_latexmk &>/dev/null
rm ./docs/**/*.fls &>/dev/null
rm ./docs/**/*.log &>/dev/null
rm ./docs/**/*.out &>/dev/null
rm ./docs/**/*.gz &>/dev/null


