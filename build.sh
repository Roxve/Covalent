# build binary and libs

cargo build -r --target-dir out 
cp out/release/covalent out 

mkdir out/libs
rustc --crate-type staticlib src/cova_std/std.rs -o out/libs/libstd.a
