function Write-BoldMessage {
    param (
        [string]$Message,
        [string]$Color
    )
    $timestamp = Get-Date -Format "yyyy-MM-dd HH:mm:ss"
    Write-Host "[ $timestamp ] $Message" -ForegroundColor $Color
    Write-Host "`n"
}

if (Test-Path gmsv_mongo_win64.dll)
{
    Write-BoldMessage "Removing gmsv_mongo_win64.dll" "Red"
    rm gmsv_mongo_win64.dll
}

Write-BoldMessage "Building gmsv_mongo_win64.dll" "DarkCyan"
cargo build --release --target x86_64-pc-windows-msvc

if (Test-Path target/x86_64-pc-windows-msvc/release/gmsv_mongo.dll)
{
    Write-BoldMessage "Renaming gmsv_mongo.dll to gmsv_mongo_win64.dll" "Yellow"
    mv target/x86_64-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win64.dll
    Write-BoldMessage "gmsv_mongo_win64.dll built successfully" "Green"
}