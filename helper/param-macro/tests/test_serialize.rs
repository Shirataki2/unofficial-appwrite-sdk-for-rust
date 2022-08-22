use param_macro::SerializeParams;

#[derive(SerializeParams)]
struct Test {
    pub code: u16,
    pub message: String,
    pub error_type: String,
    pub version: String,
}

#[test]
fn test() {
    let test = Test {
        code: 200,
        message: "OK".to_string(),
        error_type: "".to_string(),
        version: "".to_string(),
    };
    let expected = vec![
        ("code".to_string(), "200".to_string()),
        ("message".to_string(), "OK".to_string()),
        ("error_type".to_string(), "".to_string()),
        ("version".to_string(), "".to_string()),
    ];
    assert_eq!(expected, test.serialize_params());
}
