# Contributing to DR-Plugins-SDK

Thank you for your interest in contributing to DR-Plugins-SDK! This document provides guidelines and instructions for contributing.

## 📋 Table of Contents

- [Code of Conduct](#code-of-conduct)
- [Getting Started](#getting-started)
- [Development Workflow](#development-workflow)
- [Coding Standards](#coding-standards)
- [Pull Request Process](#pull-request-process)
- [Testing](#testing)
- [Documentation](#documentation)
- [Security](#security)

## Code of Conduct

This project adheres to a code of conduct. By participating, you are expected to uphold this code. Please report unacceptable behavior to the project maintainers.

## Getting Started

### Prerequisites

- Rust 1.75.0 or later
- cargo (Rust package manager)
- For Windows cross-compilation: mingw-w64

### Installation

```bash
# Clone the repository
git clone https://github.com/LuxVTZ/DR-Plugins-SDK.git
cd DR-Plugins-SDK

# Verify Rust installation
rustc --version
cargo --version
```

### Building

```bash
# Build in debug mode
cargo build

# Build for Windows (requires mingw-w64)
cargo build --target x86_64-pc-windows-gnu --release

# Run tests
cargo test

# Check formatting
cargo fmt -- --check

# Run clippy lints
cargo clippy --all-targets --all-features -- -D warnings
```

## Development Workflow

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/DR-Plugins-SDK.git
cd DR-Plugins-SDK
git remote add upstream https://github.com/LuxVTZ/DR-Plugins-SDK.git
```

### 2. Create a Branch

```bash
git checkout -b feature/your-feature-name
# or
git checkout -b fix/your-bug-fix
```

### 3. Make Changes

- Write code following the coding standards
- Add tests for new functionality
- Update documentation as needed
- Ensure all tests pass

### 4. Commit Changes

```bash
git add .
git commit -m "feat: add your feature description"
# or
git commit -m "fix: resolve issue with ..."
```

Follow [Conventional Commits](https://www.conventionalcommits.org/) specification.

### 5. Push and Create PR

```bash
git push origin feature/your-feature-name
# Then create a Pull Request on GitHub
```

## Coding Standards

### Rust Style

- Use `cargo fmt` to format code
- Follow Rust API guidelines
- Use `cargo clippy` to catch common mistakes
- Write idiomatic Rust code

### no_std Compatibility

- All code must be `no_std` compatible
- No use of standard library features
- Use core primitives only
- Avoid allocations

### Safety

- Minimize `unsafe` blocks
- Document all `unsafe` code with safety comments
- Use type-safe wrappers where possible
- Validate all inputs

### Performance

- Prefer compile-time computation over runtime
- Use zero-cost abstractions
- Optimize for size in release builds
- Avoid unnecessary dependencies

### Documentation

- Document all public APIs
- Include examples in doc comments
- Use `#![deny(missing_docs)]` where appropriate
- Keep README.md up to date

## Pull Request Process

### PR Requirements

1. **Description**: Clearly describe what changes were made and why
2. **Tests**: Include tests for new functionality
3. **Documentation**: Update relevant documentation
4. **Commits**: Use clear, conventional commit messages
5. **CI**: All CI checks must pass

### PR Template

```markdown
## Summary
Brief description of changes

## Changes
- Change 1
- Change 2
- ...

## Testing
- [ ] Unit tests pass
- [ ] Clippy passes
- [ ] Formatting is correct
- [ ] Cross-compilation works

## Related Issues
Fixes #123
```

### Review Process

1. PR will be reviewed by maintainers
2. Address all review comments
3. CI checks must pass
4. PR will be squashed and merged

## Testing

### Unit Tests

```bash
cargo test
```

### Cross-Compilation

```bash
# Check compilation
cargo check --target x86_64-pc-windows-gnu --release

# Build
cargo build --target x86_64-pc-windows-gnu --release
```

### Code Quality

```bash
# Format check
cargo fmt -- --check

# Lint check
cargo clippy --all-targets --all-features -- -D warnings

# Security audit
cargo audit
```

## Documentation

### Writing Documentation

- All public items must have documentation
- Use `///` for doc comments
- Include examples where helpful
- Document safety requirements for `unsafe` code

### Building Documentation

```bash
cargo doc --open
```

### Documentation Style

```rust
/// Short description
///
/// Longer explanation with details
///
/// # Examples
///
/// ```
/// use milow_plugin_sdk::Example;
/// let x = Example::new();
/// ```
///
/// # Safety
///
/// This function is unsafe because...
pub unsafe fn example() { ... }
```

## Security

### Reporting Vulnerabilities

**DO NOT** open public issues for security vulnerabilities.

Email security concerns to: `luxvtz@protonmail.com`

### Security Best Practices

1. Never commit secrets or credentials
2. Use type-safe wrappers
3. Validate all inputs
4. Minimize unsafe code
5. Follow principle of least privilege

## Release Process

### Creating a Release

1. Update version in `Cargo.toml`
2. Update CHANGELOG.md
3. Create and push tag: `git tag v1.0.0`
4. GitHub Actions will build and publish

### Versioning

Follow Semantic Versioning:
- MAJOR: Breaking changes
- MINOR: New features (backward compatible)
- PATCH: Bug fixes

## Communication

- GitHub Issues: For bugs and feature requests
- Pull Requests: For code contributions
- Discussions: Use GitHub Discussions for questions

## License

By contributing, you agree that your contributions will be licensed under the MIT License.

## Questions?

Feel free to open an issue for questions or discussion.

---

**Thank you for contributing!** 🚀