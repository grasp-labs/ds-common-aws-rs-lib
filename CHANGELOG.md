# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

### Added

- Initial release of DS Common AWS Rust Library
- High-level AWS SDK abstractions and utilities
- SSM Parameter Store integration with simplified API
- AWS configuration management helpers
- Comprehensive error handling for AWS operations
- Async/await support for all AWS operations
- Type-safe AWS service interactions

### Features

- **AWS SDK Integration** - High-level abstractions for AWS services
- **SSM Parameter Store** - Simplified parameter retrieval and management
- **Error Handling** - Comprehensive error handling with AWS-specific error types
- **Configuration Management** - Easy AWS configuration setup and management
- **Async Support** - Full async/await support for all operations
- **Type Safety** - Strong typing for AWS operations and responses

### Dependencies

- `aws-config` = "1.8.1" - AWS SDK configuration
- `aws-sdk-ssm` = "1.83.0" - AWS Systems Manager Parameter Store SDK
- `aws-sdk-sqs` = "1.39.0" - AWS Simple Queue Service SDK

## [0.1.0] - 2025-01-15

### Added

- Initial release
- AWS SDK integration with simplified configuration
- SSM Parameter Store client with high-level abstractions
- AWS configuration management utilities
- Comprehensive error handling for AWS operations
- Async/await support for all AWS operations
- Type-safe AWS service interactions
- Environment-specific configuration support
- Comprehensive documentation and examples

### Security

- Memory-safe implementation using Rust's ownership system
- No unsafe code blocks
- Thread-safe operations without data races
- Secure AWS credential handling
- Encrypted parameter retrieval support

### Documentation

- Complete API documentation with examples
- README with quick start guide
- Contributing guidelines
- Security policy
- Code of conduct
- Comprehensive test suite

---

## Security Information

Security-related changes will be documented in this changelog. For security vulnerabilities, please see [SECURITY.md](SECURITY.md).

## License

This project is licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT License ([LICENSE-MIT](LICENSE-MIT))

at your option.
