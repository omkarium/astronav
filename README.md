## ASTRONAV: A rust library featuring algorithms for positional astronomy
[![crate][crate-image]][crate-link]
![MIT licensed][license-image]
![Rust Version][rustc-image]
[![Downloads][downloads-image]][crate-link]
![Category][category-image]

## Purpose
To create a pure rust implementation using popular positional astronomy algorithms

### Current features
- Methods to get the `Altitude` and `Azimuth` of Stars using `RA (Right Ascension)` and `Dec (Declination)` values.
- Methods to get the Sun's Position, Sun Rise, Sun Set and other related things of the Sun using the Structs available in `coords::noaa_sun` and `coords::sun` modules.
- The module `coords::noaa_sun` is available only as a feature flag `--features "noaa-sun"`.
- Time and date functions to retrieve the below, available in the `time` module
  - Julian Day Number
  - Julian Time
  - Greenwich Mean Sidereal Time
  - Local Mean Sidereal Time
  - Day of the Year etc

### Notice regarding the Sun related modules: 
The Sun related modules `coords::noaa_sun` and `coords::sun`, cannot be fully trusted (yet) for a higher degree of accuracy. The calculations are made using certain generally available algorithms from certain popular Almanacs such as the one written by Meesus, and the algorithms published by NOAA. For example, when you take the SunRise, the SunSet and the Noon time using the Structs made available, you would typically see up to 2 mins of variation when compared with what is shown in popular apps like Stellarium. The same applies for angles such as Azimuth, Zenith, Declination, Hour Angle etc. However, we must keep in mind that there is no single source of truth available online to these things and getting precise results are extremely difficult than we think.

[//]: # (badges)

[crate-image]: https://img.shields.io/crates/v/astronav.svg
[crate-link]: https://crates.io/crates/astronav
[license-image]: https://img.shields.io/badge/License-MIT_or_Apache_2.0-yellow.svg
[rustc-image]: https://img.shields.io/badge/rustc-1.69+-blue.svg
[downloads-image]: https://img.shields.io/crates/d/astronav.svg
[category-image]: https://img.shields.io/badge/category-Astronomy_Algorithms-darkred.svg
