import {CSSBlock, CSSKeyframe, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSAtRule extends NodeBaseWithComments {
	readonly type: "CSSAtRule";
	readonly name: string;
	readonly prelude: AnyCSSValue[];
	readonly block?: CSSBlock | CSSKeyframe;
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
