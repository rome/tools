import {NodeBaseWithComments} from "@romejs/ast";
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
