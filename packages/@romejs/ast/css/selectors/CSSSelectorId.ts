import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSSelectorId = JSNodeBase & {
	type: "CSSSelectorId";
};

export const cssSelectorId = createBuilder<CSSSelectorId>(
	"CSSSelectorId",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
