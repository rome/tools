import {CSSString, NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSUrlFunction extends NodeBaseWithComments {
	readonly type: "CSSUrlFunction";
	readonly name: string;
	readonly params: [CSSString];
}

export const cssUrlFunction = createBuilder<CSSUrlFunction>(
	"CSSUrlFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
