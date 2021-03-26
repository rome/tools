import {AnyCSSValue, CSSAtRuleValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSAtRule extends NodeBaseWithComments {
	readonly type: "CSSAtRule";
	readonly name: string;
	readonly prelude: AnyCSSValue[];
	readonly block?: CSSAtRuleValue;
}
export const cssAtRule = createBuilder<CSSAtRule>(
	"CSSAtRule",
	{
		bindingKeys: {},
		visitorKeys: {
			block: true,
			prelude: true,
		},
	},
);
