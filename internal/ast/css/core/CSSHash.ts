import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSHash extends NodeBaseWithComments {
	readonly type: "CSSHash";
	readonly value: string;
}

export const CssHash = createBuilder<CSSHash>(
	"CSSHash",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
