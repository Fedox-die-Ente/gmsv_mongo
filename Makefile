win32:
	cargo build --release --target i686-pc-windows-msvc && mv target/i686-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win32.dll
win64:
	cargo build --release --target x86_64-pc-windows-msvc && mv target/x86_64-pc-windows-msvc/release/gmsv_mongo.dll gmsv_mongo_win64.dll
linux32:
	cargo build --release --target i686-unknown-linux-gnu && mv target/i686-unknown-linux-gnu/release/gmsv_mongo.so gmsv_mongo_linux32.so
linux64:
	cargo build --release --target x86_64-unknown-linux-gnu && mv target/x86_64-unknown-linux-gnu/release/gmsv_mongo.so gmsv_mongo_linux64.so