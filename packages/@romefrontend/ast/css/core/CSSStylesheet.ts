import {
	CSSAtRule,
	CSSRule,
	NodeBaseWithComments,
	RootBase,
} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSStylesheet = NodeBaseWithComments &
	RootBase & {
		type: "CSSStylesheet";
		body: Array<CSSAtRule | CSSRule>;
	};

export const cssStylesheet = createBuilder<CSSStylesheet>(
	"CSSStylesheet",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
