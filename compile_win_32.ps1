cargo build --release --target i686-pc-windows-msvc
if (Test-Path target/i686-pc-windows-msvc/release/gmsv_mongo.dll)
{
    mv target/i686-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win32.dll
}