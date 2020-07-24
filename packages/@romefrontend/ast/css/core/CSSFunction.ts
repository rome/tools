import {NodeBaseWithComments} from "@romefrontend/ast";
import {createBuilder} from "@romefrontend/ast/utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSFunction extends NodeBaseWithComments {
	type: "CSSFunction";
	name: string;
	value: Array<AnyCSSValue | undefined>;
}
export const cssFunction = createBuilder<CSSFunction>(
	"CSSFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			value: true,
		},
	},
);
