# Code Coverage Setup

This project uses **tarpaulin** for code coverage measurement and reporting.

## Local Coverage

Run coverage locally with:

```bash
./coverage.sh
```

This will:
- Install tarpaulin if not already present
- Generate an HTML coverage report
- Save results in `./coverage/` directory
- Open the report in your browser

Or run tarpaulin directly:

```bash
cargo tarpaulin --workspace --out Html --output-dir coverage
```

## Configuration

Coverage is configured in [.tarpaulin.toml](.tarpaulin.toml) with:
- **Minimum coverage**: 50%
- **Timeout**: 300 seconds per test
- **Output formats**: XML, LCOV, HTML
- **Workspace mode**: Enabled (covers all workspace members)
- **Parallel execution**: 4 jobs

## CI/CD Integration

- **GitHub Actions**: Automatically runs on push to `main`/`develop` and PRs
- **Codecov**: Coverage reports uploaded to codecov.io
- **Workflow file**: [.github/workflows/coverage.yml](.github/workflows/coverage.yml)

## Output Formats

- **HTML**: `./coverage/index.html` - Interactive report
- **XML (Cobertura)**: `./cobertura.xml` - CI/CD integration
- **LCOV**: `./lcov.info` - Editor/IDE integration

## Viewing Results

After running `./coverage.sh`, open the report:

```bash
open coverage/index.html  # macOS
xdg-open coverage/index.html  # Linux
start coverage/index.html  # Windows
```

## Excluded from Coverage

- `tests/` - Test code
- `examples/` - Example code
