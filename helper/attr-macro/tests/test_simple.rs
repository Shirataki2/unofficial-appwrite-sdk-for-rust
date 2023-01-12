use attr_macro::AppWriteModel;

#[derive(serde::Serialize, AppWriteModel)]
pub struct Data {
    #[attr(default = "1", max = "100")]
    pub integer: i32,
    pub string: String,
    #[attr(max = "32")]
    pub small_string: String,
    #[attr(parse = "Ip")]
    pub ip: String,
    pub boolean: bool,
    pub float: f32,
    pub double: f64,
    pub vec: Vec<i32>,
}

#[test]
fn test_simple() {
    let def = Data::get_attribute_definitions();
    println!("{:#?}", def);
}
