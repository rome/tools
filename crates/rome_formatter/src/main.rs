use rome_formatter::{
    format_element::ConditionalGroupContent, hard_line_break, indent, soft_block_indent,
    soft_line_break_or_space, space_token, LineWidth,
};

fn main() {
    use rome_formatter::{
        format_elements, group_elements, soft_line_break, token, FormatOptions, Formatted,
    };

    let elements = group_elements(format_elements![
        token("{"),
        soft_block_indent(format_elements![
            soft_line_break_or_space(),
            token("somethingSuperLongsomethingSuperLong2"),
            // space_token(),
            ConditionalGroupContent::new(
                token(","),
                rome_formatter::format_element::GroupPrintMode::Multiline
            )
        ]),
        token("}"),
        space_token(),
        token("from"),
        space_token(),
        token("\"somethingSuperLongsomethingSuperLongsomethingSuperLong\""),
        token(";"),
    ]);

    //     List [
    //     SyntaxTokenSlice("import"),
    //     Space,
    //     SyntaxTokenSlice("ww"),
    //     SyntaxTokenSlice(","),
    //     Space,
    //     Group(
    //         List [
    //             SyntaxTokenSlice("{"),
    //             Indent(
    //                 List [
    //                     Line(SoftOrSpace),
    //                     SyntaxTokenSlice("somethingSuperLongsomethingSuperLong2"),
    //                     ConditionalGroupContent {
    //                         content: StaticToken(","),
    //                         mode: Multiline,
    //                     },
    //                 ],
    //             ),
    //             Line(SoftOrSpace),
    //             SyntaxTokenSlice("}"),
    //         ],
    //     ),
    //     Space,
    //     SyntaxTokenSlice("from"),
    //     Space,
    //     DynamicToken("\"somethingSuperLongsomethingSuperLongsomethingSuperLong\""),
    //     StaticToken(";"),
    //     SyntaxTokenSlice(""),
    //     Line(Hard),
    // ]
    let elements = format_elements![
        token("import"),
        space_token(),
        token("ww"),
        token(","),
        space_token(),
        elements,
        // hard_line_break()
    ];
    println!(
        "{}",
        Formatted::new(elements, FormatOptions::default().with_line_width(113.try_into().unwrap()))
            .print()
            .as_code()
    );
    // assert_eq!(
    //     "a,b",
    //     Formatted::new(elements, FormatOptions::default())
    //         .print()
    //         .as_code()
    // );
}
