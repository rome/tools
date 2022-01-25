//! Generate SyntaxKind definitions as well as typed AST definitions for nodes and tokens.
//! This is derived from rust-analyzer/xtask/codegen

use std::vec;

use super::{
    kinds_src::{AstSrc, Field},
    to_lower_snake_case, Mode,
};
use crate::generate_syntax_factory::generate_syntax_factory;
use crate::kinds_src::{AstListSeparatorConfiguration, AstListSrc, TokenKind};
use crate::{
    generate_macros::generate_macros,
    generate_nodes::generate_nodes,
    generate_syntax_kinds::generate_syntax_kinds,
    kinds_src::{AstEnumSrc, AstNodeSrc, KINDS_SRC},
    update,
};
use ungrammar::{Grammar, Rule, Token};
use xtask::{project_root, Result};

// these node won't generate any code
pub const SYNTAX_ELEMENT_TYPE: &str = "SyntaxElement";

pub fn generate_ast(mode: Mode) -> Result<()> {
    let grammar_src = include_str!("../js.ungram");
    let grammar: Grammar = grammar_src.parse().unwrap();
    let mut ast = make_ast(&grammar);

    ast.sort();

    let ast_nodes_file = project_root().join(crate::AST_NODES);
    let contents = generate_nodes(&ast)?;
    update(ast_nodes_file.as_path(), &contents, mode)?;

    let syntax_kinds_file = project_root().join(crate::SYNTAX_KINDS);
    let contents = generate_syntax_kinds(KINDS_SRC)?;
    update(syntax_kinds_file.as_path(), &contents, mode)?;

    let syntax_factory_file = project_root().join(crate::SYNTAX_FACTORY);
    let contents = generate_syntax_factory(&ast)?;
    update(syntax_factory_file.as_path(), &contents, mode)?;

    let ast_macros_file = project_root().join(crate::AST_MACROS);
    let contents = generate_macros(&ast)?;
    update(ast_macros_file.as_path(), &contents, mode)?;

    Ok(())
}

fn make_ast(grammar: &Grammar) -> AstSrc {
    let mut ast = AstSrc::default();

    for node in grammar.iter() {
        let name = grammar[node].name.clone();
        if name == SYNTAX_ELEMENT_TYPE {
            continue;
        }

        let rule = &grammar[node].rule;

        match classify_node_rule(grammar, rule) {
            NodeRuleClassification::Union(variants) => ast.unions.push(AstEnumSrc {
                documentation: vec![],
                name,
                variants,
            }),
            NodeRuleClassification::Node => {
                let mut fields = vec![];
                handle_rule(&mut fields, grammar, rule, None, false);
                ast.nodes.push(AstNodeSrc {
                    documentation: vec![],
                    name,
                    fields,
                })
            }
            NodeRuleClassification::Unknown => ast.unknowns.push(name),
            NodeRuleClassification::List {
                separator,
                element_name,
            } => {
                ast.push_list(
                    name.as_str(),
                    AstListSrc {
                        element_name,
                        separator,
                    },
                );
            }
        }
    }

    ast
}

/// Classification of a node rule.
/// Determined by matching the top level production of any node.
enum NodeRuleClassification {
    /// Union of the form `A = B | C`
    Union(Vec<String>),
    /// Regular node containing tokens or sub nodes of the form `A = B 'c'
    Node,
    /// An Unknown node of the form `A = SyntaxElement*`
    Unknown,

    /// A list node of the form `A = B*` or `A = (B (',' B)*)` or `A = (B (',' B)* ','?)`
    List {
        /// Name of the nodes stored in this list (`B` in the example above)
        element_name: String,

        /// [None] if this is a node list or [Some] if this is a separated list
        separator: Option<AstListSeparatorConfiguration>,
    },
}

fn classify_node_rule(grammar: &Grammar, rule: &Rule) -> NodeRuleClassification {
    match rule {
        // this is for enums
        Rule::Alt(alternatives) => {
            let mut all_alternatives = vec![];
            for alternative in alternatives {
                match alternative {
                    Rule::Node(it) => all_alternatives.push(grammar[*it].name.clone()),
                    Rule::Token(it) if grammar[*it].name == ";" => (),
                    _ => return NodeRuleClassification::Node,
                }
            }
            NodeRuleClassification::Union(all_alternatives)
        }
        // A*
        Rule::Rep(rule) => {
            let element_type = match rule.as_ref() {
                Rule::Node(node) => &grammar[*node].name,
                _ => {
                    panic!("Lists should only be over node types");
                }
            };

            if element_type == SYNTAX_ELEMENT_TYPE {
                NodeRuleClassification::Unknown
            } else {
                NodeRuleClassification::List {
                    separator: None,
                    element_name: element_type.to_string(),
                }
            }
        }
        Rule::Seq(rules) => {
            // (T (',' T)* ','?)
            // (T (',' T)*)
            if let Some(comma_list) = handle_comma_list(grammar, rules.as_slice()) {
                NodeRuleClassification::List {
                    separator: Some(AstListSeparatorConfiguration {
                        allow_trailing: comma_list.trailing_separator,
                        separator_token: comma_list.separator_name.to_string(),
                    }),
                    element_name: comma_list.node_name.to_string(),
                }
            } else {
                NodeRuleClassification::Node
            }
        }
        _ => NodeRuleClassification::Node,
    }
}

fn clean_token_name(grammar: &Grammar, token: &Token) -> String {
    let mut name = grammar[*token].name.clone();

    // These tokens, when parsed to proc_macro2::TokenStream, generates a stream of bytes
    // that can't be recognized by [quote].
    // Hence, they need to be decorated with single quotes.
    if "[]{}()`".contains(&name) {
        name = format!("'{}'", name);
    }
    name
}

fn handle_rule(
    fields: &mut Vec<Field>,
    grammar: &Grammar,
    rule: &Rule,
    label: Option<&str>,
    optional: bool,
) {
    match rule {
        Rule::Labeled { label, rule } => {
            // Some methods need to be manually implemented because they need some custom logic;
            // we use the prefix "manual__" to exclude labelled nodes.

            if handle_tokens_in_unions(fields, grammar, rule, label, optional) {
                return;
            }

            handle_rule(fields, grammar, rule, Some(label), optional)
        }
        Rule::Node(node) => {
            let ty = grammar[*node].name.clone();
            let name = label
                .map(String::from)
                .unwrap_or_else(|| to_lower_snake_case(&ty));
            let field = Field::Node { name, ty, optional };
            fields.push(field);
        }
        Rule::Token(token) => {
            let name = clean_token_name(grammar, token);

            if name == "''" {
                // array hole
                return;
            }

            let field = Field::Token {
                name: label.map(String::from).unwrap_or_else(|| name.clone()),
                kind: TokenKind::Single(name),
                optional,
            };
            fields.push(field);
        }

        Rule::Rep(_) => {
            panic!("Create a list node for *many* children {:?}", label);
        }
        Rule::Opt(rule) => {
            handle_rule(fields, grammar, rule, label, true);
        }
        Rule::Alt(rules) => {
            for rule in rules {
                handle_rule(fields, grammar, rule, label, false);
            }
        }

        Rule::Seq(rules) => {
            for rule in rules {
                handle_rule(fields, grammar, rule, label, false);
            }
        }
    };
}

struct CommaList<'a> {
    node_name: &'a str,
    separator_name: &'a str,
    trailing_separator: bool,
}

// (T (',' T)* ','?)
// (T (',' T)*)
fn handle_comma_list<'a>(grammar: &'a Grammar, rules: &[Rule]) -> Option<CommaList<'a>> {
    // Does it match (T * ',')?
    let (node, repeat, trailing_separator) = match rules {
        [Rule::Node(node), Rule::Rep(repeat), Rule::Opt(trailing_separator)] => {
            (node, repeat, Some(trailing_separator))
        }
        [Rule::Node(node), Rule::Rep(repeat)] => (node, repeat, None),
        _ => return None,
    };

    // Is the repeat a ()*?
    let repeat = match &**repeat {
        Rule::Seq(it) => it,
        _ => return None,
    };

    // Does the repeat match (token node))
    let comma = match repeat.as_slice() {
        [comma, Rule::Node(n)] => {
            let separator_matches_trailing = if let Some(trailing) = trailing_separator {
                &**trailing == comma
            } else {
                true
            };

            if n != node || !separator_matches_trailing {
                return None;
            }

            comma
        }
        _ => return None,
    };

    let separator_name = match comma {
        Rule::Token(token) => &grammar[*token].name,
        _ => panic!("The separator in rule {:?} must be a token", rules),
    };

    Some(CommaList {
        node_name: &grammar[*node].name,
        trailing_separator: trailing_separator.is_some(),
        separator_name,
    })
}

// handle cases like:  `op: ('-' | '+' | '*')`
fn handle_tokens_in_unions(
    fields: &mut Vec<Field>,
    grammar: &Grammar,
    rule: &Rule,
    label: &str,
    optional: bool,
) -> bool {
    let (rule, optional) = match rule {
        Rule::Opt(rule) => (&**rule, true),
        _ => (rule, optional),
    };

    let rule = match rule {
        Rule::Alt(rule) => rule,
        _ => return false,
    };

    let mut token_kinds = vec![];
    for rule in rule.iter() {
        match rule {
            Rule::Token(token) => token_kinds.push(clean_token_name(grammar, token)),
            _ => return false,
        }
    }

    let field = Field::Token {
        name: label.to_string(),
        kind: TokenKind::Many(token_kinds),
        optional,
    };
    fields.push(field);
    true
}
