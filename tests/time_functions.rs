use astronav::{coords::{deg_to_dms, deg_to_hms, dms_to_deg, hours_to_hms, hours_to_hms_tuple}, time::*};

#[test]
fn test_time_methods() {
    // New york
    let time = AstroTime { day: 12, month: 5, year: 2024, hour: 17, min: 30, sec: 45, timezone: -4.0 };
    assert_eq!(2460443, time.julian_day_number());
    assert_eq!(2460443.3972106483, time.julian_time());
    assert_eq!(194.13824965432286, time.gmst_in_degrees());
    assert_eq!(120.13224965432286, time.lmst_in_degrees(-74.0060));
    assert_eq!(8.008817, time.lmst_in_decimal_hours(-74.0060));
    assert_eq!(133, time.day_of_year());
}

#[test]
fn test_time_methods_2() {
    let time = AstroTime { day: 16, month: 5, year: 2024, hour: 13, min: 08, sec: 47, timezone: 5.5 };
    assert_eq!(2460447, time.julian_day_number());
    assert_eq!(2460446.8194560185, time.julian_time());
    assert_eq!(349.5197100886144, time.gmst_in_degrees());
    assert_eq!(69.79021008861434, time.lmst_in_degrees(80.2705));
    assert_eq!(4.652681, time.lmst_in_decimal_hours(80.2705));
    assert_eq!(137, time.day_of_year());

}

#[test]
fn test_time_functions() {
    assert_eq!(2460443, julian_day_number(12,5,2024));
    assert_eq!(2460443.0013773153, julian_time(2460443,17,30,45, 5.5));
    assert_eq!(51.248097681906074, gmst_in_degrees(2460443.0013773153));
    assert_eq!(65.69809768190608, lmst_in_degrees(51.248097681906074,14.45));

}

#[test]
fn test_non_decimal_inputs_with_error() {
    assert_eq!(
        true,
        dms_to_deg("-26-29:11.8").is_err()
    );
}

#[test]
fn test_decimal_inputs() {
    assert_eq!("-66:30:16.082153",deg_to_dms(-65.4878));
    assert_eq!("12:29:16.07872",hours_to_hms(12.4878));
    assert_eq!((5,37,19.05487), hours_to_hms_tuple(5.6219597));
    assert_eq!("0:21:1.079979".to_owned(), deg_to_hms(5.2545));
    assert_eq!("14:19:59.998856".to_owned(), deg_to_hms(215.0));

}