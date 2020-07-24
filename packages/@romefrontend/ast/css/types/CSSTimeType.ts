import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSTimeType extends NodeBaseWithComments {
	type: "CSSTimeType";
	// TODO
}

export const cssTimeType = createBuilder<CSSTimeType>(
	"CSSTimeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
