#!/bin/bash

inputFilePath="$1"
additionalParams="$2"

# Define the compile and run commands with manifest paths
compileCommand="cargo run --manifest-path=../elliec/Cargo.toml -- compile  $inputFilePath -s -a -e -o $additionalParams"
runCommand="cargo run --manifest-path=../ellievm/Cargo.toml -- run ${inputFilePath%.ei}.eic -d ${inputFilePath%.ei}.eig -a"

#compileCommand="../elliec/target/release/elliec compile $inputFilePath -s -a -e -o $additionalParams"
#runCommand="../ellievm/target/release/ellievm run ${inputFilePath%.ei}.eic -d ${inputFilePath%.ei}.eig -a"

# Start the compile command process
$compileCommand

# Check if the compile command exited with an error
if [ $? -ne 0 ]; then
    echo "Compile command failed with exit code $?. Error output:"
else
    echo "Compile command completed successfully."
    # Execute the run command if the compile command succeeded
    eval $runCommand
fi
