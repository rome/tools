import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSNumber extends NodeBaseWithComments {
	readonly type: "CSSNumber";
	readonly value: number;
}
export const cssNumber = createBuilder<CSSNumber>(
	"CSSNumber",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
