/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode, MOCK_PARENT} from '@romejs/js-ast';
import {
  CompilerContext,
  Scope,
  TransformVisitor,
  TransformVisitors,
} from '@romejs/js-compiler';
import {
  AnyHookDescriptor,
  HookDescriptor,
  HookInstance,
} from '../api/createHook';
import reduce from '../methods/reduce';
import {TransformExitResult} from '../types';

export type PathOptions = {
  ancestryPaths?: Array<Path>;
  nodeKey?: string;
  listKey?: number;
  parentScope?: Scope;
  scope?: Scope;
  noArrays?: boolean;
  noScopeCreation?: boolean;
  hooks?: Handlers;
  isMock?: boolean;
};

type Handlers = Array<HookInstance>;

export default class Path {
  constructor(node: AnyNode, context: CompilerContext, opts: PathOptions) {
    const ancestryPaths = opts.ancestryPaths || [];
    this.ancestryPaths = ancestryPaths;

    if (node === MOCK_PARENT) {
      this.parentPath = this;
    } else if (ancestryPaths.length === 0) {
      this.parentPath = new Path(MOCK_PARENT, context, {
        isMock: true,
      });
    } else {
      this.parentPath = ancestryPaths[0];
    }

    this.node = node;
    this.parent = this.parentPath.node;
    this.context = context;

    const parentScope = opts.parentScope === undefined
      ? context.getRootScope()
      : opts.parentScope;

    let scope = opts.scope;
    if (scope === undefined) {
      if (opts.noScopeCreation === true) {
        scope = parentScope;
      } else {
        scope = parentScope.evaluate(node, this.parent, true);
      }
    }
    this.scope = scope;

    this.nodeKey = opts.nodeKey;
    this.listKey = opts.listKey;

    this.isMock = opts.isMock === true;
    this.opts = opts;

    this.hooks = opts.hooks === undefined ? [] : opts.hooks;
  }

  context: CompilerContext;
  node: AnyNode;
  parent: AnyNode;
  scope: Scope;
  hooks: Handlers;
  opts: PathOptions;
  isMock: boolean;

  ancestryPaths: Array<Path>;
  parentPath: Path;

  nodeKey: undefined | string;
  listKey: undefined | number;

  callHook<CallArg, CallReturn>(
    // rome-suppress-next-line lint/noExplicitAny
    descriptor: HookDescriptor<any, CallArg, CallReturn>,
    arg: CallArg,
    optionalRet?: CallReturn,
    requiredDepth?: number,
  ): CallReturn {
    const hook = this.findHook(descriptor, requiredDepth);
    if (hook === undefined) {
      if (optionalRet === undefined) {
        throw new Error(`No ${descriptor.name} hook found`);
      } else {
        return optionalRet;
      }
    }
    if (descriptor.call === undefined) {
      throw new Error("Hook doesn't have a call method");
    }

    const {depth, ref} = hook;
    const {state, value, bubble} = descriptor.call(this, ref.state, arg);
    ref.state = state;

    if (bubble === true) {
      return this.callHook(descriptor, arg, value, depth + 1);
    } else {
      return value;
    }
  }

  provideHook<State>( // rome-suppress-next-line lint/noExplicitAny
  descriptor: HookDescriptor<State, any, any>, state?: State): AnyNode {
    this.hooks.push({
      state: {
        ...descriptor.initialState,
        ...state,
      },
      descriptor,
    });

    return this.node;
  }

  findHook(
    descriptor: AnyHookDescriptor,
    requiredDepth: number = 0,
  ): undefined | {
    ref: HookInstance;
    depth: number;
  } {
    let depth = 0;
    for (const {hooks} of this.ancestryPaths) {
      for (const hook of hooks) {
        if (hook.descriptor === descriptor) {
          if (depth === requiredDepth) {
            return {ref: hook, depth};
          } else {
            depth++;
          }
        }
      }
    }
    return undefined;
  }

  findAncestry(callback: (path: Path) => boolean): undefined | Path {
    for (const path of this.ancestryPaths) {
      if (callback(path)) {
        return path;
      }
    }
    return undefined;
  }

  getChildPath(key: string): Path {
    // rome-suppress-next-line lint/noExplicitAny
    const node = (this.node as any)[key];
    if (node === undefined) {
      throw new Error(
        `Attempted to get child path for ${key} but no such node existed`,
      );
    }

    return new Path(node, this.context, {
      parentScope: this.scope,
      ancestryPaths: this.ancestryPaths.concat([this]),
      nodeKey: key,
    });
  }

  getChildPaths(key: string): Array<Path> {
    // rome-suppress-next-line lint/noExplicitAny
    const nodes = (this.node as any)[key];

    if (nodes === undefined) {
      throw new Error(
        `Attempted to get child paths for ${key} but no such node existed`,
      );
    }

    if (!Array.isArray(nodes)) {
      throw new Error(`Expected child nodes for ${key} to be an array`);
    }

    const ancestryPaths = this.ancestryPaths.concat([this]);

    return nodes.map((node: AnyNode, i: number) => {
      return new Path(node, this.context, {
        parentScope: this.scope,
        ancestryPaths,
        listKey: i,
        nodeKey: key,
      });
    });
  }

  getPathKeys(): Array<string> {
    const parts = [];

    let path: undefined | Path = this;
    while (path !== undefined && !path.isMock) {
      if (path.listKey !== undefined) {
        parts.push(String(path.listKey));
      }
      if (path.nodeKey !== undefined) {
        parts.push(path.nodeKey);
      }
      path = path.parentPath;
    }

    return parts.reverse();
  }

  fork(newNode: AnyNode): Path {
    return new Path(newNode, this.context, this.getPathOptions());
  }

  getPathOptions(): PathOptions {
    return {
        ...this.opts,
        hooks: this.hooks,
        parentScope: this.scope === undefined
          ? undefined
          : this.scope.parentScope,
      };
  }

  traverse(name: string, callback: (path: Path) => void) {
    this.reduce({
      name,
      enter(path: Path) {
        callback(path);
        return path.node;
      },
    });
  }

  reduce(
    visitors: TransformVisitor | TransformVisitors,
    opts?: Partial<PathOptions>,
  ): TransformExitResult {
    return reduce(
      this.node,
      Array.isArray(visitors) ? visitors : [visitors],
      this.context,
      {...this.getPathOptions(), ...opts},
    );
  }
}
