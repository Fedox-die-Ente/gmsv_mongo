function Write-BoldMessage {
    param (
        [string]$Message,
        [string]$Color
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Host "[ $timestamp ] $Message" -ForegroundColor $Color
    Write-Host "`n"
}

if (Test-Path gmsv_mongo_win32.dll)
{
    Write-BoldMessage "Removing gmsv_mongo_win32.dll" "Red"
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
