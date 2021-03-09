import {
	PrefixVisitor,
	wrapPrefixVisitor,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {ProjectConfig} from "@internal/project";
import transition from "@internal/compiler/transforms/compile/css-handler/prefix/prefixes/transition";
import transform from "@internal/compiler/transforms/compile/css-handler/prefix/prefixes/transform";
import display from "@internal/compiler/transforms/compile/css-handler/prefix/prefixes/display";
import {UnknownObject} from "@internal/typescript-helpers";
import {resolveBrowsers} from "@internal/codec-browsers";

const prefixVisitors: PrefixVisitor<UnknownObject>[] = [
	transition,
	transform,
	...display,
];

export default (projectConfig: ProjectConfig) =>
	prefixVisitors.map((prefixVisitor) =>
		wrapPrefixVisitor(
			prefixVisitor,
			resolveBrowsers(">0%") // TODO: load from ProjectConfig
		)
	)
;
