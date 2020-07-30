/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {
	CheckProvider,
	ModuleSignature,
	ModuleSignatureExport,
	ModuleSignatureType,
} from "../types";
import {JSRoot} from "@internal/ast";
import buildGraph from "./buildGraph";
import T from "../types/T";
import E from "../types/errors/E";
import {TransformProjectDefinition} from "@internal/compiler";
import {Dict} from "@internal/typescript-helpers";

const exportsCache: WeakMap<JSRoot, ModuleSignature> = new WeakMap();

export default async function getModuleSignature(
	opts: {
		ast: JSRoot;
		project: TransformProjectDefinition;
		provider: CheckProvider;
	},
): Promise<ModuleSignature> {
	const {ast, provider} = opts;
	const {filename} = ast;

	if (filename.includes("node_modules")) {
		return {
			filename,
			exports: [],
			types: {},
		};
	}

	const cached = exportsCache.get(ast);
	if (cached !== undefined) {
		return cached;
	}

	const {
		evaluator: {exports},
		utils,
	} = await buildGraph({
		ast,
		project: opts.project,
		connected: false,
		provider,
	});
	const types: Dict<ModuleSignatureType> = {};
	const exportMap: Array<ModuleSignatureExport> = [];

	const added: Set<T> = new Set();

	function addType(type: T): string {
		const reducedType = utils.reduce(type);
		if (added.has(reducedType)) {
			return reducedType.id;
		} else {
			added.add(reducedType);
		}

		// export errors as any types to suppress errors
		if (reducedType instanceof E) {
			types[reducedType.id] = {
				human: undefined,
				origin: reducedType.originLoc,
				type: "AnyT",
				data: {},
			};
			return reducedType.id;
		}

		const data = reducedType.serialize(addType);

		types[reducedType.id] = {
			human: reducedType.human,
			origin: reducedType.originLoc,
			type: reducedType.getConstructor().type,
			data,
		};
		return reducedType.id;
	}

	for (const def of exports) {
		if (def.type === "all") {
			exportMap.push({
				type: "all",
				source: def.source,
			});
		} else if (def.type === "local") {
			exportMap.push({
				type: "local",
				name: def.name,
				value: addType(def.value),
			});
		} else {
			throw new Error("unknown export def type");
		}
	}

	const result: ModuleSignature = {
		filename,
		exports: exportMap,
		types,
	};
	exportsCache.set(ast, result);
	return result;
}
