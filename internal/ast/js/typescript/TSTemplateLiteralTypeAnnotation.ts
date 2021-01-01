import {
	AnyTSPrimary,
	NodeBaseWithComments,
	TSTemplateElement,
} from "@internal/ast";
import {createBuilder} from "../../utils";

export interface TSTemplateLiteralTypeAnnotation extends NodeBaseWithComments {
	readonly type: "TSTemplateLiteralTypeAnnotation";
	readonly quasis: TSTemplateElement[];
	readonly expressions: AnyTSPrimary[];
}

export const tsTemplateLiteralTypeAnnotation = createBuilder<TSTemplateLiteralTypeAnnotation>(
	"TSTemplateLiteralTypeAnnotation",
	{
		bindingKeys: {},
		visitorKeys: {
			quasis: true,
			expressions: true,
		},
	},
);
