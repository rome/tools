import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTimeType = NodeBaseWithComments & {
	type: "CSSTimeType";
	// TODO
};

export const cssTimeType = createBuilder<CSSTimeType>(
	"CSSTimeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
