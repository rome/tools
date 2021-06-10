import {CSSMinmaxParam, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMinmaxFunction extends NodeBaseWithComments {
	readonly type: "CSSMinmaxFunction";
	readonly params: [CSSMinmaxParam, CSSMinmaxParam];
	readonly name: string;
}

export const cssMinmaxFunction = createBuilder<CSSMinmaxFunction>(
	"CSSMinmaxFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
