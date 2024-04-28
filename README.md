samples is a rust cli tool to sample lines from a text file(plain text, csv, log...) or from stdin

## Installation

### Cargo

```bash
cargo install samples
```

### Manual

```bash
wget https://github.com/rodrigobdz/samples/releases/download/v0.1.0/samples -O samples
```

## Usage

```bash
# from stdin
echo "1\n2\n3\n4\n5" | samples -k 2
5
1
```

```bash
# from a file
samples -k 2 example.txt
```

```bash
# pipe
cat example.txt | samples -k 2 > output.txt
```

## License

MIT or Apache-2.0