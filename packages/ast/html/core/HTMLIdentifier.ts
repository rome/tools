import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "../../utils";

export interface HTMLIdentifier extends NodeBaseWithComments {
	type: "HTMLIdentifier";
	name: string;
}

export const htmlIdentifier = createBuilder<HTMLIdentifier>(
	"HTMLIdentifier",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
