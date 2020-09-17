# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### 0.4.1 (2020-09-17)


### Bug Fixes

* **ci:** fix clippy errors and warnings ([baf97ab](https://github.com/maidsafe/sn_fake_clock/commit/baf97ab609530e5324f9f62f90f1e86987b73d23))

## [0.4.0]
- Change crate name to sn_fake_clock
- Replaced implementations of common traits with derive
- Manually impl Hash
- Added derive Hash to FakeClock

## [0.3.0]
- Use rust 1.22.1 stable / 2017-12-02 nightly
- rustfmt 0.9.0 and clippy-0.0.175

## [0.2.0]
- Use rust 1.19 stable / 2017-07-20 nightly
- rustfmt 0.9.0 and clippy-0.0.144
- Replace -Zno-trans with cargo check
- Make appveyor script using fixed version of stable

## [0.1.0]
- Initial implementation
