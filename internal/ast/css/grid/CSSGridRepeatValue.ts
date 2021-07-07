import {CSSGridRepeatValues, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSGridRepeatValue extends NodeBaseWithComments {
	readonly type: "CSSGridRepeatValue";
	readonly values: CSSGridRepeatValues;
}

export const cssGridRepeatValue = createBuilder<CSSGridRepeatValue>(
	"CSSGridRepeatValue",
	{
		bindingKeys: {},
		visitorKeys: {
			values: true,
		},
	},
);
