@echo off
PUSHD %~dp0
rd /s /q bin
mkdir bin
echo "Building."
cd deployer
cargo build --release
copy target\release\deployer.exe ..\bin
cd ..
cd cli
cargo build --release
copy target\release\cli.exe ..\bin
cd ..
cd main
cargo build --release
copy target\release\pogdotnet.exe ..\bin
cd ..
echo "Getting ready for installation."
mkdir \"Program Files"\pdn
echo "src/main.rs -> /Program Files/pdn"
echo "bin/* -> Program Files"
copy main\src\main.rs "\Program Files\pdn"
copy bin\pogdotnet.exe "\Program Files\pdn\pdn_exec.exe"
copy bin\deployer.exe "\Program Files\pdn\pdn_deploy.exe"
copy bin\cli.exe "\Program Files\pdn\pdn.exe"
echo !! ADD "\Program Files\pdn" TO PATH !!
timeout 30