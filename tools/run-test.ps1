param(
    [Parameter(Mandatory = $true, HelpMessage = "Test path relative to tests/, without extension. Example: math/fib")]
    [string]$test,
    [Parameter(HelpMessage = "Use gen2 compiler and VM")]
    [switch]$gen2
)

$toolsDir  = $PSScriptRoot
$testFile  = "tests/$test.ei"

if (-not (Test-Path (Join-Path $toolsDir $testFile))) {
    Write-Host "ERROR: test file not found: $toolsDir\$testFile" -ForegroundColor Red
    exit 1
}

Push-Location $toolsDir
try {
    if ($gen2) {
        $output    = "tests/$test.eic2"
        $debugFile = "tests/$test.eig"

        Write-Host ""
        Write-Host "==> Compiling $testFile (gen2)" -ForegroundColor Cyan
        cargo run --manifest-path=../elliec/Cargo.toml -- compile $testFile -e -o byteCode2 -p $output --disable-colors
        if ($LASTEXITCODE -ne 0) {
            Write-Host ""
            Write-Host "COMPILE FAILED" -ForegroundColor Red
            exit 1
        }

        Write-Host ""
        Write-Host "==> Running $output (gen2)" -ForegroundColor Cyan
        cargo run --manifest-path=../ellievm/Cargo.toml -- run $output -a
    } else {
        $output    = "tests/$test"
        $debugFile = "tests/$test.eig"

        Write-Host ""
        Write-Host "==> Compiling $testFile" -ForegroundColor Cyan
        cargo run --manifest-path=../elliec/Cargo.toml -- compile $testFile -s -e -o byteCode -p $output --disable-colors
        if ($LASTEXITCODE -ne 0) {
            Write-Host ""
            Write-Host "COMPILE FAILED" -ForegroundColor Red
            exit 1
        }

        Write-Host ""
        Write-Host "==> Running $output" -ForegroundColor Cyan
        cargo run --manifest-path=../ellievm/Cargo.toml -- run $output -d $debugFile -a
    }

    if ($LASTEXITCODE -ne 0) {
        Write-Host ""
        Write-Host "RUN FAILED" -ForegroundColor Red
        exit 1
    }

    Write-Host ""
    Write-Host "==> Done" -ForegroundColor Green
} finally {
    Pop-Location
}
