import {CSSRaw, CSSString, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSGridTemplateAreaValue extends NodeBaseWithComments {
	readonly type: "CSSGridTemplateAreaValue";
	readonly value: Array<CSSRaw | CSSString>;
}

export const cssGridTemplateAreaValue = createBuilder<CSSGridTemplateAreaValue>(
	"CSSGridTemplateAreaValue",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
