/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SCOPE_PRIVATE_PREFIX} from "@internal/compiler";
import {BundleCompileOptions} from "../../types";
import CompilerContext from "../../lib/CompilerContext";
import {UIDPath, createUIDPath} from "@internal/path";

export function getOptions(context: CompilerContext): BundleCompileOptions {
	const opts = context.options.bundle;
	if (opts === undefined) {
		throw new Error("No bundle options found");
	}
	return opts;
}

export function getPrivateName(name: string, moduleId: UIDPath): string {
	return `${SCOPE_PRIVATE_PREFIX}$priv$${normalizeModuleId(moduleId).format()}$${name}`;
}

// This is necessary so we can take our module uids which are paths on the file system into a valid JS jsIdentifier name
export function normalizeModuleId(id: UIDPath): UIDPath {
	// TODO probably need more stuff in this
	return createUIDPath(
		id.format().replace(/[\\\/@\-]/g, "$").replace(/[\-.]/g, "_"),
	);
}

export function getPrefixedName(
	name: string,
	moduleId: UIDPath,
	opts: BundleCompileOptions,
) {
	const forwardMap = opts.resolvedImports.get(moduleId);
	if (forwardMap !== undefined) {
		const forwarded = forwardMap.get(name);
		if (forwarded !== undefined) {
			moduleId = forwarded.id;
			name = forwarded.name;
		}
	}

	return `${getPrefixedNamespace(normalizeModuleId(moduleId))}$${name}`;
}

export function getPrefixedNamespace(moduleId: UIDPath): string {
	return `${SCOPE_PRIVATE_PREFIX}${normalizeModuleId(moduleId).format()}`;
}

export function getModuleId(
	source: string,
	opts: BundleCompileOptions,
): undefined | UIDPath {
	return opts.relativeSourcesToModuleId.get(source);
}
