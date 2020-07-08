import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export type MarkDownRoot = NodeBaseWithComments & {
	type: "MarkDownRoot";
};

export const markDownRoot = createBuilder<MarkDownRoot>(
	"MarkDownRoot",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
