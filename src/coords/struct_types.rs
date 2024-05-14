// Copyright (c) 2024 Venkatesh Omkaram

#[derive(Default, Clone)]
pub struct NoDec;

#[derive(Default, Clone)]
pub struct Dec(pub f64);

#[derive(Default, Clone)]
pub struct NoLat;

#[derive(Default, Clone)]
pub struct Lat(pub f64);

#[derive(Default, Clone)]
pub struct NoLst;

#[derive(Default, Clone)]
pub struct Lst(pub f64);

#[derive(Default, Clone)]
pub struct NoRA;

#[derive(Default, Clone)]
pub struct RA(pub f64);

#[derive(Default, Clone)]
pub struct Sealed;

#[derive(Default, Clone)]
pub struct NotSealed;