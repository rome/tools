import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type HTMLString = NodeBaseWithComments & {
	type: "HTMLString";
	value: string;
};

export const htmlString = createBuilder<HTMLString>(
	"HTMLString",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
