use std::f64::consts::PI;

pub fn solar_dec_angle_rads(frac_year_in_rads: f64) -> f64 {
    let dec: f64 = 0.006918 - (0.399912 * frac_year_in_rads.cos()) + 
        (0.070257 * frac_year_in_rads.sin()) - 
        (0.006758 * (2.0 * frac_year_in_rads).cos()) + 
        (0.000907 * (2.0 * frac_year_in_rads).sin()) - 
        (0.002697 * (3.0 * frac_year_in_rads).cos()) + 
        (0.00148 * (3.0 * frac_year_in_rads).sin ());

    dec

}

pub fn solar_hour_angle_in_deg(eot: f64, long: f64, tz: f64, hr: u16, min: u16, sec:u16) -> f64 {
    let time_offset = eot + (4.0 * long) - (60.0 * tz);
    let true_solar_time = ((hr * 60) + min + (sec/60)) as f64 + time_offset;

    let hour_angle = (true_solar_time / 4.0) - 180.0;

    hour_angle

}

pub fn solar_zenith_angle_in_deg(ha: f64, lat: f64, dec: f64) -> f64 {
    let sza = (lat.to_radians().sin() * dec.sin()) + (lat.to_radians().cos() * dec.cos() * ha.to_radians().cos());
    sza
}

pub fn solar_azimuth_angle_in_deg(az: f64, lat: f64, dec: f64) -> f64 {
    let saa = - (((lat.sin() * az.cos()) - dec.sin())/(lat.cos() * az.sin()));
    dbg!(saa);
    dbg!(saa.acos());
    360.0 - saa.acos().to_degrees()
}

pub fn frac_year_in_rads(year: u16, day_of_year: u16, hour: u8) -> f64{

    let days_in_year = if is_leap_year(year){
        366.0
    } else {
        365.0
    };

    dbg!(is_leap_year(year));

    let fy = (2.0 * PI/days_in_year) * (day_of_year as f64 - 1.0  + ((hour as f64 - 12.0)/24.0));
    fy
}

pub fn is_leap_year(year: u16) -> bool {
    if (year % 4 == 0 && !(year % 100 == 0)) || (year % 400 == 0) { true } else { false }
}

pub fn equation_of_time_in_mins(frac_year_in_rads: f64) -> f64{
    let eot = 229.18 * (0.000075 + 
        (0.001868 * frac_year_in_rads.cos()) -
        (0.032077 * frac_year_in_rads.sin()) - 
        (0.014615 * (2.0 * frac_year_in_rads).cos()) - 
        (0.040849 * (2.0 * frac_year_in_rads).sin()));
    eot
}

fn equation_of_time(y: f64, d: f64) -> f64 {
    let t = 365.0 * (y - 2000.0) + d;
    let eot = -7.659 * (6.24004077 + 0.01720197 * t).sin() +
              9.863 * (2.0 * (6.24004077 + 0.01720197 * t) + 3.5932).sin();
    eot
}

pub fn sunrise_set(lat: f64, dec: f64, long: f64, eot: f64) -> f64 {
    let ha = (90.833_f64.to_radians().cos()/(lat.to_radians().cos() * dec.cos())) - (lat.to_radians().tan() * dec.tan());
    720.0 - (4.0 * (long + ha)) + eot
}


#[test]
fn test_frac_year() {
    let fy = frac_year_in_rads(2024, 137, 15);

    assert_eq!(2.336881420600604, fy);

    let eot = equation_of_time(2024.0, 137.0);

    assert_eq!(3.5858528015263316, eot);

    let dec = solar_dec_angle_rads(fy);

    println!("declination result: {}", dec.to_degrees());

    let ha = solar_hour_angle_in_deg(eot, 79.45, 5.5, 15, 00, 00);
    println!("hour angle result: {}", ha);

    let sza = solar_zenith_angle_in_deg(42.8925, 14.45, 19.2563_f64.to_radians());
    println!("Solar zenith angle: {}", 90.0 - sza.to_degrees());

    let saa = solar_azimuth_angle_in_deg(sza, 14.45_f64.to_radians(), dec);
    println!("Solar azimuth angle: {}", saa);

    let sun_rise = sunrise_set(14.45, dec, 79.45, eot);
    println!("Sun rise : {}", sun_rise/60.0);



}

#[test]
fn test_eot() {
    let year = 2024.0;
    let day = 137.0; // Example day
    let result = equation_of_time(year, day);
    println!("Equation result: {}", result);


}