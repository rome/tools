/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformProjectDefinition, TransformRequest} from "../types";
import {AnyRoot} from "@romejs/ast";
import {JSONObject} from "@romejs/codec-json";

type CacheQuery = {
	key: string;
	ast: AnyRoot;
};

let projectIdCounter = 0;
const projectToId: WeakMap<TransformProjectDefinition, number> = new WeakMap();

export default class Cache<Result> {
	constructor() {
		this.cache = new WeakMap();
	}

	cache: WeakMap<AnyRoot, Map<string, Result>>;

	static buildQuery(
		req: TransformRequest,
		additionalOptions?: JSONObject,
	): CacheQuery {
		const {ast, project, options} = req;
		const keyParts: Array<string> = [];

		let projectId = projectToId.get(project);
		if (projectId === undefined) {
			projectId = projectIdCounter++;
			projectToId.set(project, projectId);
		}

		// Add project config cache counter
		keyParts.push(String(projectId));

		// Add options if they exist
		const extra = {
			...options,
			...additionalOptions,
		};
		if (Object.keys(extra).length > 0) {
			keyParts.push(JSON.stringify(extra));
		}

		return {
			ast,
			key: keyParts.join(";"),
		};
	}

	get(query: CacheQuery): undefined | Result {
		const astCache = this.cache.get(query.ast);
		if (astCache) {
			return astCache.get(query.key);
		} else {
			return undefined;
		}
	}

	set(query: CacheQuery, value: Result) {
		let astCache = this.cache.get(query.ast);
		if (astCache === undefined) {
			astCache = new Map();
			this.cache.set(query.ast, astCache);
		}
		astCache.set(query.key, value);
	}
}
