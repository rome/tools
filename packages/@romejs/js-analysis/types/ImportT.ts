/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AnyNode} from '@romejs/js-ast';
import {HumanBuilder} from '../Utils';
import {HydrateData} from '../Evaluator';
import {Scope} from '../scopes';
import T from './T';

export default class ImportT extends T {
  constructor(
    scope: Scope,
    originNode: undefined | AnyNode,
    opts: {
      importedName: undefined | string;
      relative?: string;
      source: string;
    },
  ) {
    super(scope, originNode);
    this.importedName = opts.importedName;
    this.relative =
      opts.relative === undefined ? scope.evaluator.filename : opts.relative;
    this.source = opts.source;
    this.absolute = undefined;
    this.resolvedType = undefined;
    scope.evaluator.addImport(this, {
      importedName: this.importedName,
      relative: this.relative,
      source: this.source,
    });
  }

  static type = 'ImportT';
  importedName: undefined | string;
  absolute: undefined | string;
  resolvedType: undefined | T;
  relative: string;
  source: string;

  setAbsolute(absolute: undefined | string) {
    this.absolute = absolute;
  }

  setResolvedType(resolvedType: T) {
    this.resolvedType = resolvedType;
  }

  serialize(): HydrateData {
    return {
      importedName: this.importedName,
      relative: this.relative,
      source: this.source,
    };
  }

  static hydrate(
    scope: Scope,
    originNode: undefined | AnyNode,
    data: HydrateData,
  ): T {
    return new ImportT(scope, originNode, {
      importedName: String(data.importedName),
      source: String(data.source),
      relative: String(data.relative),
    });
  }

  humanize(builder: HumanBuilder): string {
    let object;
    if (this.resolvedType !== undefined) {
      object = builder.humanize(this.resolvedType);
    } else if (this.absolute === undefined) {
      object = `$Exports<"${this.source}", "${this.relative}">`;
    } else {
      object = `$Exports<"${this.absolute}">`;
    }

    if (this.importedName === undefined) {
      return object;
    } else {
      return `${object}.${this.importedName}`;
    }
  }

  reduce(): T {
    if (this.resolvedType === undefined) {
      return this;
    } else {
      return this.resolvedType;
    }
  }
}
