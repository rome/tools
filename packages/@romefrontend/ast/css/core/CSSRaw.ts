import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSRaw = NodeBaseWithComments & {
	type: "CSSRaw";
	value: string;
};

export const cssRaw = createBuilder<CSSRaw>(
	"CSSRaw",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
