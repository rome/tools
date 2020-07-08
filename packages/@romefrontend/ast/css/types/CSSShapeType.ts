import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSShapeType = NodeBaseWithComments & {
	type: "CSSShapeType";
	// TODO
};

export const cssShapeType = createBuilder<CSSShapeType>(
	"CSSShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
