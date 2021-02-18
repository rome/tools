import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaOr extends NodeBaseWithComments {
	readonly type: "CSSMediaOr";
}

export const cssMediaOr = createBuilder<CSSMediaOr>(
	"CSSMediaOr",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
