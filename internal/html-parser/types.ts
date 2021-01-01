import {BaseTokens, SimpleToken, StringToken} from "@internal/parser-core";

export type Tokens = BaseTokens & {
	Doctype: StringToken<"Doctype">;
	Cdata: StringToken<"Cdata">;
	Text: StringToken<"Text">;
	// <
	TagStartOpen: SimpleToken<"TagStartOpen">;
	// />
	TagSelfClosing: SimpleToken<"TagSelfClosing">;
	// >
	TagEnd: SimpleToken<"TagEnd">;
	// </
	TagEndOpen: SimpleToken<"TagEndOpen">;
	Equals: SimpleToken<"Equals">;
	Identifier: StringToken<"Identifier">;
	String: StringToken<"String">;
	Comment: StringToken<"Comment">;
};
export type State = {
	inTagHead: boolean;
};
