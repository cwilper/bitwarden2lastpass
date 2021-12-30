# bitwarden2lastpass

Converts a BitWarden CSV export to a Generic CSV file suitable for import into LastPass.

## Building

```
cargo build --release
```

This will put the binary at `target/release/bitwarden2lastpass`.

## Usage

```
./bitwarden2lastpass input.csv output.csv
```

Where:

* `input.csv` is the path to your BitWarden export
* `output.csv` is the path where you want to save the LastPass Generic CSV for import
