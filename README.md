# my_app

A CLI tool for processing JSON and computing checksums.

## Installation

```bash
cargo install --path .
```

## Usage

### JSON Pretty Print
Parse and format JSON from files or stdin:

```bash
# From file
my_app --parse data.json

# From stdin
cat data.json | my_app --parse
```

### Checksum
Compute SHA256 checksums of files or stdin:

```bash
# From file
my_app --checksum file.txt

# From stdin
echo "hello" | my_app --checksum
```

## Development

Run tests:
```bash
cargo test
```

Run benchmarks:
```bash
cargo bench
```

Generate manual pages:
```bash
my_app man --out ./man-pages
```

Generate shell completions:
```bash
my_app completions bash > my_app.bash
```
