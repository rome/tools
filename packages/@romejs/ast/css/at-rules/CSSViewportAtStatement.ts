import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSViewportAtStatement = JSNodeBase & {
	type: "CSSViewportAtStatement";
};

export const cssViewportAtStatement = createBuilder<CSSViewportAtStatement>(
	"CSSViewportAtStatement",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
