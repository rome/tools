import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSImageType = NodeBaseWithComments & {
	type: "CSSImageType";
	// TODO
};

export const cssImageType = createBuilder<CSSImageType>(
	"CSSImageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
