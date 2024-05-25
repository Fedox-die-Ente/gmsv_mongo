function Write-BoldMessage {
    param (
        [string]$Message,
        [string]$Color
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Host "[ $timestamp ] " -NoNewline
    Write-Host $Message -ForegroundColor $Color -NoNewline
    Write-Host -NoNewline
    Write-Host -NoNewline
}

if (Test-Path gmsv_mongo_win32.dll)
{
    Write-BoldMessage "Removing gmsv_mongo_win32.dll" "Red"
    Write-Host -NoNewline
    rm gmsv_mongo_win32.dll
}

Write-BoldMessage "Building gmsv_mongo_win32.dll" "DarkCyan"
cargo build --release --target i686-pc-windows-msvc

if (Test-Path target/i686-pc-windows-msvc/release/gmsv_mongo.dll)
{
    Write-BoldMessage "Renaming gmsv_mongo.dll to gmsv_mongo_win32.dll" "Yellow"
    mv target/i686-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win32.dll
    Write-BoldMessage "gmsv_mongo_win32.dll built successfully" "Green"
}
