#!/bin/bash
ver=(`sed -n 4p Cargo.toml`)
ellie_std_ver=(`sed -n 4p ./lib/ellie.ei | sed 's/;//'`)
echo -e "\e[33m[Info]\e[0m Starting build v${ver[2]//\"} - EllieSTD v${ellie_std_ver[2]//\"}"
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
echo -e "\e[33m[Info]\e[0m Building for Linux x64"
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
    mv './target/release/elliec' './Release/elliec'
    echo -e "\e[32m[Info]\e[0m Move success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to move build output, cleaning folder"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Building ellieStd"
{
    ./Release/elliec compile ./lib/ellie.ei -m ellieStd -c "Ellie Standard Types" -p ./Release/ellieStd.bin -b ${ellie_std_ver[2]//\"}
    echo -e "\e[32m[Info]\e[0m Building ellieStd success"
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
echo -e "\e[33m[Info]\e[0m Building for Windows x64"
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
    mv './target/x86_64-pc-windows-gnu/release/elliec.exe' './Release/elliec_windows_x64.exe'
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


echo -e "\e[33m[Info]\e[0m Building for MacOS m1"
{
    cargo build --release -q --target aarch64-apple-darwin
    echo -e "\e[32m[Info]\e[0m Build success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to build release for windows, maybe `aarch64-apple-darwin` toolkit is not downloaded\nRuning:\nrustup target add aarch64-apple-darwin\nrustup toolchain install stable-aarch64-apple-darwin\ncommands may help. Cleaning folder anyways"
    exit 1
}
echo -e "\e[33m[Info]\e[0m Moving target"
{
    mv './target/aarch64-apple-darwin/release/elliec' './Release/elliec_macos_m1'
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

: '
cd ./native-bridge
echo -e "\e[33m[Info]\e[0m Creating native_bridge headers"
{
    cbindgen --config ./cbindgen.toml -o ../Release/ellie_native_bridge.h
    echo -e "\e[32m[Info]\e[0m Build success"
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to build headers. Cleaning folder anyways"
    exit 1
}
: '
#cd ../
cd ./Release
echo -e "\e[33m[Info]\e[0m Creating shasum"
{
    windows_s=`sha256sum -b ./elliec.exe`
    linux_s=`sha256sum -b ./elliec`
    #headers=`sha256sum -b ./ellie_native_bridge.h`
    std=`sha256sum -b ./ellieStd.bin`
    file="EllieBuild: v${ver[2]//\"}\\n\\t$windows_s\\n\\t$linux_s\\n\\t$std"
    printf "$file" > ./SHASUMS256.txt
    echo -e "\e[32m[Info]\e[0m Release complete"
    exit 0
} || { 
    rm -r ./Release/
    echo -e "\e[31m[Error]\e[0m Failed to create shasum, cleaning folder"
    exit 1
}
