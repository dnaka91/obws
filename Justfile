export OBS_HOST := if os() == "macos" { "host.docker.internal" } else { "127.0.0.1" }

# list available recipes
default:
    @just --list

# run integration tests with coverage
coverage:
    docker run --rm -it --security-opt seccomp=unconfined --network host -v $PWD:/volume -v $HOME/.cargo/registry:/usr/local/cargo/registry xd009642/tarpaulin cargo tarpaulin --out Html --out Lcov --all-features

# upload coverage to https://codecov.io
upload-coverage:
    @# {{env_var("CODECOV_TOKEN")}}
    just coverage
    bash -c "bash <(curl -s https://codecov.io/bash)"
