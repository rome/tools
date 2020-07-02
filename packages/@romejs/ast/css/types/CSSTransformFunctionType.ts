import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTransformFunctionType = NodeBaseWithComments & {
	type: "CSSTransformFunctionType";
	// TODO
};

export const cssTransformFunctionType = createBuilder<CSSTransformFunctionType>(
	"CSSTransformFunctionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
