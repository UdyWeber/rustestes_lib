# rustestes_lib
------
A Library powered by Rust exported via Webassembly to Python using Wasmer and WAPM for studies purposes

## Getting Started with Wasmer
------
First you'll need to install wasmer in your system by running `curl https://get.wasmer.io -sSfL | sh` in your terminal it will alloy you to use wasmer in your project. Having installed Wasmer, WAPM (Web Assembly Package Manager) will be installed as well, abling you to upload your Rust Lib to WAPM and delivery to other languages. More documentation on: `https://docs.wasmer.io/`

### WAPM
-----
After installing Wasmer and WAPM to your system, go to `https://wapm.io/` and create an account to store your Rust libs as WebAssembly packages.

## Setting-up your Rust project
------
If you're using Rust you have to have the cargo service in your system, to make things easier! So by typing `cargo new --lib {project_name}` and voilá, cargo will create a new lib.rs and cargo.toml for you to get started. When you create a new lib module using cargo, it will provide you examples of how to implement and test your pieces of code, with great documentation online in the Rust Book.

In order to define your lib's implementations you have to create a `.wai`, the wai file have to be `{lib_name}.wai` where your going to define a interface to your lib methods and also structs, enums, etc... More information on how to build the wai file in the official documentation: `https://github.com/wasmerio/wai`.

### Cargo.toml
-----
To get started with wai-bindgen you have to add this depency to your cargo.toml by typing `cargo add wai-bindgen-rust`, it will install the latest version of wai-bindgen, if you need to add a specific version you can do it by adding a `@` and the version of the package `cargo add wai-bindgen-rust@{version}`

Also for this project we need to install the `cargo-expand` tool to see what your lib struct is implementing and expand it to follow the instructions from your `.wai` file. I highly recommend to add cargo-expand to your default cargo tool set by typing `cargo install cargo-expand`, more documentation about cargo-expand in their git repository: `https://github.com/dtolnay/cargo-expand`

#### Lib configs
----- 
Also in your cargo.toml you'll need to add some infos to be able to upload your packages to WAPM.

First you have to add a [lib] section to cargo.toml, it makes you able to create the codegen of your lib to `.wasm` also to `.rlib` format.

```yaml
[lib]
# Need to specify CDYLIB in order to get a .wasm file
crate-type = ["cdylib", "rlib"]
```

In order to upload your package to WAPM you have to fill up the [package] information

```yaml
[package]
name = "{lib_name}"
# Always have to change the version to update the package we are creating
version = "{lib_version, starting at 0.1.0}"
edition = "{edition}"
# It's required that you pass in a description for your project
description = "{lib_description}"
```

Also you'll need to create a [package.metadata.wapm] section, to give infos about your WAPM account and `.wai` that will be the lib WebAssembly Interface.

```yaml
[package.metadata.wapm]
namespace = "{WAPM_username}"
abi = "none"
bindings = { wai-version = "0.1.0", exports = "{lib_name}.wai"}
```

## Lib Code Implementation
-----
Now that we are ready with all the setup, we can jump in to write some code! So in your `lib.rs` file you have to add

```rust
wai_bindgen_rust::export!("{lib_name}.wai");
```

This line of code will read and export the interface that you have created in the `file.wai` to be used and implemented by your Rust code. In order to implement methods defined in your wai file, you'll have to create a public struct that will implement those methods

```rust
pub struct LibStruct;

impl {lib_name}::{LibName} for LibStruct {
    // CODE IMPLEMENTATION
}
```
And that's all, for the Lib implementation, be aware to follow the exact wai recipe to implement your methods or the code won't even build!

## Building and Uploading
-----
Now let's compile our Rust Lib to be able to upload and use it dinamicly!

### Building Step
-----
Having followed the previous steps till now all you have to do to build your wasm package is type `cargo build --release --target wasm32-unknown-unknown` you can check if the .wasm file was created correctly by typing `ls target/wasm32-unknown-unknown/debug` it should have a `{lib_name}.wasm` file. To see what's inside of your .wasm file you can use wasmer to inspect the file by typing `wasmer inspect target/wasm32-unknown-unknown/debug/{lib_name}.wasm` and it will show the source to your WebAssembly.

### Uploading Step
-----
With your account created in WAPM you can create a access token that will give you a command line to log-in to WAPM in your PC. After having logged-in to your account just type `cargo wapm` in your terminal and it should upload the created .wasm file to WAPM


### Using Package
-----
Having uploaded the Wasm file to WAPM you can go to `https://wapm.io/` and enter your packages informations and there they will display command lines to install your WebAssembly Package via most famous languages package managers, having great support for the Python and JS programming languages. For other programming languages you can check in the official documentation: `https://docs.wasmer.io/integrations/rust`.

To use your package in Python you just have to type in `wapm install {wapm_username}/{lib_name} --pip` and it's installed to your project! You can see more in how to use it in the `test.py` example.