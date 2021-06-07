import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSImport extends NodeBaseWithComments {
	readonly type: "CSSImport";
}

export const cssImport = createBuilder<CSSImport>(
	"CSSImport",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
