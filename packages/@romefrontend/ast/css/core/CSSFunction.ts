import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export type CSSFunction = NodeBaseWithComments & {
	type: "CSSFunction";
	name: string;
	value: Array<AnyCSSValue | null>;
};

export const cssFunction = createBuilder<CSSFunction>(
	"CSSFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
