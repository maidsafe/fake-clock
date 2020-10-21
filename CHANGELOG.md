# Changelog

All notable changes to this project will be documented in this file. See [standard-version](https://github.com/conventional-changelog/standard-version) for commit guidelines.

### [0.4.6](https://github.com/maidsafe/sn_fake_clock/compare/v0.4.5...v0.4.6) (2020-10-21)


### Features

* add checked_duration_since, saturating_duration_since ([cfeca69](https://github.com/maidsafe/sn_fake_clock/commit/cfeca699c9c88d3df04ebaaba5d31039e0cd7324)), closes [#26](https://github.com/maidsafe/sn_fake_clock/issues/26)

### [0.4.5](https://github.com/maidsafe/sn_fake_clock/compare/v0.4.4...v0.4.5) (2020-10-21)


### Features

* add checked_add, checked_sub ([9e438d4](https://github.com/maidsafe/sn_fake_clock/commit/9e438d4fade6a4bd61c39f777ad77bf937ba9f20)), closes [#23](https://github.com/maidsafe/sn_fake_clock/issues/23)

### [0.4.4](https://github.com/maidsafe/sn_fake_clock/compare/v0.4.3...v0.4.4) (2020-10-21)

### [0.4.3](https://github.com/maidsafe/sn_fake_clock/compare/v0.4.2...v0.4.3) (2020-10-09)

### [0.4.2](https://github.com/maidsafe/sn_fake_clock/compare/v0.4.1...v0.4.2) (2020-09-17)

### [0.4.1](https://github.com/maidsafe/sn_fake_clock/compare/0.4.0...v0.4.1) (2020-09-17)

### Bug Fixes

* **ci:** fix clippy errors and warnings ([baf97ab](https://github.com/maidsafe/sn_fake_clock/commit/baf97ab609530e5324f9f62f90f1e86987b73d23))

### [0.4.0](https://github.com/maidsafe/sn_fake_clock/compare/0.3.0...v0.4.0) (2020-09-01)

* Change crate name to sn_fake_clock
* Replaced implementations of common traits with derive
* Manually impl Hash
* Added derive Hash to FakeClock

### [0.3.0](https://github.com/maidsafe/sn_fake_clock/compare/0.2.0...v0.3.0) (2018-01-05)
* Use rust 1.22.1 stable / 2017-12-02 nightly
* rustfmt 0.9.0 and clippy-0.0.175

### [0.2.0](https://github.com/maidsafe/sn_fake_clock/compare/0.1.0...v0.2.0) (2017-07-25)
* Use rust 1.19 stable / 2017-07-20 nightly
* rustfmt 0.9.0 and clippy-0.0.144
* Replace -Zno-trans with cargo check
* Make appveyor script using fixed version of stable

### [0.1.0] (2017-04-05)
* Initial implementation
