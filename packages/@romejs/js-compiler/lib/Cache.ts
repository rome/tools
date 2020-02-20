/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {TransformRequest, TransformProjectDefinition} from '../types';
import {Program} from '@romejs/js-ast';
import {JSONObject} from '@romejs/codec-json';

type CacheQuery = {
  key: string;
  ast: Program;
};

let projectIdCounter = 0;
const projectToId: WeakMap<TransformProjectDefinition, number> = new WeakMap();

export default class Cache<Result> {
  constructor() {
    this.cache = new WeakMap();
  }

  cache: WeakMap<Program, Map<string, Result>>;

  static buildQuery(req: TransformRequest, options?: JSONObject): CacheQuery {
    const {ast, project} = req;
    const keyParts: Array<string> = [];

    let projectId = projectToId.get(project);
    if (projectId === undefined) {
      projectId = projectIdCounter++;
      projectToId.set(project, projectId);
    }

    // Add project config cache counter
    keyParts.push(String(projectId));

    // Add options if they exist
    if (options !== undefined && Object.keys(options).length > 0) {
      keyParts.push(JSON.stringify(options));
    }

    return {
      ast,
      key: keyParts.join(';'),
    };
  }

  get(query: CacheQuery): undefined | Result {
    const astCache = this.cache.get(query.ast);
    if (astCache) {
      return astCache.get(query.key);
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
