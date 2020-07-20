import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSIdent = NodeBaseWithComments & {
	type: "CSSIdent";
	value: string;
};

export const cssIdent = createBuilder<CSSIdent>(
	"CSSIdent",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
