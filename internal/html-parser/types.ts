import {BaseTokens, SimpleToken, ValueToken} from "@internal/parser-core";

export type Tokens = BaseTokens & {
	Doctype: ValueToken<"Doctype", string>;
	Cdata: ValueToken<"Cdata", string>;
	Text: ValueToken<"Text", string>;
	// <
	TagStartOpen: SimpleToken<"TagStartOpen">;
	// />
	TagSelfClosing: SimpleToken<"TagSelfClosing">;
	// >
	TagEnd: SimpleToken<"TagEnd">;
	// </
	TagEndOpen: SimpleToken<"TagEndOpen">;
	Equals: SimpleToken<"Equals">;
	Identifier: ValueToken<"Identifier", string>;
	String: ValueToken<"String", string>;
	Comment: ValueToken<"Comment", string>;
};
export type State = {
	inTagHead: boolean;
};
