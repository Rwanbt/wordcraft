//! Independent bit-level contract tests based on the documented MS-ODRAW flags.
//! Deliberately build the raw six-byte FOPT records without using the parser to
//! construct expected values.
use crate::officeart::Props;

fn fopt_property(pid: u16, value: u32) -> [u8; 6] {
    let mut bytes = [0u8; 6];
    bytes[..2].copy_from_slice(&pid.to_le_bytes());
    bytes[2..].copy_from_slice(&value.to_le_bytes());
    bytes
}

#[test]
fn filled_value_is_ignored_without_its_use_bit() {
    let mut props = Props::default();
    props.apply(&fopt_property(0x01BF, 1 << 4), 1);
    assert_eq!(props.filled, None);
    props.apply(&fopt_property(0x01BF, (1 << 20) | (1 << 4)), 1);
    assert_eq!(props.filled, Some(true));
    props.apply(&fopt_property(0x01BF, 1 << 20), 1);
    assert_eq!(props.filled, Some(false));
    props.apply(&fopt_property(0x01BF, 1 << 4), 1);
    assert_eq!(props.filled, Some(false), "an unspecified flag must not replace the previous value");
}

#[test]
fn line_value_is_ignored_without_its_use_bit() {
    let mut props = Props::default();
    props.apply(&fopt_property(0x01FF, 1 << 3), 1);
    assert_eq!(props.line, None);
    props.apply(&fopt_property(0x01FF, (1 << 19) | (1 << 3)), 1);
    assert_eq!(props.line, Some(true));
    props.apply(&fopt_property(0x01FF, 1 << 19), 1);
    assert_eq!(props.line, Some(false));
    props.apply(&fopt_property(0x01FF, 1 << 3), 1);
    assert_eq!(props.line, Some(false));
}

#[test]
fn truncated_and_overdeclared_fopt_tables_do_not_apply_partial_flags() {
    let complete = fopt_property(0x01BF, (1 << 20) | (1 << 4));
    for len in 0..6 {
        let mut props = Props::default();
        props.apply(&complete[..len], 1);
        assert_eq!(props.filled, None, "unexpected property from {len} bytes");
    }
    let mut props = Props::default();
    props.apply(&complete, u16::MAX);
    assert_eq!(props.filled, Some(true));
    assert_eq!(props.line, None);
}

#[test]
fn independent_fill_and_line_properties_can_be_overridden_in_order() {
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&fopt_property(0x01BF, (1 << 20) | (1 << 4)));
    bytes.extend_from_slice(&fopt_property(0x01FF, 1 << 19));
    bytes.extend_from_slice(&fopt_property(0x01BF, 1 << 20));
    let mut props = Props::default();
    props.apply(&bytes, 3);
    assert_eq!(props.filled, Some(false));
    assert_eq!(props.line, Some(false));
}
