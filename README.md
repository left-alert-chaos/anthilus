# Anthilus
Anthilus is a lightweight file manager for your Linux terminal!

It can display directories, preview files, show file types, rename/move files, and delete things!
It uses no external frameworks outsite of `cc` for compilation, with all the TUI rendering logic being written independently.

# Install
You need Cargo to download Anthilus. There are directions [here](https://doc.rust-lang.org/cargo/getting-started/installation.html). For most Linux users, it's just this command:
``` Shell
curl https://sh.rustup.rs -sSf | sh
```

You can install Anthilus by downloading it from [crates.io](https://crates.io):
``` Shell
cargo install anthilus
```

You can then run it with the `anthilus` command.

## Configuring `PATH`
If your shell can't find Anthilus, you might not have configured your `PATH`. You'll need to place this line in your shell's config file (`~/.bashrc` for Bash users and `~/.zshrc` for Zsh users):

``` Shell
export PATH="$PATH:~/.cargo/bin/"
```

# Usage
Anthilus can be run without any flags or arguments, and will start in the current directory. Optionally, you can supply one argument that will be used as the starting directory. The `--help` flag can be used to print help text. The `--log` flag will print Anthilus' logs (located at `~/.cache/anthilus/anthilus.log`).

# Platform
Currently, Anthilus is only tested on Linux. It is probably not Windows-compatible, but _might_ work with macOS.

# AI
No artificial intelligence was knowingly used in the development of Anthilus. Please do not direct agents to contribute.
