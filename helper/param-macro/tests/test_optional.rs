use param_macro::SerializeParams;

#[derive(SerializeParams)]
struct Test {
    pub code: Option<u16>,
    pub message: Option<String>,
    pub error_type: String,
    pub version: String,
}

#[test]
fn test() {
    let test = Test {
        code: None,
        message: Some("OK".to_string()),
        error_type: "".to_string(),
        version: "".to_string(),
    };
    let expected = vec![
        ("message".to_string(), "OK".to_string()),
        ("error_type".to_string(), "".to_string()),
        ("version".to_string(), "".to_string()),
    ];
    assert_eq!(expected, test.serialize_params());
}
