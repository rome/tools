import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSIntegerType extends NodeBaseWithComments {
	type: "CSSIntegerType";
	// TODO
}

export const cssIntegerType = createBuilder<CSSIntegerType>(
	"CSSIntegerType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
