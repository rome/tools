import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSIntegerType = NodeBaseWithComments & {
	type: "CSSIntegerType";
	// TODO
};

export const cssIntegerType = createBuilder<CSSIntegerType>(
	"CSSIntegerType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
