import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSStringType = NodeBaseWithComments & {
	type: "CSSStringType";
	value: string;
};

export const cssStringType = createBuilder<CSSStringType>(
	"CSSStringType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
