# Horizon Mechanical Works (HMW)

Horizon Mechanical Works is a production-grade, single-purpose mechanical automation mod for Horizon Minecraft, written entirely in Rust. This mod provides advanced mechanical automation features for the Horizon engine.

## Overview

This mod implements a comprehensive mechanical automation system that allows players to create and manage complex automated machinery within their Horizon worlds. The system includes features such as:
- Automated block placement and removal
- Mechanical contraptions with configurable behaviors
- Integration with Horizon's existing game systems
- Extensible architecture for custom mechanical components

## Architecture

The mod follows the Horizon native mod template structure, using the horizon-mixins framework to inject functionality into the Horizon engine. The codebase is organized for maintainability and scalability.

## Features

- Modular design for easy extension
- Comprehensive testing suite
- Production-ready error handling
- Performance optimized operations
- Clear separation of concerns

## Getting Started

### Prerequisites

- Rust 1.95.0 or later
- Python 3.12+ 
- Git
- Cargo

### Building

```bash
cargo fetch --locked
cargo test --workspace --locked --offline
```

### Development

The mod uses the standard Horizon development workflow:
1. Clone the Horizon and horizon-mixins repositories
2. Set up the pinned revisions as specified in compatibility.json
3. Use the provided build scripts for staging and testing

## Project Structure

```text
Cargo.toml                         Main package configuration
compatibility.json                 Compatible upstream revisions
mixins/mechanical_works/           Core mod implementation
  horizon-mixin.toml               Module and injection declarations
  break.rs                         Hook adapter
  runtime.rs                       Runtime logic and tests
tools/                             Build and test utilities
  audit/                           Source audit tools
  horizon.py                       Staging and build scripts
  package.py                       Packaging script
  test_stage.py                    Stage testing
  test_template.py                 Template testing
```

## License

MIT License - see LICENSE file for details.