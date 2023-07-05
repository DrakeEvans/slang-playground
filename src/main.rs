use slang_solidity::syntax::{
    nodes::{Node, RuleKind, TokenKind},
    parser::ProductionKind,
};

use slang_solidity::language::Language;

use semver::Version;

use serde_json;

fn main() {
    let language = Language::new(Version::parse("0.8.0").unwrap()).unwrap();
    let parse_output = language.parse(ProductionKind::ContractDefinition, "contract Foo {}").unwrap();
    let parse_tree = parse_output.parse_tree().unwrap();
    let json = serde_json::to_string_pretty(&parse_tree).unwrap();

    println!("{}", json);

    let children = match parse_tree.as_ref() {
        Node::Rule { kind, children, .. } => {
            assert_eq!(*kind, RuleKind::ContractDefinition);
            children
        }
        _ => {
            panic!("Unexpected parse_tree");
        }
    };

    assert_eq!(children.len(), 4);

    assert!(
        matches!(children[0].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::ContractKeyword)
    );
    assert!(
        matches!(children[1].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::Identifier)
    );
    assert!(
        matches!(children[2].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::OpenBrace)
    );
    assert!(
        matches!(children[3].as_ref(), Node::Token { kind, .. } if *kind == TokenKind::CloseBrace)
    );
}
