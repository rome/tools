import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type HTMLIdentifier = NodeBaseWithComments & {
	type: "HTMLIdentifier";
	name: string;
};

export const htmlIdentifier = createBuilder<HTMLIdentifier>(
	"HTMLIdentifier",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
