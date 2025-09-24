# Security Policy

## Supported Versions

We provide security updates for the following versions of the DS Common AWS Rust Library:

| Version | Supported          |
| ------- | ------------------ |
| 0.1.x   | :white_check_mark: |

## Reporting a Vulnerability

We take security seriously. If you discover a security vulnerability in the DS Common AWS Rust Library, please report it responsibly.

### How to Report

**Please do NOT report security vulnerabilities through public GitHub issues.**

Instead, please report them via one of the following methods:

1. **Email**: Send details to <help@aider.no>
2. **GitHub Security Advisory**: Use GitHub's private vulnerability reporting feature

### What to Include

When reporting a vulnerability, please include:

- Description of the vulnerability
- Steps to reproduce the issue
- Potential impact
- Any suggested fixes or workarounds
- Your contact information (optional, but helpful for follow-up)

### Response Timeline

- **Acknowledgment**: Within 48 hours
- **Initial Assessment**: Within 7 days
- **Resolution**: Depends on severity and complexity

### What to Expect

1. We will acknowledge receipt of your report
2. We will investigate and assess the vulnerability
3. We will work on a fix if the vulnerability is confirmed
4. We will coordinate disclosure with you
5. We will release a security update and credit you (if desired)

## Security Best Practices

When using the DS Common AWS Rust Library:

- **Keep dependencies updated**: Regularly update to the latest version
- **Secure AWS credentials**: Use IAM roles and least privilege principles
- **Validate AWS responses**: Ensure AWS responses are properly validated
- **Handle AWS errors securely**: Don't expose internal system details in error messages
- **Sanitize AWS data**: Remove sensitive data from logs and error messages
- **Use encrypted parameters**: Always use encrypted SSM parameters for sensitive data
- **Monitor AWS operations**: Watch for unusual AWS operation patterns
- **Secure configuration**: Ensure AWS configuration doesn't expose sensitive data

## Security Features

The library includes several security-focused features:

- **Type safety**: Rust's type system helps prevent many common vulnerabilities
- **Memory safety**: No buffer overflows or use-after-free issues
- **AWS credential security**: Secure handling of AWS credentials and tokens
- **Encrypted parameter support**: Built-in support for encrypted SSM parameters
- **AWS error handling**: Secure handling of AWS-specific error types
- **Configuration validation**: Type-safe AWS configuration management
- **Thread safety**: Safe for use in multi-threaded applications without data races
- **Secure defaults**: Secure default configurations for AWS operations

## Dependencies

We regularly audit our dependencies for known vulnerabilities. If you find a security issue in one of our dependencies, please report it to us as well.

## Changelog

Security-related changes will be documented in the [CHANGELOG.md](CHANGELOG.md) file.

## Contact

For security-related questions or concerns, please contact us at <help@aider.no>.
