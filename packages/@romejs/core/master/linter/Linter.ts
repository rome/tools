/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import CompilerLinter from './CompilerLinter';
//import DeadLinter from './DeadLinter';

export default class Linter {
  constructor(req: MasterRequest) {
    this.request = req;
  }

  request: MasterRequest;

  async lint() {
    const {request} = this;
    const printer = request.createDiagnosticsPrinter({
      category: 'lint',
      message: 'Dispatched',
    });

    //const deadLinter = new DeadLinter(request, printer);
    const compilerLinter = new CompilerLinter(request, printer);

    await compilerLinter.lint();
    //await deadLinter.lint();

    throw printer;
  }
}
