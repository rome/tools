import * as n from "@romefrontend/ast";

export type AnyRoot = n.JSRoot | n.CSSStylesheet | n.HTMLRoot | n.MarkdownRoot;

export type AnyComment = n.CommentBlock | n.CommentLine;
