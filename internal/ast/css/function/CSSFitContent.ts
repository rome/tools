import {CSSFitContentValue, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSFitContent extends NodeBaseWithComments {
	readonly type: "CSSFitContent";
	readonly name: string;
	readonly params: [CSSFitContentValue];
}

export const CSSFitContent = createBuilder<CSSFitContent>(
	"CSSFitContent",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
