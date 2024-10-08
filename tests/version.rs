/*
 * Copyright (c) Peter Bjorklund. All rights reserved. https://github.com/piot/app-version
 * Licensed under the MIT License. See LICENSE in the project root for license information.
 */
use app_version::Version;
use std::str::FromStr;

#[test]
fn to_string() {
    let x = Version::new(1, 23, 46);
    assert_eq!(x.to_string(), "1.23.46");
}

#[test]
fn from_str_valid() {
    let version_str = "1.2.3";
    let version = Version::from_str(version_str).unwrap();
    assert_eq!(version, Version::new(1, 2, 3));
}

#[test]
fn test_from_tuple() {
    let version_tuple = (2, 1, 3);
    let version = Version::from(version_tuple);
    assert_eq!(version, Version::new(2, 1, 3));
}

const TEST_VERSION: Version = Version::new(0, 1, 2);
const TEST_VERSION2: Version = Version::new(TEST_VERSION.major(), 1, 2);

#[test]
fn test_from_str_invalid_format() {
    let version_str = "1.2";
    let result = Version::from_str(version_str);
    assert!(result.is_err());
    assert_eq!(result.unwrap_err().to_string(), "Invalid version format");
}

#[test]
fn get_version_fields() {
    assert_eq!(TEST_VERSION.minor(), 1);
    assert_eq!(TEST_VERSION2.major(), TEST_VERSION.major())
}

#[test]
fn check_not_compatible() {
    let x = Version::new(1, 23, 46);
    let y = Version::new(2, 23, 46);
    assert!(!x.is_compatible(&y));
}

#[test]
fn check_compatible() {
    let x = Version::new(1, 23, 46);
    let y = Version::new(1, 99, 2495);
    assert!(x.is_compatible(&y));
}
