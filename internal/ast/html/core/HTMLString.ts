import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface HTMLString extends NodeBaseWithComments {
	readonly type: "HTMLString";
	readonly value: string;
}

export const htmlString = createBuilder<HTMLString>(
	"HTMLString",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
