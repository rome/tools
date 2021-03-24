import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSMediaFeatureLT extends NodeBaseWithComments {
	readonly type: "CSSMediaFeatureLT";
	readonly hasEqual?: boolean;
}

export const cssMediaFeatureLT = createBuilder<CSSMediaFeatureLT>(
	"CSSMediaFeatureLT",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
