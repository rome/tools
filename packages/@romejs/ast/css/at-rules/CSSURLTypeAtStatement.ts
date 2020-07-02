import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSURLTypeAtStatement = NodeBaseWithComments & {
	type: "CSSURLTypeAtStatement";
};

export const cssurlTypeAtStatement = createBuilder<CSSURLTypeAtStatement>(
	"CSSURLTypeAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
