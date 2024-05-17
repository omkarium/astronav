//! All celestial coordinates related
// Copyright (c) 2024 Venkatesh Omkaram
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
 * assert_eq!("5:37:19.054919999998816".to_owned(), a);
 * ```
**/
pub fn hours_to_hms(hours: f64) -> String {
    let hms = format!("{}:{}:{}", hours.floor(), (hours.fract() * 60.0).floor(), (hours.fract() * 60.0).fract() * 60.0);
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
 * assert_eq!((5,37,19.054919999998816), a);
 * ```
**/
pub fn hours_to_hms_tuple(hours: f64) -> (u8, u8, f64) {
    (hours.floor() as u8, (hours.fract() * 60.0).floor() as u8, (hours.fract() * 60.0).fract() * 60.0)
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
 * assert_eq!("155:37:19.05491999996684".to_owned(), a);
 * ```
**/
pub fn deg_to_dms(deg: f64) -> String {
    let dms = format!("{}:{}:{}", deg.floor(), (deg.fract() * 60.0).floor(), (deg.fract() * 60.0).fract() * 60.0);
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
 * assert_eq!((125,37,19.054920000018), a);
 * ```
**/
pub fn deg_to_dms_tuple(deg: f64) -> (u8, u8, f64) {
    (deg.floor() as u8, (deg.fract() * 60.0).floor() as u8, (deg.fract() * 60.0).fract() * 60.0)
}