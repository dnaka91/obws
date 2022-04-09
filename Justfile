set dotenv-load := true

_default:
    @just --list --unsorted

# run unit and integration tests
test:
    cargo test
    cargo test --all-features --test integration -- --test-threads 1

# run integration tests with coverage
coverage:
    cargo install cargo-llvm-cov
    rustup component add llvm-tools-preview

    cargo llvm-cov --remap-path-prefix --html --all-features -- --test-threads 1
    cargo llvm-cov --remap-path-prefix --no-run --json --summary-only | \
        jq -c '.data[0].totals.lines.percent | { \
            schemaVersion: 1, \
            label: "coverage", \
            message: "\(.|round)%", \
            color: (if . < 70 then "red" elif . < 80 then "yellow" else "green" end) \
        }' > target/llvm-cov/html/coverage.json

# upload coverage to GitHub Pages
upload-coverage: coverage
    git checkout gh-pages
    rm -rf coverage coverage.json index.html style.css
    cp -R target/llvm-cov/html/ .
    git add -A coverage coverage.json index.html style.css
    git commit -m "Coverage for $(git rev-parse --short v5-api)"
    git push
    git checkout v5-api
