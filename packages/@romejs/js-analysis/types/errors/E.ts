/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from '../../scopes';
import {
  PartialDiagnosticAdviceItem,
  DiagnosticCategory,
} from '@romejs/diagnostics';
import {AnyNode} from '@romejs/js-ast';
import AnyT from '../AnyT';
import T from '../T';

export type ErrorDefinition = {
  category: DiagnosticCategory;
  advice?: Array<PartialDiagnosticAdviceItem>;
  message: string;
  lowerTarget: T;
  upperTarget?: T;
};

export default class E extends T {
  static type = 'E';

  static hydrate(scope: Scope, originNode: undefined | AnyNode): T {
    return new AnyT(scope, originNode);
  }

  humanize(): string {
    return this.getError().message;
  }

  getError(): ErrorDefinition {
    throw new Error('unimplemented');
  }

  compatibleWith(): boolean {
    return false;
  }
}
