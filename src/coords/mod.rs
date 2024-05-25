//! All celestial coordinates related
//! 
// Copyright (c) 2024 Venkatesh Omkaram

//! `Coords` - short form for coordinates is a module which focuses on coordinate transformations of celestial bodies.
//! This module helps to track celestial bodies like the Sun, the stars etc. 
//! 
//! ## Finding A Star with Equatorial Coordinates
//! In order to find or track the position of a Star from an observer's location by a latitude and longitude 
//! we must know is also the RA (Right Ascension) and Dec (Declination) values of the Star. The RA and Dec belong to the 
//! Equatorial coordinate system. You can go to the `star` module to see examples how to use the RA and Dec values.
//! 
//! ### Here are some RA and Dec values of some popular stars
//! |Star  | RA | DEC |
//! |------|----|-----|
//! |Sirius|06:45:09.0|-16:43:06|
//! |Canopus|06:3:57.10988|-52:41:44.3810|
//! |Rigel|05:14:32.049|-08:12:14.78|
//! |Vega|18:36:56.33635|38:47:01.2802|
//! |Procyon|07:40:33.5|05:09:44.5|
//! |Alpha Centauri A|14:39:36.49400|-60:50:02.3737|
//! |Betelgeuse|05:55:10.30536|07:24:25.4304|
//! |Antares|16:29:24.45970|-26:25:55.2094|
//! |Spica|13:25:11.579|-11:09:40.75|
//! |Acrux|12:26:35.89522|-63:05:56.7343|
//! |Pollux|07:45:18.94987|28:01:34.3160|
//! |Arcturus|14:15:39.7|19:10:56|
//! |Aldebaran|04:35:55.23907|16:30:33.4885|
//! |Fomalhaut|22:57:39.0465|-29:37:20.050|
//! |Polaris|02:31:49.09|89:15:50.8|

#![deny(clippy::all)]
#![cfg_attr(docsrs, feature(doc_cfg))]

pub mod star;
pub mod sun;
mod struct_types;

#[cfg(feature = "noaa-sun")]
#[cfg_attr(docsrs, doc(cfg(feature = "noaa-sun")))]
pub mod noaa_sun;


use std::num::ParseFloatError;

/**
 * function to convert Degrees Minutes Seconds to Decimal Degrees
 * 
 * # Arguments
 * * Degrees Minutes Seconds as &str in format *| "DD:MM:SS"* 
 * `(note: Do not pass + before DD in case the DD is a positive number. Pass a - in case it is a negative number)`
 * 
 * # Example
 * ```
 * use astronav::coords::dms_to_deg;
 * 
 * let a = dms_to_deg("-26:29:11.8").unwrap();
 * let b = dms_to_deg("14:16:12.2").unwrap();
 * 
 * assert_eq!(-26.48661111111111, a);
 * assert_eq!(14.270055555555556, b);
 * ```
**/
pub fn dms_to_deg(dms: &str) -> Result<f64, ParseFloatError> {
    let is_negative: bool = dms.chars().next().map(|x| x.to_string().as_str() == "-").unwrap();
    let a: Vec<&str> = dms.split(":").collect::<Vec<&str>>();

    if is_negative {
        Ok(a[0].parse::<f64>()? - 
        (a[1].parse::<f64>()? / 60.0 + 
        a[2].parse::<f64>()? / 3600.0))

    } else {
        Ok(a[0].parse::<f64>()? + 
        (a[1].parse::<f64>()? / 60.0 + 
        a[2].parse::<f64>()? / 3600.0))
    }
}

/**
 * function to convert Hours Minutes Seconds to Decimal Degrees
 * 
 * # Arguments
 * * Hours Minutes Seconds as &str in format *| "HH:MM:SS"* 
 * `(note: HH must be in 24 hour format)`
 * 
 * # Example
 * ```
 * use astronav::coords::hms_to_deg;
 * 
 * let a = hms_to_deg("16:30:55.2").unwrap();
 * 
 * assert_eq!(247.73000000000002, a);
 * ```
**/
pub fn hms_to_deg(hms: &str) -> Result<f64, ParseFloatError> {
    let a: Vec<&str> = hms.split(":").collect::<Vec<&str>>();
        
    Ok((a[0].parse::<f64>()? + 
    (a[1].parse::<f64>()? / 60.0 + 
    a[2].parse::<f64>()? / 3600.0)) * 15.0)
}

/**
 * function to convert Hours Minutes Seconds to `Degrees Minutes Seconds`
 * 
 * # Arguments
 * * Hours Minutes Seconds as &str in format *| "DD:MM:SS"* 
 * `(note: HH must be in 24 hour format)`
 * 
 * # Example
 * ```
 * use astronav::coords::hms_to_dms;
 * 
 * let a = hms_to_dms("16:30:55.2").unwrap();
 * 
 * assert_eq!("247:43:47.98462".to_string(), a);
 * ```
**/
pub fn hms_to_dms(hms: &str) -> Result<String, ParseFloatError> {
    let deg = hms_to_deg(hms)?;
    Ok(deg_to_dms(deg as f32))
}

/**
 * function to convert Hours Minutes Seconds to `(Degrees, Minutes, Seconds)` tuple
 * 
 * # Arguments
 * * Hours Minutes Seconds as &str in format *| "DD:MM:SS"* 
 * `(note: HH must be in 24 hour format)`
 * 
 * # Example
 * ```
 * use astronav::coords::hms_to_dms_tuple;
 * 
 * let a = hms_to_dms_tuple("16:30:55.2").unwrap();
 * 
 * assert_eq!((247,43,47.98462), a);
 * ```
**/
pub fn hms_to_dms_tuple(hms: &str) -> Result<(u8, u8, f32), ParseFloatError> {
    let deg = hms_to_deg(hms)?;
    Ok(deg_to_dms_tuple(deg as f32))
}


/**
 * function to convert Decimal Hours to `Hours:Minutes:Seconds` String
 * 
 * # Returns
 * * Hours Minutes Seconds as String in format *| "HH:MM:SS"* 
 * 
 * # Example
 * ```
 * use astronav::coords::hours_to_hms;
 * 
 * let a = hours_to_hms(5.6219597);
 * 
 * assert_eq!("5:37:19.05487".to_owned(), a);
 * ```
**/
pub fn hours_to_hms(hours: f32) -> String {
    let hms = format!("{}:{}:{}", hours.floor(), (hours.fract() * 60.0).floor().abs(), (hours.fract() * 60.0).fract().abs() as f32 * 60.0);
    hms
}

/**
 * function to convert Decimal Hours to `(Hours, Minutes, Seconds)` tuple
 * 
 * # Returns
 * * Hours Minutes Seconds as a tuple in format *| (HH, MM, SS)* 
 * 
 * # Example
 * ```
 * use astronav::coords::hours_to_hms_tuple;
 * 
 * let a = hours_to_hms_tuple(5.6219597);
 * 
 * assert_eq!((5,37,19.05487), a);
 * ```
**/
pub fn hours_to_hms_tuple(hours: f32) -> (u8, u8, f32) {
    (hours.floor() as u8, (hours.fract() * 60.0).floor().abs() as u8, (hours.fract() * 60.0).fract().abs() as f32 * 60.0)
}

/**
 * function to convert Decimal Degrees to `Degrees:Minutes:Seconds` String
 * 
 * # Returns
 * * Degrees Minutes Seconds as String in format *| "DD:MM:SS"* 
 * 
 * # Example
 * ```
 * use astronav::coords::deg_to_dms;
 * 
 * let a = deg_to_dms(155.6219597);
 * 
 * assert_eq!("155:37:19.068604".to_owned(), a);
 * ```
**/
pub fn deg_to_dms(deg: f32) -> String {
    let dms = format!("{}:{}:{}", deg.floor(), (deg.fract() * 60.0).floor().abs(), (deg.fract() * 60.0).fract().abs() as f32 * 60.0 );
    dms
}

/**
 * function to convert Decimal Degrees to `(Degrees, Minutes, Seconds)` tuple
 * 
 * # Returns
 * * Degrees Minutes Seconds as a tuple in format *| (DD, MM, SS)* 
 * 
 * # Example
 * ```
 * use astronav::coords::deg_to_dms_tuple;
 * 
 * let a = deg_to_dms_tuple(125.6219597);
 * 
 * assert_eq!((125,37,19.068604), a);
 * ```
**/
pub fn deg_to_dms_tuple(deg: f32) -> (u8, u8, f32) {
    (deg.floor() as u8, (deg.fract() * 60.0).floor().abs() as u8, (deg.fract() * 60.0).fract().abs() as f32 * 60.0)
}


/**
 * function to convert Decimal Degrees to `Hours:Minutes:Seconds` String
 * 
 * # Returns
 * * Hours Minutes Seconds as String in format *| "DD:MM:SS"* 
 * 
 * # Example
 * ```
 * use astronav::coords::deg_to_hms;
 * 
 * let a = deg_to_hms(5.2545);
 * 
 * assert_eq!("0:21:1.079979".to_owned(), a);
 * ```
**/
pub fn deg_to_hms(deg: f32) -> String {
    hours_to_hms(deg / 15.0)
}

/**
 * function to convert Decimal Degrees to `(Hours, Minutes, Seconds)` tuple
 * 
 * # Returns
 * * Hours Minutes Seconds as a tuple in format *| (HH, MM, SS)* 
 * 
 * # Example
 * ```
 * use astronav::coords::deg_to_hms_tuple;
 * 
 * let a = deg_to_hms_tuple(5.2545);
 * 
 * assert_eq!((0,21,1.079979), a);
 * ```
**/
pub fn deg_to_hms_tuple(deg: f32) -> (u8, u8, f32) {
    hours_to_hms_tuple(deg / 15.0)
}


