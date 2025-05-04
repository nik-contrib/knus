use ferrishot_knus::span::Span;
use ferrishot_knus::traits::Decode;

#[derive(ferrishot_knus_derive::Decode, Debug)]
#[ferrishot_knus(span_type=ferrishot_knus::span::Span)]
struct AstChildren {
    #[ferrishot_knus(children)]
    children: Vec<ferrishot_knus::ast::SpannedNode<Span>>,
}

fn parse<T: Decode<Span>>(text: &str) -> T {
    let mut nodes: Vec<T> = ferrishot_knus::parse("<test>", text).unwrap();
    assert_eq!(nodes.len(), 1);
    nodes.remove(0)
}

#[test]
fn parse_node_span() {
    let item = parse::<AstChildren>(r#"node {a; b;}"#);
    assert_eq!(item.children.len(), 2);
}
