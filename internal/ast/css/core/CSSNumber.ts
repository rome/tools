import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSNumber extends NodeBaseWithComments {
	readonly type: "CSSNumber";
	readonly value: number;
	readonly raw: string;
}
export const cssNumber = createBuilder<CSSNumber>(
	"CSSNumber",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
