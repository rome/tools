import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSRoot = NodeBaseWithComments & {
	type: "CSSRoot";
};

export const cssRoot = createBuilder<CSSRoot>(
	"CSSRoot",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
