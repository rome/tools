/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {SCOPE_PRIVATE_PREFIX} from "@internal/compiler";
import {BundleCompileOptions} from "../../types";
import CompilerContext from "../../lib/CompilerContext";

export function getOptions(context: CompilerContext): BundleCompileOptions {
	const opts = context.options.bundle;
	if (opts === undefined) {
		throw new Error("No bundle options found");
	}
	return opts;
}

export function getPrivateName(name: string, moduleId: string) {
	return `${SCOPE_PRIVATE_PREFIX}$priv$${normalizeModuleId(moduleId)}$${name}`;
}

// This is necessary so we can take our module uids which are paths on the file system into a valid JS jsIdentifier name
export function normalizeModuleId(id: string): string {
	// TODO probably need more stuff in this
	return id.replace(/[\\\/@\-]/g, "$").replace(/[\-.]/g, "_");
}

export function getPrefixedName(
	name: string,
	moduleId: string,
	opts: BundleCompileOptions,
) {
	const forwarded = opts.resolvedImports[`${moduleId}:${name}`];
	if (forwarded !== undefined) {
		moduleId = forwarded.id;
		name = forwarded.name;
	}

	return `${getPrefixedNamespace(normalizeModuleId(moduleId))}$${name}`;
}

export function getPrefixedNamespace(moduleId: string) {
	return `${SCOPE_PRIVATE_PREFIX}${normalizeModuleId(moduleId)}`;
}

export function getModuleId(
	source: string,
	opts: BundleCompileOptions,
): undefined | string {
	return opts.relativeSourcesToModuleId[source];
}
