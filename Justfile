# list available recipes
default:
    @just --list --unsorted

# run integration tests with coverage
coverage:
    cargo install grcov --git https://github.com/mozilla/grcov.git --rev c7a9b20d246a0cda812db509f206b38b3116cba4
    rustup component add --toolchain nightly llvm-tools-preview

    rm -rf *.profraw ./target/debug/coverage

    RUSTFLAGS="-Zinstrument-coverage -Clink-dead-code" LLVM_PROFILE_FILE="coverage-%p-%m.profraw" cargo +nightly test --all-features
    rustup run nightly grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage

    rm -f *.profraw

# upload coverage to GitHub Pages
upload-coverage: coverage
    git checkout gh-pages
    rm -rf badges examples src tests coverage.json index.html
    cp -R target/debug/coverage/ .
    git add -A badges examples src tests coverage.json index.html
    git commit -m "Coverage for $(git rev-parse --short main)"
    # git push
    git checkout main
