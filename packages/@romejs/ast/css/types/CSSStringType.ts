import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSStringType = NodeBaseWithComments & {
	type: "CSSStringType";
};

export const cssStringType = createBuilder<CSSStringType>(
	"CSSStringType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
