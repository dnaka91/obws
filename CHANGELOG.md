<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased] - ReleaseDate

### Changed

- The `open_projector`] request now accepts a struct for geometry to define the projector window
  instead of a simple string. This allows to set the geometry without the need to know about the Qt
  geometry works and how it has to be encoded.

### Fixed

- The version requirement for OBS Studio was reduced from `26.1.2` to `26.1.0` as the `x.x.2` was a
  MacOS specific fix version and generally fix versions don't break API compatibility.

## [0.6.0] - 2021-03-16

### Added

- If the connection to obs-websocket is lost, a `Event::ServerStopped` event is sent to signal the
  disconnect. Thanks @715209 !

### Changed

- **BREAKING CHANGE:** `Client::events` now returns a `Result` and fails with an error when trying
  to get a new event stream while disconnected from obs-websocket.
- **BREAKING CHANGE:** During `Client::connect` the version of OBS Studio and obs-websocket is now
  checked and the connection will fail if the versions are too old or too new.
- The event streams returned from `Client::events` are properly being closed when the connection to
  obs-websocket is disconnected in any way.

### Fixed

- Requests were never finishing if the connection was lost in the meantime. They're now properly
  cancelled when a disconnect happens.
- The `TransitionVideoEnd` event could not be deserialized as the `from_scene` field was wrongly
  a required field and was therefore changed to be optional. Thanks @715209 !

## [0.5.0] - 2021-03-07

### Changed

- **BREAKING CHANGE:** Upgraded to obs-websocket `4.9.0`, adding all the new events and requests.
  Some existing ones have changes as well.
- Dependencies updated to the latest version, including the just released `tokio-tungstenite` with
  support for `rustls` as TLS backend.
- The `tls` feature now uses `rustls` instead of `native-tls` as backend.

### Fixed

- obs-websocket sends a non-JSON message when shutting down that is now properly handled instead of
  failing. Thanks @715209 !

## [0.4.0] - 2021-01-22

### Added

- The client can now be shut down with the `disconnect` function and will do so automatically on
  drop. It is still recommended to call this manually as it allows to wait for the client to fully
  close all the internal machinery.

### Changed

- Events are opt-in with the `events` feature flag to reduce final binary size when events aren't
  used.

### Fixed

- All fields of `ConnectConfig` have been made public to actually allow setting them.

## [0.3.0] - 2021-01-11

### Added

- Github actions added to run automated tests and lints on each commit.
- Unit tests for all custom (de)serializers ensuring the right in-/output.
- More integration tests, covering almost all requests now.
- Added `.editorconfig` file to allow consistent indenting and other common editor settings.
- Added a `Justfile` (similar to a `Makefile`) that allows to run common tasks and especially code
  coverage conveniently.
- A new `connect_with_config` function that allows to customize the client behavior.
- Support for TLS connections.

### Changed

- Update readme to show the current code coverage.
- Upgraded to `tokio` **1.0** and all related dependencies.

### Fixed

- Corrected a few links in the API docs.
- A required tokio feature was missing as the dev dependencies added it.

## [0.2.0]

### Added

- Most request types implement `Default` for easier request creation.
- Many integration tests.
- Basic usage details in the docs and readme.

### Changed

- Parse into more concrete types where possible. For example durations and timestamps are
  represented as types from the `chrono` crate instead of strings and integers now.
- Errors are specific now, using `thiserror` instead of `anyhow` allowing to match against the error
  and find out what exactly went wrong on a type level.

### Fixed

- Various small fixes in request and response types that were found while creating the integration
  tests.
- Some links in the API docs were broken, pointing to private items.

## [0.1.0]

### Added

- Initial release.

[Unreleased]: https://github.com/dnaka91/obws/compare/v0.6.0...HEAD
[0.6.0]: https://github.com/dnaka91/obws/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/dnaka91/obws/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/dnaka91/obws/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/dnaka91/obws/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dnaka91/obws/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dnaka91/obws/releases/tag/v0.1.0
