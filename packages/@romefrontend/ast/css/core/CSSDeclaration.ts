import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export type CSSDeclaration = NodeBaseWithComments & {
	type: "CSSDeclaration";
	name: string;
	important: boolean;
	value: Array<AnyCSSValue | null>;
};

export const cssDeclaration = createBuilder<CSSDeclaration>(
	"CSSDeclaration",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
