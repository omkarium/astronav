//! Track the Positional Coordinates of stars
// Copyright (c) 2024 Venkatesh Omkaram

use std::marker::PhantomData;

use super::struct_types::*;

/// A safe way to find the Altitude and Azimuth of a given Star
#[allow(unused)]
#[derive(Debug, Clone)]
pub struct AltAz {
    dec: f64,
    lat: f64,
    lst: f64,
    ra: f64,
    alt: f64,
    ha: f64,
}

impl AltAz {
    /// Returns the Altitude of a celestial body in `Decimal Degrees`
    pub fn get_altitude(&self) -> f64 {
        self.alt.to_degrees()
    }

    /// Returns the Azimuth of a celestial body in `Decimal Degrees`
    pub fn get_azimuth(&self) -> f64 {
        let alt_tup = self.alt.sin_cos();
        let lat_tup = self.lat.sin_cos();

        let az = ((self.dec.sin() - (alt_tup.0 * lat_tup.0)) / (alt_tup.1 * lat_tup.1))
            .acos()
            .to_degrees();
        if self.ha.to_degrees() / 15.0 < 12.0 {
            360.0 - az
        } else {
            az
        }
    }
}

/// Helps to build an AltAz type using a `builder pattern`
/// # Example 1
/// 
/// The star `Sirius` has the Right Ascension `101.5504` and declination `-16.75122`.
/// You are the observer and the local mean sidereal time for your longitude is `199.05`.
/// Your latitude is `12.45`. All the values are in degrees.
/// 
/// Using this information you can construct the AltAz type like the below and call on its getter methods
/// ```
/// use astronav::coords::star::AltAzBuilder;
/// 
/// let alt_az = AltAzBuilder::new()
///             .dec(-16.75122)
///             .lat(12.45)
///             .lmst(199.05)
///             .ra(101.5504)
///             .seal()
///             .build();
/// 
/// assert_eq!(-10.613191752481162, alt_az.get_altitude());
/// assert_eq!(254.99375998808006, alt_az.get_azimuth());
/// ```
/// Notice that we are calling the seal() method on the AltAzBuilder only after setting all the setter properties.
/// If you try to set a property say ra(70.2) after calling the seal(), that will result in a compiler error.
/// 
/// # Example 2
/// Similar to the above example if you do not have the property values in degrees, you can pass Degrees Minutes Seconds and Hour Minutes Seconds
/// like below
/// ```
/// use astronav::coords::{star::AltAzBuilder, dms_to_deg, hms_to_deg};
/// 
/// // Values for Antares
/// let alt = AltAzBuilder::new()
///             .dec(dms_to_deg("-26:29:11.8").unwrap())
///             .lat(dms_to_deg("12:27:0").unwrap())
///             .lmst(hms_to_deg("13:23:30").unwrap())
///             .ra(hms_to_deg("16:30:55.2").unwrap())
///             .seal()
///             .build();
///
/// assert_eq!(30.10106212143597, alt.get_altitude());
/// assert_eq!(130.98870686438966, alt.get_azimuth());
/// ```
#[derive(Default, Clone)]
pub struct AltAzBuilder<U, K, L, M, S> {
    dec: U,
    lat: K,
    lst: L,
    ra: M,
    alt: Option<f64>,
    marker_seal: PhantomData<S>,
}

impl AltAzBuilder<NoDec, NoLat, NoLst, NoRA, NotSealed> {

    /// Returns the default implementation for AltAzBuilder
    pub fn new() -> Self {
        AltAzBuilder::default()
    }
}

impl AltAzBuilder<Dec, Lat, Lst, RA, NotSealed> {

    /// Seals the AltAzBuilder type and protects it from adding no more setter method calls
    pub fn seal(self) -> AltAzBuilder<Dec, Lat, Lst, RA, Sealed> {
        AltAzBuilder {
            dec: self.dec,
            lat: self.lat,
            lst: self.lst,
            ra: self.ra,
            alt: self.alt,
            marker_seal: PhantomData::<Sealed>,
        }
    }
}

impl AltAzBuilder<Dec, Lat, Lst, RA, Sealed> {

    /// Builds an AltAz type using an AltAzBuilder
    pub fn build(self) -> AltAz {
        let dec = self.dec.0;

        let lat = self.lat.0;
        let lst = self.lst.0;
        let ra = self.ra.0;

        let dec_tup = dec.sin_cos();
        let lat_tup = lat.sin_cos();

        let ha = if lst > ra { lst - ra } else { ra - lst };

        let ha_for_az = if lst.to_degrees() - ra.to_degrees() < 0.0 {
            (360.0_f64 + (lst.to_degrees() - ra.to_degrees())).to_radians()
        } else {
            lst - ra
        };

        let alt = (dec_tup.0 * lat_tup.0 + dec_tup.1 * lat_tup.1 * ha.cos()).asin();

        AltAz {
            dec,
            lat,
            lst,
            ra,
            alt,
            ha: ha_for_az,
        }
    }
}

impl<U, K, L, M, S> AltAzBuilder<U, K, L, M, S> {

    /// Sets the declination angle in `Decimal Degrees` and returns the AltAzBuilder
    pub fn dec(self, dec: f64) -> AltAzBuilder<Dec, K, L, M, NotSealed> {
        AltAzBuilder {
            dec: Dec(dec.to_radians()),
            lat: self.lat,
            lst: self.lst,
            ra: self.ra,
            alt: self.alt,
            marker_seal: PhantomData::<NotSealed>,
        }
    }

    /// Sets the latitude angle in `Decimal Degrees` and returns the AltAzBuilder
    pub fn lat(self, lat: f64) -> AltAzBuilder<U, Lat, L, M, NotSealed> {
        AltAzBuilder {
            dec: self.dec,
            lat: Lat(lat.to_radians()),
            lst: self.lst,
            ra: self.ra,
            alt: self.alt,
            marker_seal: PhantomData::<NotSealed>,
        }
    }

    /// Sets the local mean sidereal time in `Decimal Degrees` and returns the AltAzBuilder
    pub fn lmst(self, lst: f64) -> AltAzBuilder<U, K, Lst, M, NotSealed> {
        AltAzBuilder {
            dec: self.dec,
            lat: self.lat,
            lst: Lst(lst.to_radians()),
            ra: self.ra,
            alt: self.alt,
            marker_seal: PhantomData::<NotSealed>,
        }
    }

    /// Sets the right ascension in `Decimal Degrees` and returns the AltAzBuilder
    pub fn ra(self, ra: f64) -> AltAzBuilder<U, K, L, RA, NotSealed> {
        AltAzBuilder {
            dec: self.dec,
            lat: self.lat,
            lst: self.lst,
            ra: RA(ra.to_radians()),
            alt: self.alt,
            marker_seal: PhantomData::<NotSealed>,
        }
    }
}
