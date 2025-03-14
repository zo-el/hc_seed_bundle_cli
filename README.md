# Holochain Seed Bundle CLI

A command-line tool for managing Holochain seed bundles (`.hcsb` files). This tool allows you to display and work with Holochain agent keys stored in encrypted seed bundles.

## Features

- Display public keys from encrypted seed bundles
- Support for multiple unlock methods:
  - Password-based authentication
  - Security questions (3-question verification)
- Output formats:
  - Raw public key (hex encoded)
  - AgentPubKeyB64 format (Holochain compatible)

## Usage

```bash
Usage: hc_seed_bundle_cli <COMMAND>

Commands:
  create  Create a new seed bundle
  unlock  Display the contents of a seed bundle
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version

```

## Authentication Methods

The tool will automatically detect available authentication methods for your seed bundle:

### Password Authentication

If the bundle is password-protected, you'll be prompted to enter your password.

### Security Questions

If the bundle uses security questions, you'll be prompted to answer three security questions in sequence.

## Output Format

The tool will display:

- Public Key (raw hex format)
- Public Key (AgentPubKeyB64 format for Holochain compatibility)

## Installation

```bash
cargo install hc_seed_bundle_cli
```

## License

GPL-3.0 [LICENSE](./LICENSE)
