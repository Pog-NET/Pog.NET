mkdir -p bin
echo "Building."
cd deployer
cargo build --release
cp target/release/deployer ../bin
cd ..
cd cli
cargo build --release
cp target/release/cli ../bin
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
echo "bin/pogdotnet -> /usr/bin/pdn_exec"
sudo cp bin/pogdotnet /usr/bin/pdn_exec
echo "bin/deployer -> /usr/bin/pdn_deploy"
sudo cp bin/deployer /usr/bin/pdn_deploy
echo "bin/cli -> /usr/bin/pognet"
sudo cp bin/cli /usr/bin/pognet