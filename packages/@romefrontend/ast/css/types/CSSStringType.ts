import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSStringType extends NodeBaseWithComments {
	type: "CSSStringType";
	value: string;
}

export const cssStringType = createBuilder<CSSStringType>(
	"CSSStringType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
