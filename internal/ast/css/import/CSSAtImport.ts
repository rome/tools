import {
	CSSAtImportValue,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSAtImport extends NodeBaseWithComments {
	readonly type: "CSSAtImport";
	readonly value: CSSAtImportValue;
}

export const cssAtImport = createBuilder<CSSAtImport>(
	"CSSAtImport",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
