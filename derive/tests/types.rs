use std::path::PathBuf;

use ferrishot_knus::span::Span;
use ferrishot_knus::traits::DecodeChildren;

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Scalars {
    #[ferrishot_knus(child, unwrap(argument))]
    str: String,
    #[ferrishot_knus(child, unwrap(argument))]
    u64: u64,
    #[ferrishot_knus(child, unwrap(argument))]
    f64: f64,
    #[ferrishot_knus(child, unwrap(argument))]
    path: PathBuf,
    #[ferrishot_knus(child, unwrap(argument))]
    boolean: bool,
}

fn parse<T: DecodeChildren<Span>>(text: &str) -> T {
    ferrishot_knus::parse("<test>", text).unwrap()
}

#[test]
fn parse_enum() {
    assert_eq!(
        parse::<Scalars>(
            r#"
            str "hello"
            u64 1234
            f64 16.125e+1
            path "/hello/world"
            boolean true
        "#
        ),
        Scalars {
            str: "hello".into(),
            u64: 1234,
            f64: 161.25,
            path: PathBuf::from("/hello/world"),
            boolean: true,
        }
    );
}
