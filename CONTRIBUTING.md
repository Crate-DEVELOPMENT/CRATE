# Contributing to Crate Base

First off, thank you for considering contributing to Crate Base! It's people like you that make Crate Base such a great tool.

## Code of Conduct

This project and everyone participating in it is governed by our [Code of Conduct](CODE_OF_CONDUCT.md). By participating, you are expected to uphold this code.

## How Can I Contribute?

### Reporting Bugs

Before creating bug reports, please check the issue list as you might find out that you don't need to create one. When you are creating a bug report, please include as many details as possible:

* Use a clear and descriptive title
* Describe the exact steps which reproduce the problem
* Provide specific examples to demonstrate the steps
* Describe the behavior you observed after following the steps
* Explain which behavior you expected to see instead and why
* Include screenshots if possible

### Suggesting Enhancements

Enhancement suggestions are tracked as GitHub issues. When creating an enhancement suggestion, please include:

* A clear and descriptive title
* A detailed description of the proposed functionality
* Explain why this enhancement would be useful
* List any similar features in other projects if applicable

### Pull Requests

1. Fork the repo and create your branch from `main`
2. If you've added code that should be tested, add tests
3. If you've changed APIs, update the documentation
4. Ensure the test suite passes
5. Make sure your code lints

## Development Process

1. Clone the repository
```bash
git clone https://github.com/crate-lol/crate-base.git
cd crate-base
```

2. Create a new branch
```bash
git checkout -b feature/your-feature-name
```

3. Make your changes and commit them using conventional commits
```bash
git commit -m "feat: Add new feature"
git commit -m "fix: Address issue #123"
```

### Conventional Commits

We follow the conventional commits specification. Each commit message should be structured as follows:

```
<type>[optional scope]: <description>

[optional body]

[optional footer(s)]
```

Types include:
- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, missing semi colons, etc)
- `refactor`: Code refactoring
- `test`: Adding missing tests
- `chore`: Maintenance tasks

### Development Setup

1. Install dependencies
```bash
# Frontend
cd frontend
npm install

# Rust
cd rust
cargo build

# Go
cd services
go mod download
```

2. Set up pre-commit hooks
```bash
npm install husky --save-dev
```

### Testing

```bash
# Run all tests
npm test

# Run specific test file
npm test -- path/to/test-file.test.ts

# Run Rust tests
cargo test

# Run Go tests
go test ./...
```

### Code Style

- Frontend: Follow the ESLint configuration
- Rust: Follow rustfmt conventions
- Go: Follow gofmt standards

## Documentation

- Keep README.md updated
- Update API documentation for any interface changes
- Add comments for complex logic
- Update TypeScript types when applicable

## Review Process

1. Create a Pull Request (PR)
2. Fill in the PR template
3. Wait for review from maintainers
4. Address any requested changes
5. Once approved, your PR will be merged

## Community

- Follow us on [Twitter](https://twitter.com/crate_ai)
- Email us at support@crate.lol

## Recognition

Contributors will be recognized in:
- The project's README
- Our official documentation
- Our contributors page

## Questions?

Don't hesitate to create a GitHub Discussion or reach out via email.

Thank you for contributing to Crate Base! 🚀
