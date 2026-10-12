//! Black-box contract for names used to locate local control-channel secrets.
//! Tests use synthetic strings and never inspect actual credentials.
use std::path::Path;
use wordcraft_control_key::parse_instance_id;

#[test]
fn accepts_only_portable_single_component_identifiers() {
    for valid in ["a", "A09", "flatpak_12-3", "0"] {
        assert_eq!(
            parse_instance_id(&format!("[Instance]\ninstance-id={valid}\n")),
            Some(valid.to_owned())
        );
    }
    for invalid in [
        "", ".", "..", "../secrets", "a/b", "a\\b", "space name", "é", "a:b", "a.b", "a\0b",
    ] {
        assert_eq!(
            parse_instance_id(&format!("instance-id={invalid}\n")),
            None,
            "accepted unsafe instance: {invalid:?}"
        );
    }
}

#[test]
fn rejects_first_unsafe_value_even_when_later_value_is_safe() {
    assert_eq!(
        parse_instance_id("instance-id=../unsafe\ninstance-id=safe\n"),
        None
    );
    assert_eq!(
        parse_instance_id("[Instance]\ninstance-id=instance_1\n"),
        Some("instance_1".to_string())
    );
    assert_eq!(parse_instance_id("unrelated=true\n"), None);
}

#[test]
fn supplied_instance_id_cannot_change_a_parent_directory() {
    let base = Path::new("workspace").join("settings");
    for candidate in ["safe", "tenant_123", "alpha-beta"] {
        let id = parse_instance_id(&format!("instance-id={candidate}"))
            .expect("test input is valid");
        let path = base.join(format!("control-key.{id}"));
        assert_eq!(path.parent(), Some(base.as_path()));
        assert_eq!(
            path.file_name().and_then(|s| s.to_str()),
            Some(format!("control-key.{candidate}").as_str())
        );
    }
}
