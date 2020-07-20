import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";

export type CSSDimension = NodeBaseWithComments & {
	type: "CSSDimension";
	value: number;
	unit: string;
};

export const cssDimension = createBuilder<CSSDimension>(
	"CSSDimension",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
