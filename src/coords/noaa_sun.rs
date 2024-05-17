//! Track the Sun positional coordinates and time using NOAA algorithms
// Copyright (c) 2024 Venkatesh Omkaram

use std::f64::consts::PI;

use crate::time::day_of_year;


/// A Struct to find the Sun Rise, Sun Set and other items about the Sun using NOAA Algorithms
/// 
/// * Note: Using this struct not only helps you to find sun rise and sun set, but you can also find the 
/// Azimuth and Altitude of the Sun at any point in time. This feature is not available in the `sun` module
/// 
/// # Example 1
/// Calculating the Sun Positional Properties on May 17th 2024, Chennai India
/// ```
/// use astronav::coords::{hours_to_hms, noaa_sun::NOAASun};
/// 
/// // Test Sun rise, Sun set and other things for Chennai, India
/// let chennai_sun = NOAASun {
///     year: 2024,
///     doy: 138,
///     long: 80.2705,
///     lat: 13.0843,
///     timezone: 5.5,
///     hour: 13,
///     min: 08,
///     sec: 47,
/// };
/// 
/// let fy = chennai_sun.frac_year_by_hour_in_rads();
/// let eot = chennai_sun.true_eot_in_mins();
/// let dec = chennai_sun.declination();
/// let ha = chennai_sun.ha_pos_time_in_deg();
/// let sza = chennai_sun.zenith_in_deg();
/// let alt = chennai_sun.altitude_in_deg();
/// let saa = chennai_sun.azimuth_in_deg();
/// let sun_rise = chennai_sun.sunrise_time_hours();
/// let sun_rise_mins = chennai_sun.sunrise_time_mins();
/// let sun_noon = chennai_sun.noon_hours();
/// let sun_noon_mins = chennai_sun.noon_mins();
/// let sun_set: f64 = chennai_sun.sunset_time_hours();
/// let sun_set_mins: f64 = chennai_sun.sunset_time_mins();
/// 
/// assert_eq!(2.352617995823504, fy);
/// assert_eq!(3.8842598773463117, eot);
/// assert_eq!(19.2872916085781, dec);
/// assert_eq!(15.741565152442035, ha);
/// assert_eq!(16.3319240544742, sza);
/// assert_eq!(73.6680759455258, alt);
/// assert_eq!(294.4139960879158, saa);
/// assert_eq!("5:42:43.974809411752176".to_owned(), hours_to_hms(sun_rise));
/// assert_eq!(342.73291349019587, sun_rise_mins);
/// assert_eq!("12:5:1.952365544096324".to_owned(), hours_to_hms(sun_noon));
/// assert_eq!(725.032539425735, sun_noon_mins);
/// assert_eq!("18:27:19.929921676427682".to_owned(), hours_to_hms(sun_set));
/// assert_eq!(1107.3321653612738, sun_set_mins);
/// ```
/// # Example 2
/// We will pass the same parameters as the above example, but using setters
/// ```
/// use astronav::coords::{hours_to_hms, noaa_sun::NOAASun};
/// 
/// // Test Sun rise, Sun set and other things for Chennai, India
/// let chennai_sun = NOAASun::new()
///                     .date(2024, 05, 17)
///                     .long(80.2705)
///                     .lat(13.0843)
///                     .timezone(5.5)
///                     .hour(13)
///                     .min(08)
///                     .sec(47);
///
/// 
/// let fy = chennai_sun.frac_year_by_hour_in_rads();
/// let eot = chennai_sun.true_eot_in_mins();
/// let dec = chennai_sun.declination();
/// let ha = chennai_sun.ha_pos_time_in_deg();
/// let sza = chennai_sun.zenith_in_deg();
/// let alt = chennai_sun.altitude_in_deg();
/// let saa = chennai_sun.azimuth_in_deg();
/// let sun_rise = chennai_sun.sunrise_time_hours();
/// let sun_rise_mins = chennai_sun.sunrise_time_mins();
/// let sun_noon = chennai_sun.noon_hours();
/// let sun_noon_mins = chennai_sun.noon_mins();
/// let sun_set: f64 = chennai_sun.sunset_time_hours();
/// let sun_set_mins: f64 = chennai_sun.sunset_time_mins();
/// 
/// assert_eq!(2.352617995823504, fy);
/// assert_eq!(3.8842598773463117, eot);
/// assert_eq!(19.2872916085781, dec);
/// assert_eq!(15.741565152442035, ha);
/// assert_eq!(16.3319240544742, sza);
/// assert_eq!(73.6680759455258, alt);
/// assert_eq!(294.4139960879158, saa);
/// assert_eq!("5:42:43.974809411752176".to_owned(), hours_to_hms(sun_rise));
/// assert_eq!(342.73291349019587, sun_rise_mins);
/// assert_eq!("12:5:1.952365544096324".to_owned(), hours_to_hms(sun_noon));
/// assert_eq!(725.032539425735, sun_noon_mins);
/// assert_eq!("18:27:19.929921676427682".to_owned(), hours_to_hms(sun_set));
/// assert_eq!(1107.3321653612738, sun_set_mins);
/// ```
#[derive(Debug, Clone, Default)]
pub struct NOAASun {
    /// Year of interest
    pub year: u16,
    /// Day of the year (Example: May 16th, 2024 is day 137)
    pub doy: u16,
    /// Longitude of the point of interest in degrees (+ east, - west)
    pub long: f32,
    /// Latitude of the point of interest in degrees (+ north, - south)
    pub lat: f32,
    /// Timezone of the point of interest in hours (+ east, - west)
    pub timezone: f32,
    /// Hour of of interest (24 hour format)
    pub hour: u8,
    /// Minute of interest
    pub min: u8,
    /// Second of interest
    pub sec: u8,
}

impl NOAASun {
     /// Provides a default implementation for the value in the struct
     pub fn new() -> Self {
        Self::default()
    }

    pub fn date(self, year: u16, month: u8, day: u8) -> Self {
        let doy = day_of_year(year, month, day);
        Self { doy, year, ..self}
    }

    pub fn long(self, long: f32) -> Self {
        Self { long, ..self}
    }

    pub fn lat(self, lat: f32) -> Self {
        Self { lat, ..self }
    }

    pub fn timezone(self, timezone: f32) -> Self {
        Self { timezone, ..self }
    }

    pub fn hour(self, hour: u8) -> Self {
        Self { hour, ..self }
    }

    pub fn min(self, min: u8) -> Self {
        Self { min, ..self }
    }

    pub fn sec(self, sec: u8) -> Self {
        Self { sec, ..self }
    }
    
    /// Returns the fractional years in radians for a given year, day of the year, and the hour
    pub fn frac_year_by_hour_in_rads(&self) -> f64 {
        let days_in_year = if is_leap_year(self.year) {
            366.0
        } else {
            365.0
        };

        let fy = (2.0 * PI / days_in_year)
            * (self.doy as f64 - 1.0 + ((self.hour as f64 - 12.0) / 24.0));
        fy
    }

    /// Returns the fractional years in radians for a given year, day of the year
    pub fn frac_year_by_day_in_rads(&self) -> f64 {
        let days_in_year = if is_leap_year(self.year) {
            366.0
        } else {
            365.0
        };

        let fy = (2.0 * PI / days_in_year) * (self.doy as f64 - 1.0);
        fy
    }

    /// Returns the equation of time in mins for a computed fractional year by hour
    pub fn true_eot_in_mins(&self) -> f64 {
        let eot = 229.18
            * (0.000075 + (0.001868 * self.frac_year_by_hour_in_rads().cos())
                - (0.032077 * self.frac_year_by_hour_in_rads().sin())
                - (0.014615 * (2.0 * self.frac_year_by_hour_in_rads()).cos())
                - (0.040849 * (2.0 * self.frac_year_by_hour_in_rads()).sin()));
        eot
    }

    /// Returns the equation of time in mins for a computed fractional year
    pub fn eot_in_mins(&self) -> f64 {
        let eot = 229.18
            * (0.000075 + (0.001868 * self.frac_year_by_day_in_rads().cos())
                - (0.032077 * self.frac_year_by_day_in_rads().sin())
                - (0.014615 * (2.0 * self.frac_year_by_day_in_rads()).cos())
                - (0.040849 * (2.0 * self.frac_year_by_day_in_rads()).sin()));
        eot
    }

    /// Sun's declination for a given fractional year calculated by hour
    pub fn true_declination(&self) -> f64 {
        let dec: f64 = 0.006918 - (0.399912 * self.frac_year_by_hour_in_rads().cos())
            + (0.070257 * self.frac_year_by_hour_in_rads().sin())
            - (0.006758 * (2.0 * self.frac_year_by_hour_in_rads()).cos())
            + (0.000907 * (2.0 * self.frac_year_by_hour_in_rads()).sin())
            - (0.002697 * (3.0 * self.frac_year_by_hour_in_rads()).cos())
            + (0.00148 * (3.0 * self.frac_year_by_hour_in_rads()).sin());

        dec.to_degrees()
    }

    /// Sun's declination for a given fractional year by day
    pub fn declination(&self) -> f64 {
        let dec: f64 = 0.006918 - (0.399912 * self.frac_year_by_day_in_rads().cos())
            + (0.070257 * self.frac_year_by_day_in_rads().sin())
            - (0.006758 * (2.0 * self.frac_year_by_day_in_rads()).cos())
            + (0.000907 * (2.0 * self.frac_year_by_day_in_rads()).sin())
            - (0.002697 * (3.0 * self.frac_year_by_day_in_rads()).cos())
            + (0.00148 * (3.0 * self.frac_year_by_day_in_rads()).sin());

        dec.to_degrees()
    }

    /// Returns the Sun hour angle in degrees for a given longitude and time
    pub fn ha_pos_time_in_deg(&self) -> f64 {
        let time_offset =
            self.true_eot_in_mins() + (4.0 * self.long as f64) - 60.0 * self.timezone as f64;
        let true_solar_time = ((self.hour as u32 * 60) + self.min as u32 + (self.sec as u32 / 60))
            as f64
            + time_offset;

        let mut hour_angle = (true_solar_time / 4.0) - 180.0;

        if hour_angle < 0.0 {
            hour_angle += 360.0;
        }

        hour_angle
    }

    /// Returns the Zenith Angle of the sun for a given declination, latitude, and hour angle
    pub fn zenith_in_deg(&self) -> f64 {
        let dec = self.true_declination();
        let lat = self.lat as f64;
        let sza = ((lat.to_radians().sin() * dec.to_radians().sin())
            + (lat.to_radians().cos()
                * dec.to_radians().cos()
                * self.ha_pos_time_in_deg().to_radians().cos()))
        .acos();

        sza.to_degrees()
    }

    /// Returns the Altitude of the sun for a given declination, latitude, and hour angle
    pub fn altitude_in_deg(&self) -> f64 {
        90.0 - self.zenith_in_deg()
    }

    /// Returns the Azimuth angle of the sun for a given declination, latitude and zenith angle
    pub fn azimuth_in_deg(&self) -> f64 {
        let dec = self.true_declination();
        let lat = self.lat as f64;
        let sza = self.zenith_in_deg();
        let sha = self.ha_pos_time_in_deg();

        let saa: f64 = -(((lat.to_radians().sin() * sza.to_radians().cos())
            - dec.to_radians().sin())
            / (lat.to_radians().cos() * sza.to_radians().sin()));

        if sha > 180.0 {
            saa.acos().to_degrees()
        } else {
            360.0 - saa.acos().to_degrees()
        }
    }

    pub fn sunrise_time_hours(&self) -> f64 {
        self.sunrise_time_mins() / 60.0
    }

    pub fn noon_hours(&self) -> f64 {
        self.noon_mins() / 60.0
    }

    pub fn sunset_time_hours(&self) -> f64 {
        self.sunset_time_mins() / 60.0
    }

    pub fn sunrise_time_mins(&self) -> f64 {
        let dec = self.declination();
        let lat = self.lat as f64;
        let long = self.long as f64;
        let eot = self.eot_in_mins();

        let ha = ((90.833_f64.to_radians().cos()
            / (lat.to_radians().cos() * dec.to_radians().cos()))
            - (lat.to_radians().tan() * dec.to_radians().tan()))
        .acos();

        720.0 - (4.0 * (long + ha.to_degrees())) - eot + (self.timezone as f64 * 60.0)
    }

    pub fn noon_mins(&self) -> f64 {
        let long = self.long as f64;
        let eot = self.eot_in_mins();

        720.0 - (4.0 * (long)) - eot + (self.timezone as f64 * 60.0)
    }

    pub fn sunset_time_mins(&self) -> f64 {
        let dec = self.declination();
        let lat = self.lat as f64;
        let long = self.long as f64;
        let eot = self.eot_in_mins();

        let ha = (-(90.833_f64.to_radians().cos()
            / (lat.to_radians().cos() * dec.to_radians().cos()))
            + (lat.to_radians().tan() * dec.to_radians().tan()))
        .acos();

        1440.0 - (4.0 * (long + ha.to_degrees())) - eot + (self.timezone as f64 * 60.0)
    }
}

pub fn is_leap_year(year: u16) -> bool {
    if (year % 4 == 0 && !(year % 100 == 0)) || (year % 400 == 0) {
        true
    } else {
        false
    }
}

#[allow(unused)]
pub fn equation_of_time(y: f64, doy: f64) -> f64 {
    let t = 365.0 * (y - 2000.0) + doy;
    let eot = -7.659 * (6.24004077 + 0.01720197 * t).sin()
        + 9.863 * (2.0 * (6.24004077 + 0.01720197 * t) + 3.5932).sin();
    eot
}
