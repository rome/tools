use rome_formatter::{
    format_elements, group_elements, hard_group_elements, hard_line_break, soft_block_indent,
    space_token, token, FormatOptions, Formatted, soft_line_break_or_space,
};

// List [
//     SyntaxTokenSlice("let"),
//     Space,
//     Group(
//         List [
//             Group(
//                 List [
//                     SyntaxTokenSlice("{"),
//                     Indent(
//                         List [
//                             Line(SoftOrSpace),
//                             SyntaxTokenSlice("\"this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line\""),
//                             SyntaxTokenSlice(":"),
//                             Space,
//                             SyntaxTokenSlice("orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig"),
//                             HardGroup(
//                                 List [
//                                     SyntaxTokenSlice("("),
//                                     SyntaxTokenSlice(")"),
//                                 ],
//                             ),
//                             ConditionalGroupContent {
//                                 content: SyntaxTokenSlice(","),
//                                 mode: Multiline,
//                             },
//                         ],
//                     ),
//                     Line(SoftOrSpace),
//                     SyntaxTokenSlice("}"),
//                 ],
//             ),
//         ],
//     ),
//     SyntaxTokenSlice(";"),
//     SyntaxTokenSlice(""),
//     Line(Hard),
// ]
fn main() {
    let element = format_elements![
        token("let"),
        space_token(),
        group_elements(format_elements![
            hard_group_elements(token("a")),
            space_token(),
            hard_group_elements(token("=")),
            space_token(),
            group_elements(format_elements![
                token("{"),
                soft_block_indent(format_elements![token("\"this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line\""), token(":"), soft_line_break_or_space(), token("orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig"), hard_group_elements(format_elements![token("("), token(")")])]),
                token("}")
            ])
        ]),
        token(";"),
        token(""),
        hard_line_break()
    ];
    println!(
        "{}",
        Formatted::new(element, FormatOptions::default())
            .print()
            .as_code()
    );
}
