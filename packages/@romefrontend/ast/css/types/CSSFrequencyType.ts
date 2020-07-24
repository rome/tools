import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSFrequencyType extends NodeBaseWithComments {
	type: "CSSFrequencyType";
	// TODO
}

export const cssFrequencyType = createBuilder<CSSFrequencyType>(
	"CSSFrequencyType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
