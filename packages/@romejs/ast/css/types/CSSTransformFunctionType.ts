import {JSNodeBase} from "../../index";
import {createBuilder} from "../../utils";

export type CSSTransformFunctionType = JSNodeBase & {
	type: "CSSTransformFunctionType";
};

export const cssTransformFunctionType = createBuilder<CSSTransformFunctionType>(
	"CSSTransformFunctionType",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
