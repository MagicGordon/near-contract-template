# NEAR Contract Template

A comprehensive NEAR smart contract template featuring access control, pausable functionality, and upgradability using near-plugins.

## Features

- 🔒 **Access Control**: Role-based permissions (DAO, PauseManager, UnpauseManager, etc.)
- ⏸️ **Pausable**: Contract can be paused/unpaused by authorized roles
- 🔄 **Upgradable**: Support for contract upgrades with proper state migration
- 📦 **Versioned State**: Future-proof state management with `VersionedContractData`
- ❌ **Error Handling**: Custom error types with thiserror integration
- 🪙 **Mock FT**: Includes a mock fungible token contract for testing

## Quick Start

### Prerequisites

```bash
# Install cargo-generate
cargo install cargo-generate
```

### Create a New Project

```bash
cargo generate --git https://github.com/MagicGordon/near-contract-template.git
```

This will prompt you for:
- **Project name**: Your project name (e.g., `my-project`)
- **Contract name**: Your main contract name (e.g., `study_near`)
- **Author**: Your GitHub username (for repository URL)

### Development

```bash
# Build the contract
make build

# Format and lint
make lint

# Run tests
make test

# Build release version
make release

# Clean build artifacts
make clean
```

## Project Structure

```
{{project-name}}/
├── contracts/
│   ├── {{contract-name}}/          # Main contract
│   │   ├── src/
│   │   │   ├── lib.rs          # Contract logic
│   │   │   ├── errors.rs       # Error definitions
│   │   │   └── upgrade.rs      # State migration
│   │   └── Cargo.toml
│   └── mock_ft/                # Mock fungible token
├── Cargo.toml                  # Workspace configuration
├── Makefile                    # Build scripts
└── CLAUDE.md                   # Development guide
```

## Configuration

The template uses:
- **NEAR SDK**: 5.17
- **Rust**: 1.86.0 toolchain
- **near-plugins**: For access control, pausable, and upgradable functionality
