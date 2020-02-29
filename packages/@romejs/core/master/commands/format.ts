/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand} from '../../commands';
import {commandCategories} from '../../commands';
import {AbsoluteFilePathSet} from '@romejs/path';
import {DiagnosticsProcessor} from '@romejs/diagnostics';
import {FORMATTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import Linter from '../linter/Linter';

export default createMasterCommand({
  category: commandCategories.INTERNAL,
  description: '',

  async default(req: MasterRequest): Promise<void> {
    const {reporter} = req;

    const paths: AbsoluteFilePathSet = await Linter.getFilesFromArgs(
      req,
      FORMATTABLE_EXTENSIONS,
    );
    if (paths.size === 0) {
      reporter.warn('No files formatted');
      return;
    }

    const pathsByWorker = await req.master.fileAllocator.groupPathsByWorker(
      paths,
    );

    const progress = reporter.progress();
    progress.setTotal(paths.size);
    progress.setTitle('Formatting');

    const diagnosticsProcessor = new DiagnosticsProcessor({});

    // TODO probably add the same logic in CompilerLinter if the project config disables formatting

    await Promise.all(
      pathsByWorker.map(async paths => {
        for (const path of paths) {
          progress.setText(`<filelink target="${path.join()}" />`);
          progress.tick();

          const res = await req.requestWorkerFormat(path);
          if (res === undefined) {
            continue;
          }

          if (res.diagnostics.length > 0) {
            diagnosticsProcessor.addDiagnostics(res.diagnostics);
            continue;
          }

          //await writeFile(path, res.formatted);
        }
      }),
    );

    diagnosticsProcessor.maybeThrowDiagnosticsError();
    reporter.success(
      `<number>${paths.size}</number> files formatted successfully`,
    );
  },
});
