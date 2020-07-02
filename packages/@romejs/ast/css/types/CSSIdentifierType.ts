import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSIdentifierType = JSNodeBase & {
	type: "CSSIdentifierType";
};

export const cssIdentifierType = createBuilder<CSSIdentifierType>(
	"CSSIdentifierType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
