import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

// aria-checked
export type HTMLAriaAttribute = NodeBaseWithComments & {
	type: "HTMLAriaAttribute";
	value: string;
};

export const htmlAriaAttribute = createBuilder<HTMLAriaAttribute>(
	"HTMLAriaAttribute",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
