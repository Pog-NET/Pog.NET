cd deployer
cargo build --release
cp target/release/deployer ../bin
cd ..
cd main
cargo build --release
cp target/release/pogdotnet ../bin
cd ..