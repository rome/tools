import {
	CSSAtRule,
	CSSRule,
	NodeBaseWithComments,
	RootBase,
} from "@internal/ast";
import {createBuilder} from "../../utils";

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
