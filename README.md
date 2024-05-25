<h1 align="center" id="title">🚧 STILL IN DEVELOPMENT 🚧</h1>
<h1 align="center" id="title">gmsv_mongo</h1>

<p align="center"><img src="https://socialify.git.ci/Fedox-die-Ente/gmsv_mongo/image?font=Jost&issues=1&language=1&name=1&owner=1&pattern=Floating%20Cogs&pulls=1&stargazers=1&theme=Dark" alt="project-image"></p>
<p id="description">The gmsv_mongo module enables the usage of MongoDB connections within Garry's Mod (GMod) via Rust. This opens up diverse possibilities for database interactions and management within GMod servers.</p>

<p align="center"><img src="https://img.shields.io/badge/License-MIT-green" alt="shields">   <img src="https://img.shields.io/github/release/Fedox-die-Ente/gmsv_mongo?include_prereleases=&amp;sort=semver&amp;color=green" alt="shields"></p>

<h2>🛠️ Installation Steps:</h2>

<p>1. Download the correct file for your server operating system. You can execute the following console command to get the correct version.</p>

```
lua_run print(jit.os, jit.arch)
```

<p>2. Put the file in your <b>garrysmod/lua/bin</b> folder if it doesn't exists just create one.</p>

<p>3. Include the module in your code.</p>

```
require('mongo');
```

<h2>💻 Built with</h2>

Technologies used in the project:

* Rust
* [rglua](https://github.com/Vurv78/rglua)
* [mongodb](https://github.com/mongodb/mongo-rust-driver)
* [tokio](https://docs.rs/tokio/1.37.0/tokio/index.html)
* [serde](https://docs.rs/serde/latest/serde/)
* [dotenv](https://docs.rs/dotenv/0.15.0/dotenv/)
* [log](https://docs.rs/log/0.4.21/log/)
* [termcolor](https://docs.rs/termcolor/1.4.1/termcolor/)
* [futures](https://docs.rs/futures/0.3.30/futures/)

<h2>🔨 Build project</h2>

To build the sample project in debug mode, you need to specify the target architecture for your build.

| Platform  |                     Command                     |                                                          Description                                                           |
|:---------:|:-----------------------------------------------:|:------------------------------------------------------------------------------------------------------------------------------:|
|  `win32`  |   `cargo build --target i686-pc-windows-msvc`   | Windows 32-bit<br>Use this if your server is running Windows on the `main` branch of Garry's Mod (this is the default branch). |
|  `win64`  |  `cargo build --target x86_64-pc-windows-msvc`  |              Windows 64-bit<br>Use this if your server is running Windows on the `x86-64` branch of Garry's Mod.               |
|  `linux`  |  `cargo build --target i686-unknown-linux-gnu`  |   Linux 32-bit<br>Use this if your server is running Linux on the `main` branch of Garry's Mod (this is the default branch).   |
| `linux64` | `cargo build --target x86_64-unknown-linux-gnu` |                Linux 64-bit<br>Use this if your server is running Linux on the `x86-64` branch of Garry's Mod.                 |

If Rust reports it cannot find the target/toolchain, you may need to install it. By default, Rust installs the native
toolchain for your system, which is likely Windows 64-bit (`x86_64-pc-windows-msvc`).

Cross-compiling Linux binaries on Windows is not recommended. For compiling Linux binaries on Windows, use WSL.

### Using the Sample in Garry's Mod

First, rename the compiled binary to `gmsv_mongo_PLATFORM.dll`, where `PLATFORM` corresponds to one of the following:

| Platform  |                                                          Description                                                           |
|:---------:|:------------------------------------------------------------------------------------------------------------------------------:|
|  `win32`  | Windows 32-bit<br>Use this if your server is running Windows on the `main` branch of Garry's Mod (this is the default branch). |
|  `win64`  |              Windows 64-bit<br>Use this if your server is running Windows on the `x86-64` branch of Garry's Mod.               |
|  `linux`  |   Linux 32-bit<br>Use this if your server is running Linux on the `main` branch of Garry's Mod (this is the default branch).   |
| `linux64` |                Linux 64-bit<br>Use this if your server is running Linux on the `x86-64` branch of Garry's Mod.                 |

Next, move the renamed binary to `garrysmod/lua/bin/` on your server. If the `bin` folder does not exist, create it.

Finally, you can load the module from Lua with:

```lua
require("mongo")
```

<h2>🛡️ License:</h2>

This project is licensed under the [AGPL-3.0](LICENSE)

<h2>💖Like my work?</h2>

Leave a ⭐ on this repository :D