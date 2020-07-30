import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSIdentifier extends NodeBaseWithComments {
	readonly type: "CSSIdentifier";
	readonly value: string;
}
export const cssIdentifier = createBuilder<CSSIdentifier>(
	"CSSIdentifier",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
