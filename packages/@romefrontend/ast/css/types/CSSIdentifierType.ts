import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSIdentifierType extends NodeBaseWithComments {
	type: "CSSIdentifierType";
	name: string;
}

export const cssIdentifierType = createBuilder<CSSIdentifierType>(
	"CSSIdentifierType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
