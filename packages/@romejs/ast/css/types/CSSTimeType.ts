import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTimeType = NodeBaseWithComments & {
	type: "CSSTimeType";
};

export const cssTimeType = createBuilder<CSSTimeType>(
	"CSSTimeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
