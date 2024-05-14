//! All coordinates related
// Copyright (c) 2024 Venkatesh Omkaram

pub mod star;
mod struct_types;

use std::num::ParseFloatError;

/**
 * Helper struct to convert Degrees Minutes Seconds to Decimal Degrees
 * 
 * # Arguments
 * * Degrees Minutes Seconds as &str in format *| "DD:MM:SS"* 
 * `(note: Do not pass + before DD in case the DD is a positive number. Pass a - in case it is a negative number)`
 * 
 * # Example
 * ```
 * use astronav::coords::DMSToDecimalDeg;
 * 
 * let a = <DMSToDecimalDeg<'_> as TryInto<f64>>::try_into(DMSToDecimalDeg("-26:29:11.8")).unwrap();
 * let b = <DMSToDecimalDeg<'_> as TryInto<f64>>::try_into(DMSToDecimalDeg("14:16:12.2")).unwrap();
 * 
 * assert_eq!(-26.48661111111111, a);
 * assert_eq!(14.270055555555556, b);
 * ```
**/
pub struct DMSToDecimalDeg<'a>(pub &'a str);

/**
 * Helper struct to convert Hours Minutes Seconds to Decimal Degrees
 * 
 * # Arguments
 * * Hours Minutes Seconds as &str in format *| "HH:MM:SS"* 
 * `(note: HH must be in 24 hour format)`
 * 
 * # Example
 * ```
 * use astronav::coords::HMSToDecimalDeg;
 * 
 * let a = <HMSToDecimalDeg<'_> as TryInto<f64>>::try_into(HMSToDecimalDeg("16:30:55.2")).unwrap();
 * 
 * assert_eq!(247.73000000000002, a);
 * ```
**/
pub struct HMSToDecimalDeg<'a>(pub &'a str);


impl<'a> TryInto<f64> for DMSToDecimalDeg<'a> {
    type Error = ParseFloatError;
    fn try_into(self) -> Result<f64, ParseFloatError> {

        let is_negative = self.0.chars().next().map(|x| x.to_string().as_str() == "-").unwrap();
        let a = self.0.split(":").collect::<Vec<&str>>();

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
}

impl<'a> TryInto<f64> for HMSToDecimalDeg<'a> {
    type Error = ParseFloatError;
    fn try_into(self) -> Result<f64, ParseFloatError> {
        let a = self.0.split(":").collect::<Vec<&str>>();
        
            Ok((a[0].parse::<f64>()? + 
            (a[1].parse::<f64>()? / 60.0 + 
            a[2].parse::<f64>()? / 3600.0)) * 15.0)

    }
}