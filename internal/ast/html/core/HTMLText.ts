import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface HTMLText extends NodeBaseWithComments {
	readonly type: "HTMLText";
	readonly value: string;
}

export const htmlText = createBuilder<HTMLText>(
	"HTMLText",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
