set dotenv-load := true

nightly := "nightly-2021-10-23"

# list available recipes
default:
    @just --list --unsorted

# run integration tests with coverage
coverage:
    cargo install grcov
    rustup toolchain install {{nightly}} --component llvm-tools-preview

    rm -rf *.profraw ./target/debug/coverage

    RUSTFLAGS="-Zinstrument-coverage -Clink-dead-code" LLVM_PROFILE_FILE="coverage-%p-%m.profraw" cargo +{{nightly}} test --all-features
    rustup run {{nightly}} grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage

    rm -f *.profraw

# upload coverage to GitHub Pages
upload-coverage: coverage
    git checkout gh-pages
    rm -rf badges examples src tests coverage.json index.html
    cp -R target/debug/coverage/ .
    git add -A badges examples src tests coverage.json index.html
    git commit -m "Coverage for $(git rev-parse --short main)"
    git push
    git checkout main
