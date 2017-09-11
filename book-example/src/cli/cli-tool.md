# Command Line Tool

mdBook can be used either as a command line tool or a [Rust crate](https://crates.io/crates/mdbook).
Let's focus on the command line tool capabilities first.

## Install

There are different ways to install mdBook. The simplest, is to use the [pre-built binaries](https://github.com/azerupi/mdBook/releases) we provide on GitHub. We have pre-built binaries for Windows, MacOS and Linux.

If you have Rust and Cargo installed, you can also install from [Crates.io](https://crates.io/crates/mdbook) or from source. If you want to automatically deploy your books using continuous integration services, like Travis or other similar tools, refer to the [Automatic deployment]() section.


### Installing from Crates.io

If you have [Rust](https://www.rust-lang.org) installed on your system, you may want to install mdBook from Crates.io directly. The latest version on Crates.io should be equivalent to the pre-built binaries we provide. To install mdBook from crates.io, you simply type the following command in your terminal:

```bash
cargo install mdbook
```

This will fetch the source code from [Crates.io](https://crates.io/) and compile it. You will have to add Cargo's `bin` directory to your `PATH`.

Run `mdbook help` in your terminal to verify that it works. Congratulations, you have installed mdBook!


### Installing from source

The **[git version](https://github.com/azerupi/mdBook)** contains all the latest bug-fixes and features that will be shipped in the next version on **Crates.io**, if you can't wait until the next release. You can build the git version yourself. Open your terminal and navigate to the directory of your choice. We need to clone the git repository and then build it with Cargo. 

```bash
git clone --depth=1 https://github.com/azerupi/mdBook.git
cd mdBook
cargo build --release
```

The executable `mdbook` will be in the `./target/release` folder, this should be added to your path.

### Automatic deployment

It is possible to automatically deploy your books for your project using services like Travis. Installing mdBook on Travis is easy, you can use the crates.io method. However, there is one slightly different recommendation we make for automatic deployment scripts. We advice users to pin the installed version of mdBook to a semver compatible range, like below.

```bash
cargo install mdbook --vers "^0.1.0"
```

This allows the script to install the latest *non-breaking* version of mdBook available. You get all the latest fixes and features without the need to worry that a new version of mdBook could break the deployment of your book. When a new breaking release of mdBook ships, you can manually choose to update and bump the version range in your script.