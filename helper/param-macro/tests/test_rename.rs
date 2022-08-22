use param_macro::SerializeParams;

#[derive(SerializeParams)]
struct Test {
    #[params(rename = "Code")]
    pub code: u16,
    #[params(rename = "Message")]
    pub message: Option<String>,
    pub error_type: String,
    pub version: String,
}

#[test]
fn test() {
    let test = Test {
        code: 200,
        message: Some("OK".to_string()),
        error_type: "".to_string(),
        version: "".to_string(),
    };
    let expected = vec![
        ("Code".to_string(), "200".to_string()),
        ("Message".to_string(), "OK".to_string()),
        ("error_type".to_string(), "".to_string()),
        ("version".to_string(), "".to_string()),
    ];
    assert_eq!(expected, test.serialize_params());
}
