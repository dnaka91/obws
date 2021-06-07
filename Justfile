# list available recipes
default:
    @just --list --unsorted

# run integration tests with coverage
coverage:
    cargo install grcov
    rustup component add --toolchain nightly llvm-tools-preview

    rm -rf *.profraw ./target/debug/coverage lcov.info

    RUSTFLAGS="-Zinstrument-coverage" LLVM_PROFILE_FILE="coverage-%p-%m.profraw" cargo +nightly test --all-features
    rustup run nightly grcov . -s . --binary-path ./target/debug/ -t html --branch --ignore-not-existing -o ./target/debug/coverage
    rustup run nightly grcov . -s . --binary-path ./target/debug/ -t lcov --branch --ignore-not-existing --ignore "/*" -o lcov.info

    rm -f *.profraw

# upload coverage to https://codecov.io
upload-coverage: get-codecov
    @# {{env_var("CODECOV_TOKEN")}}
    just coverage
    bash -c "export CODECOV_TOKEN=$CODECOV_TOKEN && ./codecov -f lcov.info"

get-codecov:
    #!/usr/bin/env bash
    set -euo pipefail
    curl -s https://codecov.io/bash > codecov;
    VERSION=$(grep -o 'VERSION=\"[0-9\.]*\"' codecov | cut -d'"' -f2);
    for i in 1 256 512
    do
        shasum -a $i -c --ignore-missing <(curl -s "https://raw.githubusercontent.com/codecov/codecov-bash/${VERSION}/SHA${i}SUM") ||
        shasum -a $i -c <(curl -s "https://raw.githubusercontent.com/codecov/codecov-bash/${VERSION}/SHA${i}SUM")
    done
    chmod +x codecov
