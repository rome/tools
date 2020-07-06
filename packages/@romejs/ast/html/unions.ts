import * as n from "@romejs/ast";

export type HTMLAnyNode =
	| n.HTMLDoctypeTag
	| n.HTMLTag
	| n.HTMLXmlTag
	| n.HTMLText;
