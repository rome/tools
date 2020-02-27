/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {HydrateData} from '../Evaluator';
import {Scope} from '../scopes';
import NumericLiteralT from './NumericLiteralT';
import ObjT from './ObjT';
import T from './T';

export default class NumericT extends ObjT {
  constructor(scope: Scope, originNode: undefined | AnyNode) {
    super(scope, originNode, {
      props: [],
      proto: scope.intrinsics.NumberPrototype,
      calls: [],
    });
  }

  static type = 'NumericT';

  serialize(): HydrateData {
    return {};
  }

  static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
    return new NumericT(scope, originNode);
  }

  humanize(): string {
    return 'number';
  }

  compatibleWith(type: T): boolean {
    // a numeric literal can flow into a generic number
    return type instanceof NumericT || type instanceof NumericLiteralT;
  }
}
