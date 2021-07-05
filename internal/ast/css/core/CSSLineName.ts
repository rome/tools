import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSLineName extends NodeBaseWithComments {
	readonly type: "CSSLineName";
	readonly value: string;
}

export const cssLineName = createBuilder<CSSLineName>(
	"CSSLineName",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true
		},
	},
);
