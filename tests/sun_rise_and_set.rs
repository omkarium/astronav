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
    assert_eq!(
        "5:37:19.05487060546875".to_owned(),
        hours_to_hms(rising.unwrap() as f64)
    )
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
    assert_eq!(
        "20:7:58.887176513671875".to_owned(),
        hours_to_hms(setting.unwrap() as f64)
    )
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
    assert_eq!(
        "20:7:58.887176513671875".to_owned(),
        hours_to_hms(setting.unwrap() as f64)
    )
}

#[cfg(feature = "noaa-sun")]
mod noaa_sun {
    use astronav::coords::{hours_to_hms, noaa_sun::{equation_of_time, NOAASun}};


    #[test]
    fn test_frac_year() {

        // Test Sun rise, Sun set and other things for Chennai, India
        let chennai_sun = NOAASun {
            year: 2024,
            doy: 138,
            long: 80.2705,
            lat: 13.0843,
            timezone: 5.5,
            hour: 13,
            min: 08,
            sec: 47,
        };

        let fy = chennai_sun.frac_year_by_hour_in_rads();
        let eot = chennai_sun.true_eot_in_mins();
        let dec = chennai_sun.declination();
        let ha = chennai_sun.ha_pos_time_in_deg();
        let sza = chennai_sun.zenith_in_deg();
        let alt = chennai_sun.altitude_in_deg();
        let saa = chennai_sun.azimuth_in_deg();
        let sun_rise = chennai_sun.sunrise_time_hours();
        let sun_rise_mins = chennai_sun.sunrise_time_mins();
        let sun_noon = chennai_sun.noon_hours();
        let sun_noon_mins = chennai_sun.noon_mins();
        let sun_set: f64 = chennai_sun.sunset_time_hours();
        let sun_set_mins: f64 = chennai_sun.sunset_time_mins();

        assert_eq!(2.352617995823504, fy);
        assert_eq!(3.8842598773463117, eot);
        assert_eq!(19.2872916085781, dec);
        assert_eq!(15.741565152442035, ha);
        assert_eq!(16.3319240544742, sza);
        assert_eq!(73.6680759455258, alt);
        assert_eq!(294.4139960879158, saa);
        assert_eq!("5:42:43.974809411752176".to_owned(), hours_to_hms(sun_rise));
        assert_eq!(342.73291349019587, sun_rise_mins);
        assert_eq!("12:5:1.952365544096324".to_owned(), hours_to_hms(sun_noon));
        assert_eq!(725.032539425735, sun_noon_mins);
        assert_eq!("18:27:19.929921676427682".to_owned(), hours_to_hms(sun_set));
        assert_eq!(1107.3321653612738, sun_set_mins);
    }

    #[test]
    fn test_eot() {
        let year = 2024.0;
        let day = 137.0; // Example day
        let result = equation_of_time(year, day);
        println!("Equation result: {}", result);
    }
}
