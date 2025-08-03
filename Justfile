_default:
    @just --list --unsorted

# format all Rust source code
fmt:
    cargo +nightly fmt --all

# run unit and integration tests
test:
    cargo nextest run --all-features

# run integration tests with coverage
coverage:
    cargo llvm-cov test --all-features
    cargo llvm-cov report --html
    cargo llvm-cov report --json --summary-only | \
        jq -c '.data[0].totals.lines.percent | { \
            schemaVersion: 1, \
            label: "coverage", \
            message: "\(.|round)%", \
            color: (if . < 70 then "red" elif . < 80 then "yellow" else "green" end) \
        }' > `cargo metadata | jq -r '.target_directory'`/llvm-cov/html/coverage.json
