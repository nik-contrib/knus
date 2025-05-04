use std::fmt;

use miette::Diagnostic;

use ferrishot_knus::traits::DecodeChildren;
use ferrishot_knus::{span::Span, Decode};

#[derive(ferrishot_knus_derive::Decode, Default, Debug, PartialEq)]
struct Prop1 {
    #[ferrishot_knus(property)]
    label: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct FlatProp {
    #[ferrishot_knus(flatten(property))]
    props: Prop1,
}

#[derive(ferrishot_knus_derive::Decode, Default, Debug, PartialEq)]
struct Unwrap {
    #[ferrishot_knus(child, unwrap(argument))]
    label: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct FlatChild {
    #[ferrishot_knus(flatten(child))]
    children: Unwrap,
}

fn parse<T: Decode<Span>>(text: &str) -> T {
    let mut nodes: Vec<T> = ferrishot_knus::parse("<test>", text).unwrap();
    assert_eq!(nodes.len(), 1);
    nodes.remove(0)
}

fn parse_err<T: Decode<Span> + fmt::Debug>(text: &str) -> String {
    let err = ferrishot_knus::parse::<Vec<T>>("<test>", text).unwrap_err();
    err.related()
        .unwrap()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

fn parse_doc<T: DecodeChildren<Span>>(text: &str) -> T {
    ferrishot_knus::parse("<test>", text).unwrap()
}

fn parse_doc_err<T: DecodeChildren<Span> + fmt::Debug>(text: &str) -> String {
    let err = ferrishot_knus::parse::<T>("<test>", text).unwrap_err();
    err.related()
        .unwrap()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join("\n")
}

#[test]
fn parse_flat_prop() {
    assert_eq!(
        parse::<FlatProp>(r#"node label="hello""#),
        FlatProp {
            props: Prop1 {
                label: Some("hello".into())
            }
        }
    );
    assert_eq!(
        parse_err::<FlatProp>(r#"node something="world""#),
        "unexpected property `something`"
    );
}

#[test]
fn parse_flat_child() {
    assert_eq!(
        parse_doc::<FlatChild>(r#"label "hello""#),
        FlatChild {
            children: Unwrap {
                label: Some("hello".into())
            }
        }
    );
    assert_eq!(
        parse_doc_err::<FlatChild>(r#"something "world""#),
        "unexpected node `something`"
    );
}
