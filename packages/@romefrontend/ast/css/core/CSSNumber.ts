import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSNumber = NodeBaseWithComments & {
	type: "CSSNumber";
	value: number;
};

export const cssNumber = createBuilder<CSSNumber>(
	"CSSNumber",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
