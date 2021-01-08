import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSString extends NodeBaseWithComments {
	readonly type: "CSSString";
	readonly value: string;
}

export const cssString = createBuilder<CSSString>(
	"CSSString",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
