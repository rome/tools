import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSPercentage = NodeBaseWithComments & {
	type: "CSSPercentage";
	value: number;
};

export const cssPercentage = createBuilder<CSSPercentage>(
	"CSSPercentage",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
