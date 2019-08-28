use serde_tuple::*;

use std::borrow::Cow;

#[derive(Debug, Serialize_tuple, Deserialize_tuple)]
#[serde(rename = "FOO")]
#[serde(deny_unknown_fields)]
pub struct Foo<'a, T: serde::Serialize + serde::de::DeserializeOwned> {
    string: &'a str,
    #[serde(skip)]
    baz: i32,
    other: T,
}

#[derive(Debug, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct Bar {
    count: i32,
}

fn main() {
    let foo = Foo { string: "Yes".into(), baz: 22, other: Bar { count: 3 } };

    let json = serde_json::to_string_pretty(&foo).unwrap();

    println!("{}", &json);

    let foo = serde_json::from_str::<Foo<Bar>>(&json).unwrap();

    println!("{:?}", &foo);
}
