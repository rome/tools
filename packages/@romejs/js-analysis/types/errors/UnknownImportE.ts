/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {PartialDiagnosticAdvice} from '@romejs/diagnostics';
import {Scope} from '../../scopes';
import {findClosestStringMatch} from '@romejs/string-utils';
import E from './E';
import {AnyNode} from '@romejs/js-ast';

export default class UnknownImportE extends E {
  constructor(
    scope: Scope,
    originNode: undefined | AnyNode,
    opts: {
      possibleNames: Array<string>;
      importedName: string;
      source: string;
    },
  ) {
    super(scope, originNode);
    this.possibleNames = opts.possibleNames;
    this.importedName = opts.importedName;
    this.source = opts.source;
  }

  static type = 'UnknownImportE';
  importedName: string;
  source: string;
  possibleNames: Array<string>;

  getError() {
    const {possibleNames} = this;
    const suggestion = findClosestStringMatch(this.importedName, possibleNames);

    let infoMessage = undefined;
    let infoList = undefined;

    if (possibleNames.length === 0) {
      infoMessage = 'This file contains no exports.';
    } else {
      if (suggestion === undefined) {
        infoList = this.possibleNames;
        infoMessage = 'Did you mean one of these?';
      } else {
        infoMessage = `Did you mean <emphasis>${suggestion}</emphasis>?`;
      }
    }

    const advice: PartialDiagnosticAdvice = [
      {
        type: 'log',
        category: 'info',
        message: infoMessage,
      },
    ];
    if (infoList !== undefined) {
      advice.push({
        type: 'list',
        list: infoList,
      });
    }

    return {
      message: `Unknown import '${this.importedName}' in '${this.source}'`,
      advice,
      lowerTarget: this,
    };
  }
}
