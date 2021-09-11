#!/bin/bash
ver=(`sed -n 4p Cargo.toml`)
echo -e "\e[33m[Info]\e[0m Starting build v${ver[2]//\"}"
{
    mkdir ./Release
} &> /dev/null || { 
    echo -e "\e[31m[Error]\e[0m Failed to create release directory"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Cleaning previous build"
{
    cargo clean
    echo -e "\e[32m[Info]\e[0m Clean success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to clean build, running command manualy may help `cargo clean`"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Building for Linux"
{
    cargo build -q --release
    echo -e "\e[32m[Info]\e[0m Build success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to build release for linux, cleaning folder"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Moving target"
{
    mv './target/release/ellie' './Release/ellie'
    echo -e "\e[32m[Info]\e[0m Move success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to move build output, cleaning folder"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Cleaning linux build"
{
    cargo clean
    echo -e "\e[32m[Info]\e[0m Clean success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to clean build, running command manualy may help `cargo clean`"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Building for Windows"
{
    cargo build --release -q --target x86_64-pc-windows-gnu
    echo -e "\e[32m[Info]\e[0m Build success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to build release for windows, maybe `x86_64-pc-windows-gnu` toolkit is not downloaded\nRuning:\nrustup target add x86_64-pc-windows-gnu\nrustup toolchain install stable-x86_64-pc-windows-gnu\ncommands may help. Cleaning folder anyways"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Moving target"
{
    mv './target/x86_64-pc-windows-gnu/release/ellie.exe' './Release/ellie.exe'
    echo -e "\e[32m[Info]\e[0m Move success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to move build output, cleaning folder"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Cleaning leftovers"
{
    cargo clean
    echo -e "\e[32m[Info]\e[0m Clean success"
} || {
    echo -e "\e[33m[Warning]\e[0m Failed to clean build continuing anyway, running command manualy may help `cargo clean`"
}
cd ./Release
echo -e "\e[33m[Info]\e[0m Creating shasum"
{
    windows_s=`sha256sum -b ./ellie.exe`
    linux_s=`sha256sum -b ./ellie`
    file="EllieBuild: v${ver[2]//\"}\\n\\t$windows_s\\n\\t$linux_s"
    printf "$file" > ./Release/SHASUMS256.txt
    echo -e "\e[32m[Info]\e[0m Release complete"
    exit 0
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to create shasum, cleaning folder"
    exit 1
}
