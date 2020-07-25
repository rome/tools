import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSDeclaration extends NodeBaseWithComments {
	type: "CSSDeclaration";
	name: string;
	important: boolean;
	value: Array<AnyCSSValue | undefined>;
}
export const cssDeclaration = createBuilder<CSSDeclaration>(
	"CSSDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
