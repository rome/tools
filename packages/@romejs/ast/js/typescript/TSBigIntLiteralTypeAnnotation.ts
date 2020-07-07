import {NodeBaseWithComments} from "@romejs/ast";
import {createBuilder} from "../../utils";

export type TSBigIntLiteralTypeAnnotation = NodeBaseWithComments & {
	type: "TSBigIntLiteralTypeAnnotation";
	value: string;
};

export const tsBigIntLiteralTypeAnnotation = createBuilder<TSBigIntLiteralTypeAnnotation>(
	"TSBigIntLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
