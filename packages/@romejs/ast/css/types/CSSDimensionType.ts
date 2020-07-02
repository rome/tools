import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSDimensionType = JSNodeBase & {
	type: "CSSDimensionType";
};

export const cssDimensionType = createBuilder<CSSDimensionType>(
	"CSSDimensionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
