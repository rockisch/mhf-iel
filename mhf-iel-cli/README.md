# MHF IELess Launcher CLI

CLI interface for `mhf-iel`.

## Usage

1. Get a `mhf-iel-cli.exe` file by either compiling the project or downloading the latest release.
2. Download [`config.json`](config.json).
1. Copy both files to the MHF folder.
2. Modify `config.json` to have valid values. Specifically, make sure the `char_*` keys and `user_token` have correct values.
3. Run `mhf-iel-cli.exe`.

If you plan on using the CLI interface as the entrypoint of your external application, run `mhf-iel-cli.exe --help` to see some extra options available.

## Compiling

Before running `cargo build`, make sure you have the `nightly` toolchain and the `i686-pc-windows-msvc` target intalled:

```
rustup toolchain install nightly
rustup target add i686-pc-windows-msvc
```
