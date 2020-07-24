import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export interface CSSTransformFunctionType extends NodeBaseWithComments {
	type: "CSSTransformFunctionType";
	// TODO
}

export const cssTransformFunctionType = createBuilder<CSSTransformFunctionType>(
	"CSSTransformFunctionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
