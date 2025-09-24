# Contributing to DS Common AWS Rust Library

Thank you for your interest in contributing to the DS Common AWS Rust Library! This document provides guidelines and information for contributors.

## Getting Started

1. **Fork the repository** on GitHub
2. **Clone your fork** locally:

   ```bash
   git clone https://github.com/your-username/ds-common-aws-rs-lib.git
   cd ds-common-aws-rs-lib
   ```

3. **Add the upstream remote**:

   ```bash
   git remote add upstream https://github.com/grasp-labs/ds-common-aws-rs-lib.git
   ```

## Development Setup

### Prerequisites

- Rust 1.76.0 or later
- AWS CLI configured (for testing AWS integrations)
- AWS credentials configured (for testing AWS operations)

### Building

```bash
cargo build
```

### Running Tests

```bash
cargo test
```

### Running Examples

```bash
# Basic AWS configuration example
cargo run --example basic_aws_config

# SSM parameter retrieval example
cargo run --example ssm_parameters

# AWS error handling example
cargo run --example aws_error_handling

# Environment-specific configuration example
cargo run --example env_config
```

### Development Testing

```bash
# Run all tests
cargo test

# Run specific test modules
cargo test error::tests

# Run tests with verbose output
cargo test -- --nocapture
```

## Making Changes

### Code Style

- Follow Rust conventions and use `cargo fmt` to format code
- Use `cargo clippy` to check for linting issues
- Ensure all tests pass with `cargo test`
- Add tests for new functionality

### Commit Messages

Use clear, descriptive commit messages:

```text
feat: add support for custom error codes
fix: resolve error serialization issue
docs: update README with new error handling examples
test: add integration tests for error context
perf: optimize error creation performance
```

### Pull Request Process

1. **Create a feature branch** from `main`:

   ```bash
   git checkout -b feature/your-feature-name
   ```

2. **Make your changes** and ensure they pass all tests

3. **Update documentation** if you've added new features or changed APIs

4. **Push your branch** and create a pull request:

   ```bash
   git push origin feature/your-feature-name
   ```

5. **Fill out the PR template** with a clear description of your changes

## Areas for Contribution

- **Bug fixes** - Report and fix issues with AWS SDK integration
- **New features** - Add support for additional AWS services (S3, DynamoDB, etc.)
- **Documentation** - Improve examples, docs, and comments
- **Performance** - Optimize AWS operations performance and memory usage
- **Testing** - Add more comprehensive tests for AWS operations
- **Examples** - Create more usage examples for different AWS service patterns
- **Integration** - Add support for additional AWS services or features
- **Configuration** - Enhance AWS configuration management
- **Error handling** - Improve AWS error handling and recovery mechanisms

## Reporting Issues

When reporting issues, please include:

- Rust version (`rustc --version`)
- Library version
- Operating system
- AWS SDK version
- Steps to reproduce
- Expected vs actual behavior
- Relevant error messages or stack traces
- Code examples that demonstrate the issue
- AWS region and service being used

## Code of Conduct

This project follows the [Rust Code of Conduct](https://www.rust-lang.org/policies/code-of-conduct). Please be respectful and inclusive in all interactions.

## Questions?

Feel free to open an issue for questions or start a discussion. We're happy to help!
