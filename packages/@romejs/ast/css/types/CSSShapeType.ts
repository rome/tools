import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSShapeType = JSNodeBase & {
	type: "CSSShapeType";
};

export const cssShapeType = createBuilder<CSSShapeType>(
	"CSSShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
