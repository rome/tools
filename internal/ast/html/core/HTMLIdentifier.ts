import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface HTMLIdentifier extends NodeBaseWithComments {
	readonly type: "HTMLIdentifier";
	readonly name: string;
}

export const htmlIdentifier = createBuilder<HTMLIdentifier>(
	"HTMLIdentifier",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
