//! Sun positional coordinates and time
// Copyright (c) 2024 Venkatesh Omkaram

// Sunrise/sunset calculation in squirrel, hugo@electricimp.com
// Based on http://williams.best.vwh.net/sunrise_sunset_algorithm.htm

use std::f32::consts::PI;

const ZENITH: f32 = 90.833;

// An enum only related to the SunRiseAndSet Struct
#[derive(Debug)]
pub enum SunMood {
    NeverRise,
    NeverSet,
    Rise,
    Set,
}

/// A Struct to related to Sun Rise and Sun Set
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

    pub fn sunrise_declination(&self) -> f32 {
        let stl = self.sunrise_true_long_in_deg();
        (0.39782 * stl.to_radians().sin()).asin().to_degrees()
    }

    pub fn sunset_declination(&self) -> f32 {
        let stl = self.sunset_true_long_in_deg();
        (0.39782 * stl.to_radians().sin()).asin().to_degrees()
    }

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
}

