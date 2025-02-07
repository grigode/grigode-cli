# Grigode CLI

Grigode CLI is a tool for managing passwords and secret keys. It allows you to generate secure passwords, store them, update them, and efficiently manage secret keys from the command line.

## Installation

To install Grigode CLI, clone the repository and compile the binary:

```sh
git clone https://github.com/grigode/grigode-cli.git
cd grigode_cli
cargo build --release
```

## Usage

Run the following command to see the available options:

```sh
grigode_cli --help
```

### Generate a password

```sh
grigode_cli gpws -L 16 --uppercase --number --symbol
```

Available options:

- `-L, --length <NUM>`: Password length (default: 12 characters).
- `-u, --uppercase`: Include uppercase letters.
- `-l, --lowercase`: Include lowercase letters.
- `-n, --number`: Include numbers.
- `-s, --symbol`: Include symbols.

### Store a password

(Coming soon)

### Update a password

(Coming soon)

### Manage secret keys

(Coming soon)

## Examples

Generate a 20-character password with letters and numbers:

```sh
grigode_cli gpws -L 20 -u -l -n
```

Generate a 10-character password with symbols:

```sh
grigode_cli gpws -L 10 --symbol
```
