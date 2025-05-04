use std::fmt;

use ferrishot_knus::Decode;
use ferrishot_knus::span::Span;
use miette::Diagnostic;

#[derive(ferrishot_knus::DecodeScalar, Debug, PartialEq)]
enum SomeScalar {
    First,
    AnotherOption,
}

#[derive(ferrishot_knus::Decode, Debug, PartialEq)]
struct Item {
    #[ferrishot_knus(argument)]
    value: SomeScalar,
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

#[test]
fn parse_some_scalar() {
    assert_eq!(
        parse::<Item>(r#"node "first""#),
        Item {
            value: SomeScalar::First
        }
    );
    assert_eq!(
        parse::<Item>(r#"node "another-option""#),
        Item {
            value: SomeScalar::AnotherOption
        }
    );
    assert_eq!(
        parse_err::<Item>(r#"node "test""#),
        "expected one of `first`, `another-option`"
    );
}
