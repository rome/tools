import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface CSSQualifiedName extends NodeBaseWithComments {
	readonly type: "CSSQualifiedName";
	readonly namespace?: string;
	readonly localName: string;
}
export const cssQualifiedName = createBuilder<CSSQualifiedName>(
	"CSSQualifiedName",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
