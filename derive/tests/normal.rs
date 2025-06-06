use std::collections::BTreeMap;
use std::default::Default;
use std::fmt;

use miette::Diagnostic;

use ferrishot_knus::traits::DecodeChildren;
use ferrishot_knus::{Decode, span::Span};

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Arg1 {
    #[ferrishot_knus(argument)]
    name: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Arg1RawIdent {
    #[ferrishot_knus(argument)]
    r#type: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ArgDef {
    #[ferrishot_knus(argument, default)]
    name: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ArgDefValue {
    #[ferrishot_knus(argument, default="unnamed".into())]
    name: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ArgDefOptValue {
    #[ferrishot_knus(argument, default=Some("unnamed".into()))]
    name: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct OptArg {
    #[ferrishot_knus(argument)]
    name: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Extra {
    field: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct VarArg {
    #[ferrishot_knus(arguments)]
    params: Vec<u64>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq, Default)]
struct Prop1 {
    #[ferrishot_knus(property)]
    label: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq, Default)]
struct Prop1RawIdent {
    #[ferrishot_knus(property)]
    r#type: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct PropDef {
    #[ferrishot_knus(property, default)]
    label: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct PropDefValue {
    #[ferrishot_knus(property, default="unknown".into())]
    label: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct PropDefOptValue {
    #[ferrishot_knus(property, default=Some("unknown".into()))]
    label: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct PropNamed {
    #[ferrishot_knus(property(name = "x"))]
    label: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct OptProp {
    #[ferrishot_knus(property)]
    label: Option<String>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct VarProp {
    #[ferrishot_knus(properties)]
    scores: BTreeMap<String, u64>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Children {
    #[ferrishot_knus(children)]
    children: Vec<Arg1>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct FilteredChildren {
    #[ferrishot_knus(children(name = "left"))]
    left: Vec<OptArg>,
    #[ferrishot_knus(children(name = "right"))]
    right: Vec<OptArg>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
enum Variant {
    Arg1(Arg1),
    Prop1(Prop1),
    #[ferrishot_knus(skip)]
    #[allow(dead_code)]
    Var3(u32),
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Child {
    #[ferrishot_knus(child)]
    main: Prop1,
    #[ferrishot_knus(child)]
    extra: Option<Prop1>,
    #[ferrishot_knus(child)]
    flag: bool,
    #[ferrishot_knus(child)]
    унікод: Option<Prop1>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ChildDef {
    #[ferrishot_knus(child, default)]
    main: Prop1,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ChildDefValue {
    #[ferrishot_knus(child, default=Prop1 { label: String::from("prop1") })]
    main: Prop1,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Unwrap {
    #[ferrishot_knus(child, unwrap(argument))]
    label: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct UnwrapRawIdent {
    #[ferrishot_knus(child, unwrap(argument))]
    r#type: String,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct UnwrapFiltChildren {
    #[ferrishot_knus(children(name = "labels"), unwrap(arguments))]
    labels: Vec<Vec<String>>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct UnwrapChildren {
    #[ferrishot_knus(children, unwrap(arguments))]
    labels: Vec<Vec<String>>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Parse {
    #[ferrishot_knus(child, unwrap(argument, str))]
    listen: std::net::SocketAddr,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct ParseOpt {
    #[ferrishot_knus(property, str)]
    listen: Option<std::net::SocketAddr>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct Bytes {
    #[ferrishot_knus(child, unwrap(argument, bytes))]
    data: Vec<u8>,
}

#[derive(ferrishot_knus_derive::Decode, Debug, PartialEq)]
struct OptBytes {
    #[ferrishot_knus(property, bytes)]
    data: Option<Vec<u8>>,
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
fn parse_arg1() {
    assert_eq!(
        parse::<Arg1>(r#"node "hello""#),
        Arg1 {
            name: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<Arg1>(r#"node "hello" "world""#),
        "unexpected argument"
    );
    assert_eq!(
        parse_err::<Arg1>(r#"(some)node "hello""#),
        "no type name expected for this node"
    );
    assert_eq!(
        parse_err::<Arg1>(r#"node"#),
        "additional argument `name` is required"
    );
}

#[test]
fn parse_arg1_raw_ident() {
    assert_eq!(
        parse::<Arg1RawIdent>(r#"node "hello""#),
        Arg1RawIdent {
            r#type: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<Arg1RawIdent>(r#"node "hello" "world""#),
        "unexpected argument"
    );
    assert_eq!(
        parse_err::<Arg1RawIdent>(r#"(some)node "hello""#),
        "no type name expected for this node"
    );
    assert_eq!(
        parse_err::<Arg1RawIdent>(r#"node"#),
        "additional argument `type` is required"
    );
}

#[test]
fn parse_arg_default() {
    assert_eq!(
        parse::<ArgDef>(r#"node "hello""#),
        ArgDef {
            name: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<ArgDef>(r#"node "hello" "world""#),
        "unexpected argument"
    );
    assert_eq!(parse::<ArgDef>(r#"node"#), ArgDef { name: "".into() });
}

#[test]
fn parse_arg_def_value() {
    assert_eq!(
        parse::<ArgDefValue>(r#"node "hello""#),
        ArgDefValue {
            name: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<ArgDefValue>(r#"node "hello" "world""#),
        "unexpected argument"
    );
    assert_eq!(
        parse::<ArgDefValue>(r#"node"#),
        ArgDefValue {
            name: "unnamed".into()
        }
    );

    assert_eq!(
        parse::<ArgDefOptValue>(r#"node "hello""#),
        ArgDefOptValue {
            name: Some("hello".into())
        }
    );
    assert_eq!(
        parse_err::<ArgDefValue>(r#"node "hello" "world""#),
        "unexpected argument"
    );
    assert_eq!(
        parse::<ArgDefOptValue>(r#"node"#),
        ArgDefOptValue {
            name: Some("unnamed".into())
        }
    );
    assert_eq!(
        parse::<ArgDefOptValue>(r#"node null"#),
        ArgDefOptValue { name: None }
    );
}

#[test]
fn parse_opt_arg() {
    assert_eq!(
        parse::<OptArg>(r#"node "hello""#),
        OptArg {
            name: Some("hello".into())
        }
    );
    assert_eq!(parse::<OptArg>(r#"node"#), OptArg { name: None });
    assert_eq!(parse::<OptArg>(r#"node null"#), OptArg { name: None });
}

#[test]
fn parse_prop() {
    assert_eq!(
        parse::<Prop1>(r#"node label="hello""#),
        Prop1 {
            label: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<Prop1>(r#"node label="hello" y="world""#),
        "unexpected property `y`"
    );
    assert_eq!(
        parse_err::<Prop1>(r#"node"#),
        "property `label` is required"
    );
}

#[test]
fn parse_prop_raw_ident() {
    assert_eq!(
        parse::<Prop1RawIdent>(r#"node type="hello""#),
        Prop1RawIdent {
            r#type: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<Prop1RawIdent>(r#"node type="hello" y="world""#),
        "unexpected property `y`"
    );
    assert_eq!(
        parse_err::<Prop1RawIdent>(r#"node"#),
        "property `type` is required"
    );
}

#[test]
fn parse_prop_default() {
    assert_eq!(
        parse::<PropDef>(r#"node label="hello""#),
        PropDef {
            label: "hello".into()
        }
    );
    assert_eq!(parse::<PropDef>(r#"node"#), PropDef { label: "".into() });
}

#[test]
fn parse_prop_def_value() {
    assert_eq!(
        parse::<PropDefValue>(r#"node label="hello""#),
        PropDefValue {
            label: "hello".into()
        }
    );
    assert_eq!(
        parse::<PropDefValue>(r#"node"#),
        PropDefValue {
            label: "unknown".into()
        }
    );

    assert_eq!(
        parse::<PropDefOptValue>(r#"node label="hello""#),
        PropDefOptValue {
            label: Some("hello".into())
        }
    );
    assert_eq!(
        parse::<PropDefOptValue>(r#"node"#),
        PropDefOptValue {
            label: Some("unknown".into())
        }
    );
    assert_eq!(
        parse::<PropDefOptValue>(r#"node label=null"#),
        PropDefOptValue { label: None }
    );
}

#[test]
fn parse_prop_named() {
    assert_eq!(
        parse::<PropNamed>(r#"node x="hello""#),
        PropNamed {
            label: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<PropNamed>(r#"node label="hello" y="world""#),
        "unexpected property `label`"
    );
    assert_eq!(
        parse_err::<PropNamed>(r#"node"#),
        "property `x` is required"
    );
}

#[test]
fn parse_unwrap() {
    assert_eq!(
        parse::<Unwrap>(r#"node { label "hello"; }"#),
        Unwrap {
            label: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<Unwrap>(r#"node label="hello""#),
        "unexpected property `label`"
    );
    assert_eq!(
        parse_err::<Unwrap>(r#"node"#),
        "child node `label` is required"
    );
    assert_eq!(
        parse_doc::<Unwrap>(r#"label "hello""#),
        Unwrap {
            label: "hello".into()
        }
    );
}

#[test]
fn parse_unwrap_raw_ident() {
    assert_eq!(
        parse::<UnwrapRawIdent>(r#"node { type "hello"; }"#),
        UnwrapRawIdent {
            r#type: "hello".into()
        }
    );
    assert_eq!(
        parse_err::<UnwrapRawIdent>(r#"node type="hello""#),
        "unexpected property `type`"
    );
    assert_eq!(
        parse_err::<UnwrapRawIdent>(r#"node"#),
        "child node `type` is required"
    );
    assert_eq!(
        parse_doc::<UnwrapRawIdent>(r#"type "hello""#),
        UnwrapRawIdent {
            r#type: "hello".into()
        }
    );
}

#[test]
fn parse_unwrap_filtered_children() {
    assert_eq!(
        parse::<UnwrapFiltChildren>(r#"node { labels "hello" "world"; labels "oh" "my"; }"#),
        UnwrapFiltChildren {
            labels: vec![
                vec!["hello".into(), "world".into()],
                vec!["oh".into(), "my".into()],
            ]
        },
    );
}

#[test]
fn parse_unwrap_children() {
    assert_eq!(
        parse::<UnwrapChildren>(r#"node { some "hello" "world"; other "oh" "my"; }"#),
        UnwrapChildren {
            labels: vec![
                vec!["hello".into(), "world".into()],
                vec!["oh".into(), "my".into()],
            ]
        },
    );
}

#[test]
fn parse_opt_prop() {
    assert_eq!(
        parse::<OptProp>(r#"node label="hello""#),
        OptProp {
            label: Some("hello".into())
        }
    );
    assert_eq!(parse::<OptProp>(r#"node"#), OptProp { label: None });
    assert_eq!(
        parse::<OptProp>(r#"node label=null"#),
        OptProp { label: None }
    );
}

#[test]
fn parse_var_arg() {
    assert_eq!(
        parse::<VarArg>(r#"sum 1 2 3"#),
        VarArg {
            params: vec![1, 2, 3]
        }
    );
    assert_eq!(parse::<VarArg>(r#"sum"#), VarArg { params: vec![] });
}

#[test]
fn parse_var_prop() {
    let mut scores = BTreeMap::new();
    scores.insert("john".into(), 13);
    scores.insert("jack".into(), 7);
    assert_eq!(
        parse::<VarProp>(r#"scores john=13 jack=7"#),
        VarProp { scores }
    );
    assert_eq!(
        parse::<VarProp>(r#"scores"#),
        VarProp {
            scores: BTreeMap::new()
        }
    );
}

#[test]
fn parse_children() {
    assert_eq!(
        parse::<Children>(r#"parent { - "val1"; - "val2"; }"#),
        Children {
            children: vec![
                Arg1 {
                    name: "val1".into()
                },
                Arg1 {
                    name: "val2".into()
                },
            ]
        }
    );
    assert_eq!(
        parse::<Children>(r#"parent"#),
        Children {
            children: Vec::new()
        }
    );

    assert_eq!(
        parse_doc::<Children>(r#"- "val1"; - "val2""#),
        Children {
            children: vec![
                Arg1 {
                    name: "val1".into()
                },
                Arg1 {
                    name: "val2".into()
                },
            ]
        }
    );
    assert_eq!(
        parse_doc::<Children>(r#""#),
        Children {
            children: Vec::new()
        }
    );
}

#[test]
fn parse_filtered_children() {
    assert_eq!(
        parse_doc::<FilteredChildren>(r#"left "v1"; right "v2"; left "v3""#),
        FilteredChildren {
            left: vec![
                OptArg {
                    name: Some("v1".into())
                },
                OptArg {
                    name: Some("v3".into())
                },
            ],
            right: vec![OptArg {
                name: Some("v2".into())
            },]
        }
    );
    assert_eq!(
        parse_doc::<FilteredChildren>(r#"right; left"#),
        FilteredChildren {
            left: vec![OptArg { name: None },],
            right: vec![OptArg { name: None },]
        }
    );
    assert_eq!(
        parse_doc_err::<FilteredChildren>(r#"some"#),
        "unexpected node `some`"
    );
}

#[test]
fn parse_child() {
    assert_eq!(
        parse::<Child>(r#"parent { main label="val1"; }"#),
        Child {
            main: Prop1 {
                label: "val1".into()
            },
            extra: None,
            flag: false,
            унікод: None,
        }
    );
    assert_eq!(
        parse::<Child>(
            r#"parent {
                    main label="primary";
                    extra label="replica";
                 }"#
        ),
        Child {
            main: Prop1 {
                label: "primary".into()
            },
            extra: Some(Prop1 {
                label: "replica".into()
            }),
            flag: false,
            унікод: None,
        }
    );
    assert_eq!(
        parse_err::<Child>(r#"parent { something; }"#),
        "unexpected node `something`\n\
                child node `main` is required"
    );
    assert_eq!(
        parse_err::<Child>(r#"parent"#),
        "child node `main` is required"
    );

    assert_eq!(
        parse_doc::<Child>(r#"main label="val1""#),
        Child {
            main: Prop1 {
                label: "val1".into()
            },
            extra: None,
            flag: false,
            унікод: None,
        }
    );
    assert_eq!(
        parse_doc::<Child>(
            r#"
                    main label="primary"
                    extra label="replica"
                    flag
                 "#
        ),
        Child {
            main: Prop1 {
                label: "primary".into()
            },
            extra: Some(Prop1 {
                label: "replica".into()
            }),
            flag: true,
            унікод: None,
        }
    );
    assert_eq!(
        parse_doc_err::<Child>(r#"something"#),
        "unexpected node `something`\n\
                child node `main` is required"
    );
    assert_eq!(
        parse_doc_err::<Child>(r#""#),
        "child node `main` is required"
    );

    assert_eq!(
        parse_doc::<Child>(
            r#"
                    main label="primary"
                    унікод label="працює"
                 "#
        ),
        Child {
            main: Prop1 {
                label: "primary".into()
            },
            extra: None,
            flag: false,
            унікод: Some(Prop1 {
                label: "працює".into()
            }),
        }
    );
}

#[test]
fn parse_child_def() {
    assert_eq!(
        parse::<ChildDef>(r#"parent { main label="val1"; }"#),
        ChildDef {
            main: Prop1 {
                label: "val1".into()
            },
        }
    );
    assert_eq!(
        parse::<ChildDef>(r#"parent"#),
        ChildDef {
            main: Prop1 { label: "".into() },
        }
    );
}

#[test]
fn parse_child_def_value() {
    assert_eq!(
        parse::<ChildDefValue>(r#"parent { main label="val1"; }"#),
        ChildDefValue {
            main: Prop1 {
                label: "val1".into()
            },
        }
    );
    assert_eq!(
        parse::<ChildDefValue>(r#"parent"#),
        ChildDefValue {
            main: Prop1 {
                label: "prop1".into()
            },
        }
    );
}

#[test]
fn parse_enum() {
    assert_eq!(
        parse::<Variant>(r#"arg1 "hello""#),
        Variant::Arg1(Arg1 {
            name: "hello".into()
        })
    );
    assert_eq!(
        parse::<Variant>(r#"prop1 label="hello""#),
        Variant::Prop1(Prop1 {
            label: "hello".into()
        })
    );
    assert_eq!(
        parse_err::<Variant>(r#"something"#),
        "expected one of `arg1`, `prop1`"
    );
}

#[test]
fn parse_str() {
    assert_eq!(
        parse_doc::<Parse>(r#"listen "127.0.0.1:8080""#),
        Parse {
            listen: "127.0.0.1:8080".parse().unwrap()
        }
    );
    assert!(parse_doc_err::<Parse>(r#"listen "2/3""#).contains("invalid"));

    assert_eq!(
        parse::<ParseOpt>(r#"server listen="127.0.0.1:8080""#),
        ParseOpt {
            listen: Some("127.0.0.1:8080".parse().unwrap())
        }
    );
    assert!(parse_err::<ParseOpt>(r#"server listen="2/3""#).contains("invalid"));
    assert_eq!(
        parse::<ParseOpt>(r#"server listen=null"#),
        ParseOpt { listen: None }
    );
}

#[test]
fn parse_bytes() {
    assert_eq!(
        parse_doc::<Bytes>(r#"data (base64)"aGVsbG8=""#),
        Bytes {
            data: b"hello".to_vec()
        }
    );
    assert_eq!(
        parse_doc::<Bytes>(r#"data "world""#),
        Bytes {
            data: b"world".to_vec()
        }
    );
    assert_eq!(
        parse_doc_err::<Bytes>(r#"data (base64)"2/3""#),
        "Invalid padding"
    );

    assert_eq!(
        parse::<OptBytes>(r#"node data=(base64)"aGVsbG8=""#),
        OptBytes {
            data: Some(b"hello".to_vec())
        }
    );
    assert_eq!(
        parse::<OptBytes>(r#"node data="world""#),
        OptBytes {
            data: Some(b"world".to_vec())
        }
    );
    assert_eq!(
        parse::<OptBytes>(r#"node data=null"#),
        OptBytes { data: None }
    );
}

#[test]
fn parse_extra() {
    assert_eq!(parse::<Extra>(r#"data"#), Extra { field: "".into() });
    assert_eq!(parse_err::<Extra>(r#"data x=1"#), "unexpected property `x`");
}
