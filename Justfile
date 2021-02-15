export OBS_HOST := if os() == "macos" { "host.docker.internal" } else { "127.0.0.1" }

# list available recipes
default:
    @just --list

# run integration tests with coverage (using Docker)
coverage:
    docker run --rm -it --security-opt seccomp=unconfined --network host -v $PWD:/volume -v $HOME/.cargo/registry:/usr/local/cargo/registry xd009642/tarpaulin cargo tarpaulin --out Html --out Lcov --all-features

# run integration tests with coverage (using Vagrant)
coverage-vagrant:
    vagrant up
    vagrant ssh -c 'cd /vagrant; false; while [ "$?" -eq 1 ]; do cargo tarpaulin --all-features --no-run; done'
    vagrant ssh -c 'cd /vagrant && cargo tarpaulin --out Html --out Lcov --all-features'

# upload coverage to https://codecov.io
upload-coverage:
    @# {{env_var("CODECOV_TOKEN")}}
    just coverage-vagrant
    bash -c "bash <(curl -s https://codecov.io/bash)"
