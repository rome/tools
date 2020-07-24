import {
	CSSAtRule,
	CSSRule,
	NodeBaseWithComments,
	RootBase,
} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSRoot = NodeBaseWithComments &
	RootBase & {
		type: "CSSRoot";
		body: Array<CSSAtRule | CSSRule>;
	};

export const cssRoot = createBuilder<CSSRoot>(
	"CSSRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
