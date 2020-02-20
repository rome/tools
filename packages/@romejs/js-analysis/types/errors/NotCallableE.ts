/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {Scope} from '../../scopes';
import E from './E';
import T from '../T';

export default class NotCallableE extends E {
  constructor(scope: Scope, originNode: undefined | AnyNode, callee: T) {
    super(scope, originNode);
    this.callee = callee;
  }

  static type = 'NotCallableE';
  callee: T;

  getError() {
    return {
      message: `This type isn't callable`,
      lowerTarget: this.callee,
    };
  }
}
