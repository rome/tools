import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export type CSSMediaValidType = "all" | "print" | "screen";

export interface CSSMediaType extends NodeBaseWithComments {
	readonly type: "CSSMediaType";
	readonly value: CSSMediaValidType;
}

export const cssMediaType = createBuilder<CSSMediaType>(
	"CSSMediaType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
