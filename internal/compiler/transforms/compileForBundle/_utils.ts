/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SCOPE_PRIVATE_PREFIX} from "@internal/compiler";
import {BundleCompileOptions} from "../../types";
import CompilerContext from "../../lib/CompilerContext";
import { createUIDPath, UIDPath } from "@internal/path";

export function getOptions(context: CompilerContext): BundleCompileOptions {
	const opts = context.options.bundle;
	if (opts === undefined) {
		throw new Error("No bundle options found");
	}
	return opts;
}

export function getPrivateName(name: string, moduleId: UIDPath) {
	return `${SCOPE_PRIVATE_PREFIX}$priv$${normalizeModuleId(moduleId)}$${name}`;
}

// This is necessary so we can take our module uids which are paths on the file system into a valid JS jsIdentifier name
export function normalizeModuleId(id: UIDPath): UIDPath {
	// TODO probably need more stuff in this
	return createUIDPath(id.join().replace(/[\\\/@\-]/g, "$").replace(/[\-.]/g, "_"));
}

export function getPrefixedName(
	name: string,
	moduleId: UIDPath,
	opts: BundleCompileOptions,
) {
	const forwarded = opts.resolvedImports[`${moduleId.join()}:${name}`];
	if (forwarded !== undefined) {
		moduleId = forwarded.id;
		name = forwarded.name;
	}

	return `${getPrefixedNamespace(normalizeModuleId(moduleId))}$${name}`;
}

export function getPrefixedNamespace(moduleId: UIDPath) {
	return `${SCOPE_PRIVATE_PREFIX}${normalizeModuleId(moduleId)}`;
}

export function getModuleId(
	source: string,
	opts: BundleCompileOptions,
): undefined | UIDPath {
	return opts.relativeSourcesToModuleId[source];
}
