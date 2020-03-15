/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {HumanBuilder} from '../Utils';
import {HydrateTypeFactory, HydrateData} from '../Evaluator';
import T, {SerialTypeFactory, TypeCompatibilityReturn} from './T';
import {Scope} from '../scopes';
import {AnyNode} from '@romejs/js-ast';

export default class ObjPropT extends T {
  constructor(
    scope: Scope,
    originNode: undefined | AnyNode,
    key: string,
    value: T,
  ) {
    super(scope, originNode);
    this.key = key;
    this.value = value;
  }

  static type = 'ObjPropT';
  key: string;
  value: T;

  serialize(addType: SerialTypeFactory): HydrateData {
    return {
      key: this.key,
      value: addType(this.value),
    };
  }

  static hydrate(
    scope: Scope,
    originNode: AnyNode,
    data: HydrateData,
    getType: HydrateTypeFactory,
  ): T {
    return new ObjPropT(
      scope,
      originNode,
      String(data.key),
      getType(data.value),
    );
  }

  compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
    if (otherType instanceof ObjPropT && otherType.key === this.key) {
      return this.utils.checkCompability(this.value, otherType.value);
    } else {
      return false;
    }
  }

  humanize(builder: HumanBuilder): string {
    return `${this.key}: ${builder.humanize(this.value)}`;
  }
}
