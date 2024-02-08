# build binary and libs

cargo build -r --target-dir out 
cp out/release/covalent out 
rm -rf out/release
mkdir out/libs
rustc --crate-type staticlib src/cova_std/mod.rs -o out/libs/libstd.a
