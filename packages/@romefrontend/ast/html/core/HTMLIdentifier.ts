import {NodeBaseWithComments} from "@romefrontend/ast";
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
