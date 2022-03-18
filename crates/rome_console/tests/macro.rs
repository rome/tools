use rome_console::{MarkupElement, MarkupNode};

#[test]
fn test_macro() {
    let category = "test";

    match
    // Due to how MarkupNode is implemented, the result of the markup macro
    // cannot be stored a binding and must be matched upon immediately
    rome_markup::markup! {
        <Emphasis>"{category} Commands"</Emphasis>
    }
    {
        MarkupNode::Element { kind, children } => {
            assert_eq!(kind, MarkupElement::Emphasis);
            assert_eq!(children.len(), 1);

            match children[0] {
                MarkupNode::Text(args) => {
                    let args = args.to_string();
                    assert_eq!(args, format!("{category} Commands"));
                }
                markup => panic!("unexpected MarkupNode {markup:?}"),
            }
        }
        markup => panic!("unexpected MarkupNode {markup:?}"),
    }
}
