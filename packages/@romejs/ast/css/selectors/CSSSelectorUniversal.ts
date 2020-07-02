import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorUniversal = JSNodeBase & {
	type: "CSSSelectorUniversal";
};

export const cssSelectorUniversal = createBuilder<CSSSelectorUniversal>(
	"CSSSelectorUniversal",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
