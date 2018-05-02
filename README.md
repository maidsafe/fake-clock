# fake_clock

**Maintainer:** Andreas Fackler (andreas.fackler@maidsafe.net)

|Crate|Documentation|Linux/OS X|Windows|Issues|
|:---:|:-----------:|:--------:|:-----:|:----:|
|[![](http://meritbadge.herokuapp.com/fake_clock)](https://crates.io/crates/fake_clock)|[![Documentation](https://docs.rs/fake_clock/badge.svg)](https://docs.rs/fake_clock)|[![Build Status](https://travis-ci.org/maidsafe/fake_clock.svg?branch=master)](https://travis-ci.org/maidsafe/fake_clock)|[![Build status](https://ci.appveyor.com/api/projects/status/oq5s0j82ykvb52du/branch/master?svg=true)](https://ci.appveyor.com/project/MaidSafe-QA/fake-clock/branch/master)|[![Stories in Ready](https://badge.waffle.io/maidsafe/fake_clock.png?label=ready&title=Ready)](https://waffle.io/maidsafe/fake_clock)|

| [MaidSafe website](https://maidsafe.net) | [SAFE Dev Forum](https://forum.safedev.org) | [SAFE Network Forum](https://safenetforum.org) |
|:----------------------------------------:|:-------------------------------------------:|:----------------------------------------------:|

## Overview

This crate supplies a `FakeClock` struct, which mimics the interface of `std::time::Instant` and enables full control over the flow of time as perceived by the code during testing.

## License

Licensed under the General Public License (GPL), version 3 ([LICENSE](LICENSE) http://www.gnu.org/licenses/gpl-3.0.en.html).
