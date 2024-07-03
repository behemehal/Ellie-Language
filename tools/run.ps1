param (
    [string]$inputFilePath,
    [string]$additionalParams
)

# Define the compile and run commands with manifest paths
$compileCommand = "cargo run --manifest-path=../elliec/Cargo.toml -- compile $inputFilePath -s -a -e -o $additionalParams"
$runCommand = "cargo run --manifest-path=../ellievm/Cargo.toml -- run $($inputFilePath -replace '\.ei$', '.eic') -d $($inputFilePath -replace '\.ei$', '.eig') -a"

# Start the compile command process
$compileProcess = Start-Process -FilePath "cmd" -ArgumentList "/c $compileCommand" -PassThru -NoNewWindow

# Wait for the compile process to complete
$compileProcess.WaitForExit()

# Check if the compile process exited with an error and if StandardError is not null
if ($compileProcess.ExitCode -ne 0) {
    Write-Output "Compile command failed with exit code $($compileProcess.ExitCode). Error output:"
} else {
    Write-Output "Compile command completed successfully."
    # Execute the run command if the compile command succeeded
    Invoke-Expression $runCommand
}
