/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import E from './E';
import T from '../T';
import {AnyNode} from '@romejs/js-ast';

export default class MissingUnionE extends E {
  constructor(
    scope: Scope,
    originNode: undefined | AnyNode,
    target: T,
    union: T,
    missing: Array<T>,
  ) {
    super(scope, originNode);
    this.target = target;
    this.union = union;
    this.missing = missing;
  }

  static type = 'MissingUnionE';
  target: T;
  union: T;
  missing: Array<T>;

  getError() {
    return {
      message: `Missing the conditions ${this.missing
        .map(type => this.utils.humanize(type))
        .join(', ')}`,
      lowerTarget: this.target,
    };
  }
}
