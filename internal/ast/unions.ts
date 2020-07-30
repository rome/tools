import * as n from "@romefrontend/ast";

export type AnyRoot =
	| n.JSRoot
	| n.CSSRoot
	| n.HTMLRoot
	| n.MarkdownRoot
	| n.CommitRoot;

export type AnyComment = n.CommentBlock | n.CommentLine;
