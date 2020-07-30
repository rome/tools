import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSFunction extends NodeBaseWithComments {
	readonly type: "CSSFunction";
	readonly name: string;
	readonly value: Array<AnyCSSValue | undefined>;
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
