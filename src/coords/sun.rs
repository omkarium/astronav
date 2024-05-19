//! Track the Sun positional coordinates and time
// Copyright (c) 2024 Venkatesh Omkaram

// Sunrise/sunset calculation in squirrel, hugo@electricimp.com
// Based on http://williams.best.vwh.net/sunrise_sunset_algorithm.htm

use std::f32::consts::PI;

use crate::time::day_of_year;

const ZENITH: f32 = 90.833;

// An enum only related to the SunRiseAndSet Struct
#[derive(Debug)]
pub enum SunMood {
    NeverRise,
    NeverSet,
    Rise,
    Set,
}

/// A Struct to find the Sun Rise, Sun Set and other items about the Sun
/// 
/// * Note: Checkout similar feature but using NOAA algorithms in `noaa_sun` module
/// 
/// # Example 1 - Sun Rise
/// Calculating the Sun Positional Properties for the Rise on May 16th 2024, New York
/// ```
/// use astronav::coords::{sun::SunRiseAndSet, hours_to_hms};
/// 
/// let sun_new_york = SunRiseAndSet {
///     doy: 137,
///     long: -74.0060,
///     lat: 40.7128,
///     timezone: -4.0,
/// };
///
/// let sma = sun_new_york.sunrise_mean_anomaly();
/// let stl = sun_new_york.sunrise_true_long_in_deg();
/// let ra = sun_new_york.sunrise_ra_in_hours();
/// let dec = sun_new_york.sunrise_declination();
/// let lha = sun_new_york.sunrise_local_ha_in_deg();
///
/// let rising = sun_new_york.sunrise_time();
/// 
/// assert_eq!(132.18721, sma);
/// assert_eq!(56.220978, stl);
/// assert_eq!(3.5939937, ra);
/// assert_eq!(19.309036, dec);
/// assert_eq!(16.748438, lha.unwrap());
/// assert_eq!(5.6219597, *rising.as_ref().unwrap());
/// assert_eq!("5:37:19.05487".to_owned(), hours_to_hms(rising.unwrap()));
/// ```
/// By this we found that the sun rise occurred at 5:37:19.05 AM in New York on the given day
/// 
/// # Example 2 - Sun Set
/// Calculating the Sun Positional Properties for Set on May 16th 2024, New York
/// ```
/// use astronav::coords::{sun::SunRiseAndSet, hours_to_hms};
/// 
/// let sun_new_york = SunRiseAndSet {
///     doy: 137,
///     long: -74.0060,
///     lat: 40.7128,
///     timezone: -4.0,
/// };
///
/// let sma = sun_new_york.sunset_mean_anomaly();
/// let stl = sun_new_york.sunset_true_long_in_deg();
/// let ra = sun_new_york.sunset_ra_in_hours();
/// let dec = sun_new_york.sunset_declination();
/// let lha = sun_new_york.sunset_local_ha_in_deg();
///
/// let setting = sun_new_york.sunset_time();
/// 
/// assert_eq!(132.68001, sma);
/// assert_eq!(56.702637, stl);
/// assert_eq!(3.6270912, ra);
/// assert_eq!(19.42125, dec);
/// assert_eq!(7.25926, lha.unwrap());
/// assert_eq!(20.133024, *setting.as_ref().unwrap());
/// assert_eq!("20:7:58.887177".to_owned(), hours_to_hms(setting.unwrap()));
/// ```
/// By this we found that the sun set occurred at 20:7:58.88 PM in New York on the given day
/// 
/// # Example 3 - Sun Set - Using new() and setter methods
/// Calculating the Sun Properties for Set on May 16th 2024, New York
/// ```
/// use astronav::coords::{sun::SunRiseAndSet, hours_to_hms};
/// 
/// let sun_new_york = SunRiseAndSet::new()
///                     .date(2024, 05, 16)
///                     .long(-74.0060)
///                     .lat(40.7128)
///                     .timezone(-4.0);
///
/// let sma = sun_new_york.sunset_mean_anomaly();
/// let stl = sun_new_york.sunset_true_long_in_deg();
/// let ra = sun_new_york.sunset_ra_in_hours();
/// let dec = sun_new_york.sunset_declination();
/// let lha = sun_new_york.sunset_local_ha_in_deg();
///
/// let setting = sun_new_york.sunset_time();
/// 
/// assert_eq!(132.68001, sma);
/// assert_eq!(56.702637, stl);
/// assert_eq!(3.6270912, ra);
/// assert_eq!(19.42125, dec);
/// assert_eq!(7.25926, lha.unwrap());
/// assert_eq!(20.133024, *setting.as_ref().unwrap());
/// assert_eq!("20:7:58.887177".to_owned(), hours_to_hms(setting.unwrap()));
/// ```
/// By this we found that the sun set occurred at 20:7:58.88 PM in New York on the given day
#[derive(Debug, Clone, Default)]
pub struct SunRiseAndSet {
    /// Day of the year (Example: May 16th, 2024 is day 137)
    pub doy: u16,
    /// Longitude of the point of interest in degrees (+ east, - west)
    pub long: f32,
    /// Latitude of the point of interest in degrees (+ north, - south)
    pub lat: f32,
    /// Timezone of the point of interest in hours (+ east, - west)
    pub timezone: f32,
}

impl SunRiseAndSet {
    /// Provides a default implementation for the value in the struct
    pub fn new() -> Self {
        Self::default()
    }

    pub fn date(self, year: u16, month: u8, day: u8) -> Self {
        let doy = day_of_year(year, month, day);
        Self { doy, ..self }
    }

    pub fn long(self, long: f32) -> Self {
        Self { long, ..self }
    }

    pub fn lat(self, lat: f32) -> Self {
        Self { lat, ..self }
    }

    pub fn timezone(self, timezone: f32) -> Self {
        Self { timezone, ..self }
    }

    pub fn sunrise_mean_anomaly(&self) -> f32 {
        let long_hour = self.long / 15.0;

        let t = self.doy as f32 + ((6.0 - long_hour) / 24.0);

        (0.9856 * t) - 3.289
    }

    pub fn sunset_mean_anomaly(&self) -> f32 {
        let long_hour = self.long / 15.0;

        let t = self.doy as f32 + ((18.0 - long_hour) / 24.0);

        (0.9856 * t) - 3.289
    }

    pub fn sunrise_true_long_in_deg(&self) -> f32 {
        let sma = self.sunrise_mean_anomaly();
        let mut l = sma
            + (1.916 * sma.to_radians().sin())
            + (0.020 * (2.0 * sma).to_radians().sin())
            + 282.634;

        let stl = if l < 0.0 {
            l += 360.0;
            l
        } else if l > 360.0 {
            l -= 360.0;
            l
        } else {
            l
        };

        stl
    }

    pub fn sunset_true_long_in_deg(&self) -> f32 {
        let sma = self.sunset_mean_anomaly();
        let mut l = sma
            + (1.916 * sma.to_radians().sin())
            + (0.020 * (2.0 * sma).to_radians().sin())
            + 282.634;

        let stl = if l < 0.0 {
            l += 360.0;
            l
        } else if l > 360.0 {
            l -= 360.0;
            l
        } else {
            l
        };

        stl
    }

    pub fn sunrise_declination(&self) -> f32 {
        let stl = self.sunrise_true_long_in_deg();
        (0.39782 * stl.to_radians().sin()).asin().to_degrees()
    }

    pub fn sunset_declination(&self) -> f32 {
        let stl = self.sunset_true_long_in_deg();
        (0.39782 * stl.to_radians().sin()).asin().to_degrees()
    }

    pub fn sunrise_time(&self) -> Result<f32, SunMood> {
        let lha = self.sunrise_local_ha_in_deg()?;
        let ra = self.sunrise_ra_in_hours();
        let doy = self.doy;
        let long = self.long;
        let local_offset = self.timezone;
        let long_hour = long / 15.0;

        let t = doy as f32 + ((6.0 - long_hour) / 24.0);

        let t = lha + ra - (0.06571 * t) - 6.622;
        let mut ut = t - long_hour;

        ut += local_offset;
        if ut < 0.0 {
            ut += 24.0
        };
        if ut > 24.0 {
            ut -= 24.0
        };

        Ok(ut)
    }

    pub fn sunset_time(&self) -> Result<f32, SunMood> {
        let lha = self.sunset_local_ha_in_deg()?;
        let ra = self.sunset_ra_in_hours();
        let doy = self.doy;
        let long = self.long;
        let local_offset = self.timezone;
        let long_hour = long / 15.0;

        let t = doy as f32 + ((18.0 - long_hour) / 24.0);

        let t = lha + ra - (0.06571 * t) - 6.622;
        let mut ut = t - long_hour;

        ut += local_offset;
        if ut < 0.0 {
            ut += 24.0
        };
        if ut > 24.0 {
            ut -= 24.0
        };

        Ok(ut)
    }

    pub fn day_length(&self) -> Result<f32, SunMood> {
        Ok(self.sunset_time()? - self.sunrise_time()?)
    }

    /// Sun Rise Right Ascension on the given day and location
    pub fn sunrise_ra_in_hours(&self) -> f32 {
        let stl = self.sunrise_true_long_in_deg();
        let mut ra = (180.0 / PI) * (0.91764 * stl.to_radians().tan()).atan();

        let mut ra = if ra < 0.0 {
            ra += 360.0;
            ra
        } else if ra > 360.0 {
            ra -= 360.0;
            ra
        } else {
            ra
        };

        let l_quadrant = (stl / 90.0).floor() * 90.0;
        let r_quadrant = (ra / 90.0).floor() * 90.0;

        ra = (ra + l_quadrant - r_quadrant) / 15.0;

        ra
    }

    /// Sun Set Right Ascension on the given day and location
    pub fn sunset_ra_in_hours(&self) -> f32 {
        let stl = self.sunset_true_long_in_deg();
        let mut ra = (180.0 / PI) * (0.91764 * stl.to_radians().tan()).atan();

        let mut ra = if ra < 0.0 {
            ra += 360.0;
            ra
        } else if ra > 360.0 {
            ra -= 360.0;
            ra
        } else {
            ra
        };

        let l_quadrant = (stl / 90.0).floor() * 90.0;
        let r_quadrant = (ra / 90.0).floor() * 90.0;

        ra = (ra + l_quadrant - r_quadrant) / 15.0;

        ra
    }

    /// Sun Rise Local Hour Angle on the given day and location.
    /// This returns a Result<> as there are locations where the Sun never rises on a given day
    pub fn sunrise_local_ha_in_deg(&self) -> Result<f32, SunMood> {
        let dec = self.sunrise_declination();
        let lat = self.lat;
        let cos_lha = (ZENITH.to_radians().cos()
            - (dec.to_radians().sin() * lat.to_radians().sin()))
            / (dec.to_radians().cos() * lat.to_radians().cos());

        if cos_lha > 1.0 {
            return Err(SunMood::NeverRise);
        } else if cos_lha < -1.0 {
            return Err(SunMood::NeverSet);
        } else {
            //
        }

        let ha = (180.0 / PI) * cos_lha.acos();
        let ha = 360.0 - ha;
        Ok(ha / 15.0)
    }

    /// Sun Set Local Hour Angle on the given day and location.
    /// This returns a Result<> as there are locations where the Sun never sets on a given day
    pub fn sunset_local_ha_in_deg(&self) -> Result<f32, SunMood> {
        let dec = self.sunset_declination();
        let lat = self.lat;
        let cos_lha = (ZENITH.to_radians().cos()
            - (dec.to_radians().sin() * lat.to_radians().sin()))
            / (dec.to_radians().cos() * lat.to_radians().cos());

        if cos_lha > 1.0 {
            return Err(SunMood::NeverRise);
        } else if cos_lha < -1.0 {
            return Err(SunMood::NeverSet);
        } else {
            //
        }

        let ha = (180.0 / PI) * cos_lha.acos();
        let ha = ha;
        Ok(ha / 15.0)
    }

}

