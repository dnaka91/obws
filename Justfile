# list available recipes
default:
    @just --list --unsorted

# run integration tests with coverage
coverage:
    cargo install grcov
    rustup component add --toolchain nightly llvm-tools-preview

    rm -f *.profraw ./target/debug/coverage lcov.info

    RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="coverage-%p-%m.profraw" cargo +nightly test --all-features
    rustup run nightly grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage
    rustup run nightly grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info

    rm -f *.profraw

# upload coverage to https://codecov.io
upload-coverage:
    @# {{env_var("CODECOV_TOKEN")}}
    just coverage
    bash -c "bash <(curl -s https://codecov.io/bash) -f lcov.info"
