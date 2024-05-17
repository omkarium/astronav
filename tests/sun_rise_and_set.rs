use astronav::coords::{hours_to_hms, sun::SunRiseAndSet};

#[test]
fn test_sun_rise_in_new_york() {

    // May 16th 2024
    let sun_new_york = SunRiseAndSet {
        doy: 137,
        long: -74.0060,
        lat: 40.7128,
        timezone: -4.0,
    };

    let sma = sun_new_york.sunrise_mean_anomaly();
    let stl = sun_new_york.sunrise_true_long_in_deg();
    let ra = sun_new_york.sunrise_ra_in_hours();
    let dec = sun_new_york.sunrise_declination();
    let lha = sun_new_york.sunrise_local_ha_in_deg();

    let rising = sun_new_york.sunrise_time();

    //dbg!(sma); dbg!(stl); dbg!(ra); dbg!(dec); dbg!(lha.unwrap()); dbg!(rising.unwrap());

    assert_eq!(132.18721, sma);
    assert_eq!(56.220978, stl);
    assert_eq!(3.5939937, ra);
    assert_eq!(19.309036, dec);
    assert_eq!(16.748438, lha.unwrap());
    assert_eq!(5.6219597, *rising.as_ref().unwrap());
    assert_eq!("5:37:19.05487060546875".to_owned(), hours_to_hms(rising.unwrap() as f64))
}

#[test]
fn test_sun_set_in_new_york() {

    // May 16th 2024
    let sun_new_york = SunRiseAndSet {
        doy: 137,
        long: -74.0060,
        lat: 40.7128,
        timezone: -4.0,
    };

    let sma = sun_new_york.sunset_mean_anomaly();
    let stl = sun_new_york.sunset_true_long_in_deg();
    let ra = sun_new_york.sunset_ra_in_hours();
    let dec = sun_new_york.sunset_declination();
    let lha = sun_new_york.sunset_local_ha_in_deg();

    let setting = sun_new_york.sunset_time();

   // dbg!(sma); dbg!(stl); dbg!(ra); dbg!(dec); dbg!(lha.unwrap()); dbg!(setting.unwrap());

    assert_eq!(132.68001, sma);
    assert_eq!(56.702637, stl);
    assert_eq!(3.6270912, ra);
    assert_eq!(19.42125, dec);
    assert_eq!(7.25926, lha.unwrap());
    assert_eq!(20.133024, *setting.as_ref().unwrap());
    assert_eq!("20:7:58.887176513671875".to_owned(), hours_to_hms(setting.unwrap() as f64))
}

#[test]
fn test_sun_set_in_new_york_using_setters() {

    // May 16th 2024
    let sun_new_york = SunRiseAndSet::new()
                        .date(2024, 05, 16)
                        .long(-74.0060)
                        .lat(40.7128)
                        .timezone(-4.0);

    let sma = sun_new_york.sunset_mean_anomaly();
    let stl = sun_new_york.sunset_true_long_in_deg();
    let ra = sun_new_york.sunset_ra_in_hours();
    let dec = sun_new_york.sunset_declination();
    let lha = sun_new_york.sunset_local_ha_in_deg();

    let setting = sun_new_york.sunset_time();

   // dbg!(sma); dbg!(stl); dbg!(ra); dbg!(dec); dbg!(lha.unwrap()); dbg!(setting.unwrap());

    assert_eq!(132.68001, sma);
    assert_eq!(56.702637, stl);
    assert_eq!(3.6270912, ra);
    assert_eq!(19.42125, dec);
    assert_eq!(7.25926, lha.unwrap());
    assert_eq!(20.133024, *setting.as_ref().unwrap());
    assert_eq!("20:7:58.887176513671875".to_owned(), hours_to_hms(setting.unwrap() as f64))
}