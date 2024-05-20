//! All date and time related
// Copyright (c) 2024 Venkatesh Omkaram

/**
Computes the Julian day number by a given day, month and year
**/
pub fn julian_day_number(day: u8, month: u8, year: u16) -> u32 {
    let a = (((14 - month) / 12) as f32).floor() as u8;
    let y = year + 4800 - a as u16;
    let m = month + (12 * a) - 3;

    let jd = day as u32
        + (((153 * m as u16 + 2) / 5) as f32).floor() as u32
        + (365 * y as u32) as u32
        + ((y / 4) as f32).floor() as u32
        - ((y / 100) as f32).floor() as u32
        + ((y / 400) as f32).floor() as u32
        - 32045;

    jd
}

/**
 * Computes the Julian Time by a given Julian day number, hour, minutes, seconds
 **/
pub fn julian_time(julian_day: u32, hour: u8, min: u8, sec: u8) -> f64 {
    let jt =
        julian_day as f64 + ((hour as f64 - 12.0) / 24.0) + (min as f64 / 1440.0) + (sec as f64 / 86400.0);
    jt
}

/**
 * Computes the Greenwich Mean Sidereal Time by a given Julian Time
 * 
 * # Returns
 *  Greenwich Mean Sidereal Time in `Decimal Degrees` 
 **/
pub fn gmst_in_degrees(julian_time: f64) -> f64 {
    let jdt_tt = julian_time - 2451545.0;
    let frac_time_elapsed = jdt_tt / 36525.0;
    dbg!(jdt_tt);
    let gmst =
        (280.46061837 + (360.98564736629 * jdt_tt) + (0.000387933 * frac_time_elapsed.powi(2))
            - (frac_time_elapsed.powi(3) / 38710000.0))
            .rem_euclid(360.0);
    gmst
}

/**
 * Computes the Local Mean Sidereal Time by a given Greenwich Mean Sidereal Time and Longitude
 * 
 * # Arguments
 * * `gmst_in_deg`: Greenwich Mean Sidereal Time in | `Decimal Degrees floating point`
 * * `longitude`: Longitude of Local Meridian in | `Decimal Degrees floating point`
 * 
 * # Returns
 *  Local Mean Sidereal Time in `Decimal Degrees` 
 **/
pub fn lmst_in_degrees(gmst_in_deg: f64, longitude: f64) -> f64 {
    gmst_in_deg + longitude
}

/// Computes the day of the year
pub fn day_of_year(year: u16, month: u8, day: u8) -> u16 {
        let n1 = (275 * month as u16) / 9;
        let n2 = ((month + 9) / 12) as u16 * (1 + ((year - 4 * (year / 4) + 2) / 3));
        let n3 = 30_u16;
        (n1 - n2 + day as u16 - n3).into()
}

/// Computes the fractional day of the year by the hour
pub fn frac_day_of_year(year: u16, month: u8, day: u8, timezone: f32, hour: f32) -> f32 {
    let days_in_year = if is_leap_year(year) {
        366.0
    } else {
        365.0
    };
    let doy = day_of_year(year, month, day);
    let fy = (doy as f32 / days_in_year) + (doy as f32 - 1.0) - (timezone/24.0) + (hour/24.0);
    fy
}

pub fn is_leap_year(year: u16) -> bool {
    if (year % 4 == 0 && !(year % 100 == 0)) || (year % 400 == 0) {
        true
    } else {
        false
    }
}

/**
 * Use this struct if do not wish to use free standing functions in the `time` module.
 **/
pub struct AstroTime {
    pub day: u8,
    pub month: u8,
    pub year: u16,
    pub hour: u8,
    pub min: u8,
    pub sec: u8,
}

impl AstroTime {

/**
 * Returns the Greenwich Mean Sidereal Time in `Decimal Degrees`
**/
    pub fn gmst_in_degrees(&self) -> f64 {
        let julian_time = self.julian_time();
        let jdt_tt = julian_time - 2451545.0;
        let frac_time_elapsed = jdt_tt / 36525.0;

        let gmst =
            (280.46061837 + (360.98564736629 * jdt_tt) + (0.000387933 * frac_time_elapsed.powi(2))
                - (frac_time_elapsed.powi(3) / 38710000.0))
                .rem_euclid(360.0);
        gmst
    }

/**
 * Returns the Julian Day Number
**/
    pub fn julian_day_number(&self) -> u32 {
        julian_day_number(self.day, self.month, self.year)
    }

/**
 * Returns the Julian Time
**/
    pub fn julian_time(&self) -> f64 {
        julian_time(self.julian_day_number(), self.hour, self.min, self.sec)
    }

/**
 * Compute and return the Local Mean Sidereal Time by a given Longitude in `Decimal Degrees` 
 * 
 * # Arguments
 * * `longitude`: Longitude of the Local Meridian in | `Decimal Degrees floating point`
 **/
    pub fn lmst_in_degrees(&self, longitude: f64) -> f64 {
        lmst_in_degrees(self.gmst_in_degrees(), longitude)
    }

/**
 * Computes the Local Mean Sidereal Time by a given Longitude in `Decimal Degrees`
 * 
 * # Arguments
 * * `longitude`: Longitude of the Local Meridian in | `Decimal Degrees floating point`
 **/
    pub fn lmst_in_decimal_hours(&self, longitude: f64) -> f32 {
        (lmst_in_degrees(self.gmst_in_degrees(), longitude)/15.0) as f32
    }

    /// Computes the day of the year
    pub fn day_of_year(&self) -> u16 {
       day_of_year(self.year, self.month, self.day)
    }
    
}
