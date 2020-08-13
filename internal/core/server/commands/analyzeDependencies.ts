/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ServerRequest} from "@internal/core";
import {commandCategories} from "../../common/commands";
import {createServerCommand} from "../commands";
import {Consumer} from "@internal/consume";
import {SourceLocation} from "@internal/parser-core";
import {markup} from "@internal/markup";

type Flags = {
	focusSource: undefined | string;
	compact: boolean;
};

function removeLoc<T extends {
	loc?: SourceLocation;
}>(obj: T): Omit<T, "loc"> {
	const {loc, ...locless} = obj;
	loc;
	return locless;
}

export default createServerCommand({
	category: commandCategories.SOURCE_CODE,
	description: markup`analyze and dump the dependencies of a file`,
	usage: "",
	examples: [],
	hidden: true,
	defineFlags(c: Consumer): Flags {
		return {
			compact: c.get("compact").asBoolean(false),
			focusSource: c.get("focusSource").asStringOrVoid(),
		};
	},
	async callback(req: ServerRequest, commandFlags: Flags): Promise<void> {
		const {reporter} = req;
		const filename = await req.resolveEntryAssertPathArg(0);

		let res = await req.requestWorkerAnalyzeDependencies(filename, {});

		const {focusSource} = commandFlags;
		if (focusSource !== undefined) {
			res = {
				...res,
				importFirstUsage: res.importFirstUsage.filter((dep) => {
					return dep.source === focusSource;
				}),
				dependencies: res.dependencies.filter((dep) => {
					return dep.source === focusSource;
				}),
			};
		}

		if (commandFlags.compact) {
			res = {
				...res,
				topLevelLocalBindings: {},
				importFirstUsage: res.importFirstUsage.map((imp) => {
					return removeLoc(imp);
				}),
				exports: res.exports.map((exp) => {
					// This weird switch is because TS only returns an object with the properties common amongst all
					switch (exp.type) {
						case "local":
							return removeLoc(exp);

						case "external":
							return removeLoc(exp);

						case "externalAll":
							return removeLoc(exp);

						case "externalNamespace":
							return removeLoc(exp);
					}
				}),
				dependencies: res.dependencies.map((dep) => {
					return {
						...removeLoc(dep),
						names: dep.names.map((name) => {
							return removeLoc(name);
						}),
					};
				}),
			};
		}

		reporter.inspect(res);
	},
});
