import {NodeBaseWithComments} from "../../index";
import {createBuilder} from "../../utils";

export type HTMLXmlTag = NodeBaseWithComments & {
	type: "HTMLXmlTag";
};

export const htmlXmlTag = createBuilder<HTMLXmlTag>(
	"HTMLXmlTag",
	{
		bindingKeys: {},
		visitorKeys: {},
	},
);
