import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface HTMLText extends NodeBaseWithComments {
	type: "HTMLText";
	value: string;
}

export const htmlText = createBuilder<HTMLText>(
	"HTMLText",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
