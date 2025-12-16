# Contributing to CourtListener Worker

Thank you for your interest in contributing! This document provides guidelines and instructions for contributing to this project.

## Code of Conduct

This project adheres to the [Contributor Covenant Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How to Contribute

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When creating a bug report, include as many details as possible:

- **Clear title and description**
- **Steps to reproduce** the behavior
- **Expected behavior**
- **Actual behavior**
- **Environment details** (Rust version, Wrangler version, etc.)
- **Screenshots** (if applicable)

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, include:

- **Clear title and description**
- **Use case** - why is this enhancement useful?
- **Proposed solution** (if you have one)
- **Alternatives considered** (if any)

### Pull Requests

1. **Fork the repository**
2. **Create a feature branch** (`git checkout -b feature/amazing-feature`)
3. **Make your changes**
4. **Follow the coding standards** (see below)
5. **Add tests** for new functionality
6. **Update documentation** as needed
7. **Commit your changes** using [Conventional Commits](https://www.conventionalcommits.org/)
8. **Push to your branch** (`git push origin feature/amazing-feature`)
9. **Open a Pull Request**

## Development Setup

1. Clone the repository:

   ```bash
   git clone https://github.com/drengskapur/courtlistener-worker.git
   cd courtlistener-worker
   ```

2. Install dependencies:

   ```bash
   rustup target add wasm32-unknown-unknown
   npm install -g wrangler
   ```

3. Set up environment:

   ```bash
   cp .env.example .env
   # Edit .env with your API token
   ```

4. Run tests:

   ```bash
   cargo test
   ```

5. Run locally:

   ```bash
   npx wrangler dev
   ```

## Coding Standards

### Rust Code Style

- Follow Rust standard formatting (`cargo fmt`)
- Run clippy before submitting (`cargo clippy --all-targets --all-features -- -D warnings`)
- Ensure all tests pass (`cargo test --all-features`)
- Use meaningful variable and function names
- Add documentation comments for public APIs

### Commit Messages

We use [Conventional Commits](https://www.conventionalcommits.org/):

- `feat:` A new feature
- `fix:` A bug fix
- `docs:` Documentation only changes
- `style:` Code style changes (formatting, etc.)
- `refactor:` Code refactoring
- `test:` Adding or updating tests
- `chore:` Maintenance tasks

Example:

```plaintext
feat(api): add support for new endpoint
```

### Testing

- Write tests for new features
- Ensure existing tests still pass
- Test edge cases and error conditions
- Update integration tests if API changes

### Documentation

- Update README.md if user-facing features change
- Add doc comments for public functions and types
- Update API documentation if endpoints change

## Review Process

1. All pull requests require at least one review
2. Maintainers will review your PR and may request changes
3. Once approved, a maintainer will merge your PR
4. Thank you for contributing! 🎉

## Questions?

Feel free to open an issue with the `question` label if you have any questions about contributing.
