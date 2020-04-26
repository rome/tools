/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HydrateData} from '../Evaluator';
import {AnyNode} from '@romejs/js-ast';
import {Scope} from '../scopes';
import ObjT from './ObjT';
import T from './T';

export default class StringLiteralT extends ObjT {
  constructor(scope: Scope, originNode: undefined | AnyNode, value: string) {
    super(
      scope,
      originNode,
      {
        props: [],
        proto: scope.intrinsics.StringPrototype,
        calls: [],
      },
    );
    this.value = value;
  }

  static type = 'StringLiteralT';
  value: string;

  serialize(): HydrateData {
    return {value: this.value};
  }

  static hydrate(
    scope: Scope,
    originNode: undefined | AnyNode,
    data: HydrateData,
  ): T {
    return new StringLiteralT(scope, originNode, String(data.value));
  }

  humanize(): string {
    let str: string = JSON.stringify(this.value);
    if (this.value.includes("'")) {
      return str;
    } else {
      return `'${str.slice(1, -1)}'`;
    }
  }

  compatibleWith(type: T): boolean {
    return type instanceof StringLiteralT && type.value === this.value;
  }
}
