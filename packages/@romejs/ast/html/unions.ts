import * as n from "@romejs/ast";

export type HTMLAnyNode =
	| n.HTMLCommentBlock
	| n.HTMLDoctypeTag
	| n.HTMLTag
	| n.HTMLXmlTag
	| n.HTMLText;

export type AnyHTMLAttribute =
	| n.HTMLAriaAttribute
	| n.HTMLAttribute
	| n.HTMLDataAttribute;
