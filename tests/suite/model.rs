use serde_json;

#[test]
fn top_level_dotted_keys() {
    do_test(r#"
name = "Orange"
physical.color = "orange"
physical.shape = "round"
site."google.com" = true
"#, r#"
{
  "name": "Orange",
  "physical": {
    "color": "orange",
    "shape": "round"
  },
  "site": {
    "google.com": true
  }
}
"#);
}

#[test]
fn table_dotted_key() {
    do_test(r#"
[dog."tater.man"]
type.name = "pug"
"#, r#"
{ "dog": { "tater.man": { "type": { "name": "pug" } } } }
"#);
}

fn do_test(toml: &str, json: &str) {
    let doc = ::toml(toml);
    let model = doc.model();
    let actual = model.to_string();
    let actual: serde_json::Value = serde_json::from_str(&actual).unwrap();
    let expected: serde_json::Value = serde_json::from_str(&json).unwrap();
    assert_eq!(actual, expected);
}
