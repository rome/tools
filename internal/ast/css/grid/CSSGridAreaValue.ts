import {CSSGridLine, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSGridAreaValue extends NodeBaseWithComments {
	readonly type: "CSSGridAreaValue";
	readonly value: CSSGridLine[];
}

export const cssGridAreaValue = createBuilder<CSSGridAreaValue>(
	"CSSGridAreaValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
