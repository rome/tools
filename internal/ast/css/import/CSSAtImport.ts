import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSAtImport extends NodeBaseWithComments {
	readonly type: "CSSAtImport";
}

export const CSSAtImport = createBuilder<CSSAtImport>(
	"CSSAtImport",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
