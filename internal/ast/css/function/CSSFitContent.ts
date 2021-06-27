import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSFitContent extends NodeBaseWithComments {
	readonly type: "CSSFitContent";
}

export const CSSFitContent = createBuilder<CSSFitContent>(
	"CSSFitContent",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
