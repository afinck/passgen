# passgen

A simple and flexible command-line password generator written in Rust.  
`passgen` allows you to generate random passwords with customizable length and complexity, and optionally copy them directly to your clipboard.

---

## Features

- Generate a random password with a specified number of characters
- Optional complexity by adding special characters
- Option to copy the password directly to your clipboard
- Cross-platform support (Linux, macOS, Windows)
- Easy to use command-line interface

---

## Downloading RPM and DEB Packages

After each release, you can download the latest `.rpm` and `.deb` packages directly from the [GitLab Releases page](https://gitlab.com/finck_for_fun-group/passgen/-/releases).  
Select the desired release and download the package for your system from the "Assets" section.  
Alternatively, you can find the artifacts in the CI/CD pipeline job for each release.

---

## Installation

### Using Cargo (Rust)

Make sure you have Rust and Cargo installed. Clone the repository and build the project:

```bash
git clone https://gitlab.com/finck_for_fun-group/passgen.git
cd passgen
cargo build --release
```

The compiled binary will be located at `target/release/passgen`.

### Using RPM/DEB Packages

Download the appropriate package from the [Releases page](https://gitlab.com/finck_for_fun-group/passgen/-/releases) and install it:

**For RPM-based systems (Fedora, openSUSE, etc.):**
```bash
sudo dnf install passgen-<version>.rpm
```

**For DEB-based systems (Debian, Ubuntu, etc.):**
```bash
sudo dpkg -i passgen_<version>.deb
```

---

## Usage

Run the program with:

```bash
passgen --help
```

### Command-line Arguments

- `-n`, `--numbers <number>`: Number of characters in the password (default: 12)
- `-x`, `--complex`: Add special characters (default: false)
- `-c`, `--copy`: Copy the password to the clipboard (default: false)

### Example

Generate a 16-character complex password and copy it to the clipboard:

```bash
passgen -n 16 -x -c
```

---

## Example Output

```bash
$ passgen -n 16 -x
Generated password: 8f!kPz#2Qw@1Lm$e
```

If you use `-c`, the password will be copied to your clipboard and a confirmation will be printed.

---

## License

This project is licensed under the MIT License. See the [LICENSE](LICENSE) file for details.

---

## Contributing

Contributions are welcome! Please open issues or merge requests on GitLab.

---

## Author

Andreas Finck