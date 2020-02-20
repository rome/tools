/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {HydrateTypeFactory, HydrateData} from '../Evaluator';
import {SerialTypeFactory} from './T';
import {Scope} from '../scopes';
import {HumanBuilder} from '../Utils';
import T from './T';

export default class UnionT extends T {
  constructor(scope: Scope, originNode: undefined | AnyNode, types: Array<T>) {
    super(scope, originNode);
    this.types = [...new Set(types)];
  }

  static type = 'UnionT';
  types: Array<T>;

  serialize(addType: SerialTypeFactory): HydrateData {
    return {
      types: this.types.map(type => addType(type)),
    };
  }

  static hydrate(
    scope: Scope,
    originNode: AnyNode,
    data: HydrateData,
    getType: HydrateTypeFactory,
  ): T {
    return new UnionT(
      scope,
      originNode,
      Array(data.types).map(id => getType(id)),
    );
  }

  reduce(): T {
    const uniqTypes = [];
    const types = this.explodeUnion();

    for (const type of types) {
      let foundMatch = false;
      for (const compareType of uniqTypes) {
        const isCompatible = this.utils.isCompatibleWith(compareType, type);
        if (isCompatible) {
          foundMatch = true;
          break;
        }
      }
      if (foundMatch === false) {
        uniqTypes.push(type);
      }
    }

    if (uniqTypes.length === types.length) {
      return this;
    } else if (uniqTypes.length === 1) {
      return uniqTypes[0];
    } else {
      return new UnionT(this.scope, this.originNode, uniqTypes);
    }
  }

  explodeUnion(): Array<T> {
    let types: Array<T> = [];
    const visited: Set<T> = new Set([this]);

    for (const type of this.types) {
      const reduced = this.utils.reduce(type);
      if (visited.has(reduced)) {
        continue;
      } else {
        visited.add(reduced);
      }

      types = types.concat(this.utils.explodeUnion(type));
    }

    return types;
  }

  compatibleWith(otherType: T) {
    const ourTypes = this.utils.explodeUnion(this);

    // fast path to check if a union contains a type
    if (ourTypes.includes(otherType)) {
      return true;
    }

    const otherTypes = this.utils.explodeUnion(otherType);
    const missing: Array<T> = [];

    for (const type of ourTypes) {
      let compatible = false;

      for (const otherType of otherTypes) {
        if (this.utils.isCompatibleWith(type, otherType)) {
          compatible = true;
        }
      }

      if (compatible === false) {
        missing.push(type);
      }
    }

    if (missing.length === 0) {
      return true;
    } else {
      // create custom error with the types that weren't in the opposing one
      //return new MissingUnionE(this.scope, otherType.originNode, otherType, this, missing);
      return false;
    }
  }

  humanize(builder: HumanBuilder): string {
    return this.types.map(type => builder.humanize(type)).join(' | ');
  }
}
