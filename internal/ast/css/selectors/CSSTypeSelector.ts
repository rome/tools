import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {CSSQualifiedName} from "../core/CSSQualifiedName";

export interface CSSTypeSelector extends NodeBaseWithComments {
	readonly type: "CSSTypeSelector";
	readonly value: CSSQualifiedName;
}

export const cssTypeSelector = createBuilder<CSSTypeSelector>(
	"CSSTypeSelector",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
