cd deployer
cargo build --release
cp target/release/deployer ../bin
cd ..
cd main
cargo build --release
cp target/release/pogdotnet ../bin
cd ..
echo "Getting ready for installation."
echo "Please enter your password."
sudo mkdir -p /etc/pdn
echo "src/main.rs -> /etc/pdn"
sudo cp main/src/main.rs /etc/pdn
echo "bin/pogdotnet -> /bin/pdn"
sudo cp bin/pogdotnet /bin/pdn
echo "bin/deployer -> /bin/pdnd"
sudo cp bin/deployer /bin/pdnd