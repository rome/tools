import {AnyCSSToken, CSSParser, Tokens} from "../types";

type TokenType = AnyCSSToken["type"];

function skipComments(parser: CSSParser) {
	while (parser.matchToken("Comment")) {
		const start = parser.getPosition();
		const token = parser.eatToken("Comment") as Tokens["Comment"];
		parser.registerComment(
			parser.comments.createComment({
				type: "CommentBlock",
				loc: parser.finishLoc(start),
				value: token.value,
			}),
		);
	}
}

export function nextToken(parser: CSSParser) {
	skipComments(parser);
	return parser.nextToken();
}

export function matchToken(parser: CSSParser, type: TokenType) {
	skipComments(parser);
	return parser.matchToken(type);
}

export function readToken(parser: CSSParser, type: TokenType) {
	skipComments(parser);
	return parser.eatToken(type);
}
