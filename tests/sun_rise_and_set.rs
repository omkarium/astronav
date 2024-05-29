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
        "5:37:19.05487".to_owned(),
        hours_to_hms(rising.unwrap())
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
        "20:7:58.887177".to_owned(),
        hours_to_hms(setting.unwrap())
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
        "20:7:58.887177".to_owned(),
        hours_to_hms(setting.unwrap())
    )
}

#[test]
fn test_day_length_new_york() {
    // May 16th 2024
    let sun_new_york = SunRiseAndSet::new()
        .date(2024, 05, 16)
        .long(-74.0060)
        .lat(40.7128)
        .timezone(-4.0);

    let day_length = sun_new_york.day_length();
    assert_eq!(
        "14:30:39.832306".to_owned(),
        hours_to_hms(day_length.unwrap())
    )

}


#[cfg(feature = "noaa-sun")]
mod noaa_sun {
    use astronav::coords::{deg_to_hms, hours_to_hms, noaa_sun::{eot_in_mins_2, NOAASun}};


    #[test]
    fn test_frac_year() {

        // Test Sun rise, Sun set and other things for Chennai, India
        // May 16th, doy 137
        let chennai_sun = NOAASun {
            year: 2024,
            doy: 137,
            long: 80.2705,
            lat: 13.0843,
            timezone: 5.5,
            hour: 13,
            min: 08,
            sec: 47,
        };

        let fy = chennai_sun.frac_year_by_hour_in_rads();
        let eot = chennai_sun.eot_in_mins();
        let dec = chennai_sun.declination();
        let ha = chennai_sun.ha_in_deg();
        let ra = chennai_sun.ra_in_deg();
        let sza = chennai_sun.zenith_in_deg();
        let alt = chennai_sun.altitude_in_deg();
        let saa = chennai_sun.azimuth_in_deg();
        let sun_rise = chennai_sun.sunrise_time_hours();
        let sun_rise_mins = chennai_sun.sunrise_time_mins();
        let sun_noon = chennai_sun.noon_hours();
        let sun_noon_mins = chennai_sun.noon_mins();
        let sun_set = chennai_sun.sunset_time_hours();
        let sun_set_mins = chennai_sun.sunset_time_mins();
        let day_length = chennai_sun.day_length();

        assert_eq!(2.3354508228530677, fy);
        assert_eq!(3.6061869394507577, eot);
        assert_eq!(19.251991, dec);
        assert_eq!(15.672046917968146, ha);
        assert_eq!("1:2:41.291313".to_owned(), deg_to_hms(ha as f32));
        assert_eq!(54.1181633537517, ra);
        assert_eq!("3:36:28.358917".to_owned(), deg_to_hms(ra as f32));        
        assert_eq!(16.254976645690405, sza);
        assert_eq!(73.7450233543096, alt);
        assert_eq!(294.3440866717512, saa);
        assert_eq!("5:43:2.9990387".to_owned(), hours_to_hms(sun_rise as f32));
        assert_eq!(343.0499741423742, sun_rise_mins);
        assert_eq!("12:5:18.709946".to_owned(), hours_to_hms(sun_noon as f32));
        assert_eq!(725.3118123281274, sun_noon_mins);
        assert_eq!("18:27:34.41742".to_owned(), hours_to_hms(sun_set as f32));
        assert_eq!(1107.5736505138805, sun_set_mins);
        assert_eq!(12.742061272858436, day_length);

    }

    #[test]
    fn test_eot() {
        let year = 2024;
        let day = 137; // Example day
        let result = eot_in_mins_2(year, day);
        println!("Equation result: {}", result);
    }
}
