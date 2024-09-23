<!-- markdownlint-disable MD024 -->

# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

<!-- next-header -->
## [Unreleased] - ReleaseDate

## [0.13.0] - 2024-09-23

### Changed

- Updated tokio-tungstenite to the latest `v0.24`.

### Fixed

- **BREAKING CHANGE:** Deserialize the UUIDs of scene list responses, effectively exchanging the `name` field for an `id`. This was an oversight when introducing the UUIDs.

## [0.12.0] - 2024-05-04

### Added

- New features from obs-websocket [v5.3.0](https://github.com/obsproject/obs-websocket/releases/tag/5.3.0)
  - New `set_record_directory` command, that allows to modify the output directory for recordings.
  - New `NotReady` status code, that signals the obs-websocket server is not ready yet to accept any commands.
- New features from obs-websocket [v5.4.0](https://github.com/obsproject/obs-websocket/releases/tag/5.4.0)
  - **BREAKING CHANGE:** Support for UUIDs to identify sources, inputs, scenes and transitions. This includes various breaking changes around those fields to make it impossible to misuse them in requests (basically using an enum to only use the name _or_ UUID in a request but not both).
  - New `list_kinds` command for filters.
  - New `source` command for scene items.
  - New `InputSettingsChanged` and `SourceFilterSettingsChanged` event.
- **BREAKING CHANGE:** Extend the connection config with a `connection_timeout` setting that allows to cancel the connection attempt after a set duration. The `Client::connect` method now defaults to a _30 second_ timeout.

### Removed

- **BREAKING CHANGE:** Removed the `Error::NoPassword` variant, which was a relic from the previous v4.x API of obs-websocket. The new API doesn't provide any way to tell whether the password is the cause of an authentication error. However, the `WebSocketCloseCode::AuthenticationFailed` is the closest to identifying this error.

## [0.11.5] - 2023-09-04

### Changed

- Explicitly unsubscribe from all events when the `events` feature isn't enabled. By default, `obs-websocket` sends all possible events, but those are not needed without the events feature flag in obws.

### Fixed

- Any received event from `obs-websocket` was wrongly reported as error level log while the `events` feature flag wasn't enabled in `obws`. These are now handled properly and logged as trace level log.

## [0.11.4] - 2023-08-23

### Changed

- Unpin `serde` and update to `1.0.185`, which resolved the issue in `serde_derive` <https://github.com/serde-rs/serde/issues/2538>. Also, raised the version of `time` as it pinned the serde version as well.

## [0.11.3] - 2023-08-19

### Changed

- Updated several dependencies, most notably `tokio-tungstenite` to `0.20`.
- Pin `serde` to `1.0.171`, as newer versions use a pre-compiled binary for the derive macros.

## [0.11.2] - 2023-06-22

### Fixed

- Some of the bitflag values like `FontFlags`, `Alignment` and `EventSubscription` were not always serialized as numbers but their string form, causing errors in the respective request or response.

## [0.11.1] - 2023-05-22

### Added

- All client methods now have a doc alias with the original name from the [`obs-websocket` spec](https://github.com/obsproject/obs-websocket/blob/master/docs/generated/protocol.md).
  - The search will point to the right function when putting the original name in the search bar on docs.rs (or in locally generated docs).
  - [Latest `rust-analyzer`](https://rust-analyzer.github.io/thisweek/2023/05/22/changelog-182.html) can use this information in the method completion menu.

## [0.11.0] - 2023-05-18

### Added

- New features from obs-websocket v5.1.0
  - New `ScreenshotSaved` event, that is triggered by hotkeys in the OBS UI.
  - New variants for the `OutputState` enum, that signal reconnecting and reconnected states.

### Changed

- **BREAKING CHANGE:** Due to the update of `bitflags` the list of derived traits changed.
- Update all dependencies to their latest version, most notably `base64`, `bitflags` and
  `tokio-tungstenite`.
- The MSRV is now **1.68**.

## [0.10.0] - 2022-11-14

### Changed

- **BREAKING CHANGE:** Update to the new obs-websocket v5 API, which is a complete rewrite, so
  all APIs of this crate had to change as well. Expect all types and function calls to be slightly
  different.
  - In addition, I didn't agree with the new naming scheme in obs-websocket, as it includes a great
    amount of naming repetition. Therefore, most function calls and data structures/fields are
    named different to be more concise.
  - Thank you to [@715209](https://github.com/715209) and [@Elinvynia](https://github.com/Elinvynia)
    for testing out these changes pre-release ❤️. Your ideas and bug reports helped a lot!
- All response and event data structures now implement the recommended common traits, where
  possible. In addition, `serde::Serialize` and `serde::Deserialize` is implemented for all of
  them. That means, they can now be constructed easier, and used in more places, for example, as a
  `HashMap` key.

## [0.9.1] - 2022-02-25

### Fixed

- Deserialization of Freetype2 and GDI+ failed in some situations, especially when many fields were
  not set to a custom value (as OBS omits default values).
  Thank you [@peddermaster2](https://github.com/peddermaster2)!

## [0.9.0] - 2021-12-20

This is going to be the last release before the upcoming **obs-websocket v5.0** release. Support
is currently in progress on the `v5-api` branch and the release is expected to be soon.

### Added

- Several new types are available at the `requests::custom` module, that can be used for the generic
  `set_source_settings` and `set_transition_settings` requests to provide typed versions for common
  sources and transitions out of the box.
  - **Note:** These typed versions are not covered by automated tests yet, so careful testing is
    advised.

### Changed

- **BREAKING CHANGE:** Updated many dependencies, including `tokio-tungstenite` which included some
  changes to the TLS feature.
- **BREAKING CHANGE:** Replace the `chrono` crate with `time` to avoid potential security issues.
  This crate only uses the `Duration` type, but the dependency was still switched in case of future
  issues.
- Reduce size of the crate bundle by excluding unnecessary files, making downloads from
  <https://crates.io> faster.
- Improved on several spelling errors throughout the API docs.

## [0.8.0] - 2021-06-14

### Added

- Dependabot is now used to create new PRs for version updates in cargo and GitHub Actions.

### Changed

- **BREAKING CHANGE:** Upgraded to obs-websocket `4.9.1`, adding a few new events and requests.
  Unfortunately although this looks like a patch release of obs-websocket, it actually included
  some breaking changes.
- **BREAKING CHANGE:** As with the upgrade to `4.9.1`, the new required minimum version for OBS is
  `27.0.0` and for obs-websocket is `4.9.1`. Versions requirements have been further relaxed for OBS
  so that even a `28.0.0` release will work with the current version of obws.
- Code coverage data is now directly hosted on GitHub Pages at <https://dnaka91.github.io/obws>,
  instead of using codecov.io. This has been done to reduce the amount of dependencies on external
  services and reduce security risks due to recent codecov incidents.
- Several dependencies have been updated to their latest version.

### Fixed

- Internal renaming and fixes have been done, mostly for naming conventions or other clippy lints.
  No logical fixes related to obs-websocket though.

## [0.7.0] - 2021-03-27

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

<!-- next-url -->
[Unreleased]: https://github.com/dnaka91/obws/compare/v0.13.0...HEAD
[0.13.0]: https://github.com/dnaka91/obws/compare/v0.12.0...v0.13.0
[0.12.0]: https://github.com/dnaka91/obws/compare/v0.11.5...v0.12.0
[0.11.5]: https://github.com/dnaka91/obws/compare/v0.11.4...v0.11.5
[0.11.4]: https://github.com/dnaka91/obws/compare/v0.11.3...v0.11.4
[0.11.3]: https://github.com/dnaka91/obws/compare/v0.11.2...v0.11.3
[0.11.2]: https://github.com/dnaka91/obws/compare/v0.11.1...v0.11.2
[0.11.1]: https://github.com/dnaka91/obws/compare/v0.11.0...v0.11.1
[0.11.0]: https://github.com/dnaka91/obws/compare/v0.10.0...v0.11.0
[0.10.0]: https://github.com/dnaka91/obws/compare/v0.9.1...v0.10.0
[0.9.1]: https://github.com/dnaka91/obws/compare/v0.9.0...v0.9.1
[0.9.0]: https://github.com/dnaka91/obws/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/dnaka91/obws/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/dnaka91/obws/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/dnaka91/obws/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/dnaka91/obws/compare/v0.4.0...v0.5.0
[0.4.0]: https://github.com/dnaka91/obws/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/dnaka91/obws/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/dnaka91/obws/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/dnaka91/obws/releases/tag/v0.1.0
