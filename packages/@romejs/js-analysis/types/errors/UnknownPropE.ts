/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {buildSuggestionAdvice} from '@romejs/diagnostics';
import {Scope} from '../../scopes';
import T from '../T';
import {orderBySimilarity} from '@romejs/string-utils';
import E, {ErrorDefinition} from './E';
import {AnyNode} from '@romejs/js-ast';

export default class UnknownPropE extends E {
  constructor(
    scope: Scope,
    originNode: undefined | AnyNode,
    opts: {
      object: T;
      property: T;
      key: string;
      thisKeys: Array<string>;
      protoKeys: Array<string>;
    },
  ) {
    super(scope, originNode);
    this.thisKeys = opts.thisKeys;
    this.protoKeys = opts.protoKeys;
    this.allProps = [...this.thisKeys, ...this.protoKeys];
    this.key = opts.key;
    this.object = opts.object;
    this.property = opts.property;
  }

  static type = 'UnknownPropE';
  allProps: Array<string>;
  thisKeys: Array<string>;
  protoKeys: Array<string>;
  property: T;
  object: T;
  key: string;

  sortProps(props: Array<string>): Array<string> {
    if (props.length === 0) {
      return props;
    }

    const ratings = orderBySimilarity(this.key, props);
    const sortedProps = ratings.map((prop) => prop.target);
    return sortedProps;
  }

  getError(): ErrorDefinition {
    let message: string = `Property '${this.key}' not found in`;

    return {
      category: 'typeCheck/unknownProperty',
      message,
      advice: buildSuggestionAdvice(this.key, this.allProps),
      lowerTarget: this.property,
      upperTarget: this.object,
    };
  }
}
