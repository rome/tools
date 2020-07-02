import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTransformFunctionType = NodeBaseWithComments & {
	type: "CSSTransformFunctionType";
};

export const cssTransformFunctionType = createBuilder<CSSTransformFunctionType>(
	"CSSTransformFunctionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
