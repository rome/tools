import {
	PrefixVisitor,
	wrapPrefixVisitor,
} from "@internal/compiler/transforms/compile/css-handler/prefix/utils";
import {ProjectConfig} from "@internal/project";
import {UnknownObject} from "@internal/typescript-helpers";
import {resolveBrowsers} from "@internal/codec-browsers";

/* GENERATED:START(hash:cd506990d1f961e2a79cd38e90f7adcdabbb4055,id:main) Everything below is automatically generated. DO NOT MODIFY. Run `./rome run scripts/generated-files/lint-rules` to update. */
import display from "./prefixes/display";
import transform from "./prefixes/transform";
import transition from "./prefixes/transition";

const prefixVisitors: PrefixVisitor<UnknownObject>[] = [
	...display,
	...transform,
	...transition,
];
/* GENERATED:END(id:main) */

export default (projectConfig: ProjectConfig) =>
	prefixVisitors.map((prefixVisitor) =>
		wrapPrefixVisitor(
			prefixVisitor,
			resolveBrowsers(">0%"), // TODO: load from ProjectConfig
		)
	)
;
