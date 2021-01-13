import {PrefixVisitor, wrapPrefixVisitor} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {ProjectConfig} from "@internal/project";
import transition from "@internal/compiler/transforms/compile/css-handler/prefix/prefixes/transition";

const prefixVisitors: PrefixVisitor<any>[] = [
	transition
];

export default (projectConfig: ProjectConfig) => (
	prefixVisitors.map((prefixVisitor) => wrapPrefixVisitor(prefixVisitor, {
		target: "modern" // TODO implement target browser
		// mode: projectConfig.targetBrowser or something
	}))
)
