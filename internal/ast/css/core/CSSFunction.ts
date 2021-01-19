import {NodeBaseWithComments} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyCSSValue} from "../../../css-parser/types";

export interface CSSFunction extends NodeBaseWithComments {
	readonly type: "CSSFunction";
	readonly name: string;
	readonly params: AnyCSSValue[];
}

export const cssFunction = createBuilder<CSSFunction>(
	"CSSFunction",
	{
		bindingKeys: {},
		visitorKeys: {
			params: true,
		},
	},
);
