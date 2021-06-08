import {
	AnyCSSValue,
	CSSAtImportValue,
	NodeBaseWithComments,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSAtImport extends NodeBaseWithComments {
	readonly type: "CSSAtImport";
	readonly name: string;
	readonly prelude: AnyCSSValue[];
	readonly block?: CSSAtImportValue;
}

export const cssAtImport = createBuilder<CSSAtImport>(
	"CSSAtImport",
	{
		bindingKeys: {},
		visitorKeys: {
			block: true,
			prelude: true,
		},
	},
);
