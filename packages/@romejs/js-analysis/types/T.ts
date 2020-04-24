/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import Evaluator, {HydrateData, HydrateTypeFactory} from '../Evaluator';
import {SourceLocation} from '@romejs/parser-core';
import {AnyNode} from '@romejs/js-ast';
import Graph from '../Graph';
import {Scope} from '../scopes';
import Hub from '../Hub';
import Utils, {HumanBuilder} from '../Utils';

let counter = 0;

export type SerialTypeFactory = (type: T) => string;

export type TypeCompatibilityReturn = {type: 'compatible'} | {
  type: 'incompatible';
  lower: T;
  upper: T;
};

export default class T {
  constructor(scope: Scope, originNode: undefined | AnyNode) {
    this.human = undefined;
    this.scope = scope;

    const {hub} = scope;
    this.hub = hub;
    this.utils = hub.utils;
    this.evaluator = hub.evaluator;
    this.originEvaluator = scope.evaluator.evaluatingType;

    // setup graph
    this.graph = scope.evaluator.graph;
    this.graph.addNode(this);

    this.originNode = originNode;
    this.originLoc = originNode === undefined ? undefined : originNode.loc;
    this.id = `${String(process.pid)}:${String(counter++)}`;

    this.compatibilityCache = new Map();
  }

  static type = 'T';
  utils: Utils;
  evaluator: Evaluator;
  graph: Graph<T>;
  scope: Scope;
  hub: Hub;

  compatibilityCache: Map<T, TypeCompatibilityReturn>;

  human: undefined | string;
  id: string;

  originNode: undefined | AnyNode;
  originLoc: undefined | SourceLocation;
  originEvaluator: undefined | string;

  getConstructor(): typeof T {
    // @ts-ignore
    return this.constructor;
  }

  setHuman(human: undefined | string) {
    this.human = human;
  }

  shouldMatch(type: T) {
    this.hub.assertOpen();
    this.graph.addLine(this, type);
  }

  hasConnections(): boolean {
    return this.graph.hasConnections(this);
  }

  explodeUnion(): Array<T> {
    return [this];
  }

  compatibleWith(otherType: T): boolean | TypeCompatibilityReturn {
    return otherType instanceof this.constructor;
  }

  clone() {
    const idsToType: Map<string, T> = new Map();

    const addType: SerialTypeFactory = (type: T) => {
      const reduced = this.utils.reduce(type);
      idsToType.set(type.id, type);
      return reduced.id;
    };

    const data = this.serialize(addType);

    const getType: HydrateTypeFactory = (id: unknown): T => {
      if (typeof id !== 'string') {
        throw new Error('Expected id to be a string');
      }

      const type = idsToType.get(id);
      if (type === undefined) {
        throw new Error('Expected type');
      }
      return type;
    };

    return this.getConstructor().hydrate(
      this.scope,
      this.originNode,
      data,
      getType,
    );
  }

  static hydrate(
    scope: Scope,
    originNode: undefined | AnyNode,
    data: HydrateData,
    getType: HydrateTypeFactory,
  ): T {
    throw new Error(`Unimplemented ${this.type}.hydrate`);
  }

  serialize(addType: SerialTypeFactory): HydrateData {
    throw new Error(
      `Unimplemented ${this.getConstructor().type}.prototype.serialize`,
    );
  }

  reduce(): T {
    return this;
  }

  humanize(builder: HumanBuilder): string {
    const reduced = this.utils.reduce(this);
    if (reduced === this) {
      throw new Error('unimplemented');
    } else {
      return builder.humanize(reduced);
    }
  }

  inspect() {
    return this.utils.inspect(this);
  }
}
