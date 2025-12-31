# Contributing

Thank you for considering contributing to DIVW.

## Development Setup

1. Fork and clone the repository
2. Install dependencies:
   ```bash
   cargo build
   cd sdk && npm install
   ```
3. Run tests:
   ```bash
   cargo test
   npm test
   ```

## Pull Request Process

1. Create a branch from `develop`
2. Make changes with clear commits
3. Add tests for new functionality
4. Submit a PR with description

## Commit Convention

We use conventional commits:
- `feat:` New features
- `fix:` Bug fixes
- `docs:` Documentation
- `test:` Tests
- `refactor:` Refactoring
- `chore:` Maintenance
