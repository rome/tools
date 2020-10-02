import {NodeBaseWithComments, RootBase} from "@internal/ast";
import {createBuilder} from "../../utils";
import {AnyTomlNode} from "@internal/ast/toml/unions";

export interface TomlRoot extends NodeBaseWithComments,
RootBase {
	readonly type: "TomlRoot";
	readonly body: Array<AnyTomlNode>;
}

export const tomlRoot = createBuilder<TomlRoot>(
	"TomlRoot",
	{
		bindingKeys: {},
		visitorKeys: {
			body: true,
			comments: true,
		},
	},
);
