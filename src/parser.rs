// TODO: Filters
// TODO: Error type for parse errors, use Result and not Option
//
// pub struct Filter {
//     field: String,
//     value: String,
// }

pub struct SQLStatement {
    pub table: String,
    pub fields: Vec<String>,
    // filters: Vec<Filter>,
}

pub fn parse (mut input : String) -> Option<SQLStatement> {
    input = input.to_lowercase();

    let from_idx = input.find("from");
    if !input.starts_with("select") || from_idx.is_none() {
        return None;
    }

    let mut without_select = input.trim_left_matches("select ").to_string();
    let (fields_input, rest) = without_select.split_at_mut(from_idx.unwrap());
    let fields = fields_input
        .split(",")
        .map(|field| field.trim().to_string())
        .collect::<Vec<String>>();
    let table = rest.trim_left_matches("from ").to_string();

    let result : SQLStatement = SQLStatement {
        table: table,
        fields: fields,
        // filters: vec![],
    };
    Some(result)
}
