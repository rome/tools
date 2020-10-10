import * as n from "@internal/ast";

export type AnyRoot =
	| n.JSRoot
	| n.CSSRoot
	| n.HTMLRoot
	| n.MarkdownRoot
	| n.CommitRoot
	| n.TomlRoot;

export type AnyComment = n.CommentBlock | n.CommentLine;
