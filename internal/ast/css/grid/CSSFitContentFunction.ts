import {CSSFitContentValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSFitContentFunction extends NodeBaseWithComments {
	readonly type: "CSSFitContentFunction";
	readonly name: string;
	readonly params: [CSSFitContentValue];
}

export const cssFitContentFunction = createBuilder<CSSFitContentFunction>(
	"CSSFitContentFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
