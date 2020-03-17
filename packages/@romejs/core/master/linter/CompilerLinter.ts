/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {DiagnosticsPrinter} from '@romejs/cli-diagnostics';
import {AbsoluteFilePathSet} from '@romejs/path';

export default class CompilerLinter {
  constructor(req: MasterRequest, printer: DiagnosticsPrinter, fix: boolean) {
    this.request = req;
    this.fix = fix;
    this.printer = printer;
  }

  fix: boolean;
  request: MasterRequest;
  printer: DiagnosticsPrinter;

  async lint(paths: AbsoluteFilePathSet) {
    const {request, printer} = this;
    const {master, reporter} = request;

    const pathsByWorker = await master.fileAllocator.groupPathsByWorker(paths);

    const spinner = reporter.progress();
    spinner.setTitle('Linting');
    spinner.setTotal(paths.size);

    await Promise.all(
      pathsByWorker.map(async paths => {
        for (const path of paths) {
          const text = `<filelink target="${path.join()}" />`;
          spinner.pushText(text);

          const {
            diagnostics,
            suppressions,
          } = await this.request.requestWorkerLint(path, this.fix);
          printer.processor.addSuppressions(suppressions);
          printer.addDiagnostics(diagnostics);

          spinner.popText(text);
          spinner.tick();
        }
      }),
    );

    spinner.end();
  }
}
