import {AnyCSSToken, CSSParser, Tokens} from "../types";

type TokenType = AnyCSSToken["type"];

function skipComments(parser: CSSParser) {
	while (!parser.matchToken("EOF") && parser.matchToken("Comment")) {
		parser.registerComment(
			parser.comments.createComment({
				type: "CommentBlock",
				loc: parser.finishLoc(parser.getPosition()),
				value: (parser.getToken() as Tokens["Comment"]).value,
			}),
		);
		parser.eatToken("Comment");
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
