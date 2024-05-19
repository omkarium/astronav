use astronav::{coords::{deg_to_dms, deg_to_hms, dms_to_deg, hours_to_hms, hours_to_hms_tuple}, time::*};

#[test]
fn test_time_methods() {
    let time = AstroTime { day: 12, month: 5, year: 2024, hour: 17, min: 30, sec: 45 };
    assert_eq!(2460443, time.julian_day_number());
    assert_eq!(2460443.2296875003, time.julian_time());
    assert_eq!(133.6647976222448, time.gmst_in_degrees());
    assert_eq!(146.1147976222448, time.lmst_in_degrees(12.45));
    assert_eq!(9.740987, time.lmst_in_decimal_hours(12.45));
    assert_eq!(133, time.day_of_year());


}

#[test]
fn test_time_functions() {
    assert_eq!(2460443, julian_day_number(12,5,2024));
    assert_eq!(2460443.2296875003, julian_time(2460443,17,30,45));
    assert_eq!(133.6647976222448, gmst_in_degrees(2460443.2296875003));
    assert_eq!(146.1147976222448, lmst_in_degrees(133.6647976222448,12.45));

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