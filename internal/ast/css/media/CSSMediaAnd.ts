import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaAnd extends NodeBaseWithComments {
	readonly type: "CSSMediaAnd";
}

export const cssMediaAnd = createBuilder<CSSMediaAnd>(
	"CSSMediaAnd",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
