use astronav::coords::{DMSToDecimalDeg, HMSToDecimalDeg, star::AltAzBuilder};

#[test]
fn test_decimal_inputs() {
    // Fomalhaut
    let alt = AltAzBuilder::new()
        .dec(-29.4925)
        .lat(12.45)
        .lmst(27.15)
        .ra(344.745)
        .seal()
        .build();

    assert_eq!(31.430612305028138, alt.get_altitude());
    assert_eq!(223.46562682045789, alt.get_azimuth());

    // Sirius
    let alt = AltAzBuilder::new()
        .dec(-16.75122)
        .lat(12.45)
        .lmst(199.05)
        .ra(101.5504)
        .seal()
        .build();

    assert_eq!(-10.613191752481162, alt.get_altitude());
    assert_eq!(254.99375998808006, alt.get_azimuth());

    // Antares
    let alt = AltAzBuilder::new()
        .dec(-26.4866)
        .lat(12.45)
        .lmst(200.875)
        .ra(247.73)
        .seal()
        .build();

    assert_eq!(30.101068424513866, alt.get_altitude());
    assert_eq!(130.98869628774506, alt.get_azimuth());
}

#[test]
fn test_non_decimal_inputs() {
    // Antares
    let alt = AltAzBuilder::new()
        .dec(DMSToDecimalDeg("-26:29:11.8").try_into().unwrap())
        .lat(DMSToDecimalDeg("12:27:0").try_into().unwrap())
        .lmst(HMSToDecimalDeg("13:23:30").try_into().unwrap())
        .ra(HMSToDecimalDeg("16:30:55.2").try_into().unwrap())
        .seal()
        .build();

    assert_eq!(30.10106212143597, alt.get_altitude());
    assert_eq!(130.98870686438966, alt.get_azimuth());
}

