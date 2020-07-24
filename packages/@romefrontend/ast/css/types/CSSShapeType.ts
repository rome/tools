import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSShapeType extends NodeBaseWithComments {
	type: "CSSShapeType";
	// TODO
}

export const cssShapeType = createBuilder<CSSShapeType>(
	"CSSShapeType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
