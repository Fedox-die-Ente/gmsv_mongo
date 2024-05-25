cargo build --release --target x86_64-pc-windows-msvc
if (Test-Path target/x86_64-pc-windows-msvc/release/gmsv_mongo.dll)
{
    mv target/x86_64-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win64.dll
}