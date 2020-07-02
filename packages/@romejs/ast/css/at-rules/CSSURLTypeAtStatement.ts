import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSURLTypeAtStatement = JSNodeBase & {
	type: "CSSURLTypeAtStatement";
};

export const cssurlTypeAtStatement = createBuilder<CSSURLTypeAtStatement>(
	"CSSURLTypeAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
