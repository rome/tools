import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSImageType extends NodeBaseWithComments {
	type: "CSSImageType";
	// TODO
}

export const cssImageType = createBuilder<CSSImageType>(
	"CSSImageType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
