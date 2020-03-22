/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {MasterRequest} from '@romejs/core';
import {createMasterCommand} from '../../commands';
import {commandCategories} from '../../commands';
import {DiagnosticsProcessor, descriptions} from '@romejs/diagnostics';
import {FORMATTABLE_EXTENSIONS} from '@romejs/core/common/fileHandlers';
import {Consumer} from '@romejs/consume';

type Flags = {write: boolean};

export default createMasterCommand({
  category: commandCategories.INTERNAL,
  description: 'TODO',

  defineFlags(consumer: Consumer): Flags {
    return {
      write: consumer.get('write').asBoolean(false),
    };
  },

  async default(req: MasterRequest, flags: Flags): Promise<void> {
    const {reporter, master} = req;

    const {paths} = await req.getFilesFromArgs({
      getProjectIgnore: (project) =>
        ({
          patterns: project.config.format.ignore,
          source: master.projectManager.findProjectConfigConsumer(project, (
            consumer,
          ) =>
            consumer.has('format') && consumer.get('format').has('ignore')
              ? consumer.get('format').get('ignore') : undefined
          ),
        }),
      getProjectEnabled: (project) =>
        ({
          enabled: project.config.format.enabled,
          source: master.projectManager.findProjectConfigConsumer(project, (
            consumer,
          ) =>
            consumer.has('format')
              ? consumer.get('format').get('enabled') : undefined
          ),
        }),
      noun: 'formatting',
      verb: 'linting',
      configCategory: 'format',
      extensions: FORMATTABLE_EXTENSIONS,
      disabledDiagnosticCategory: 'format/disabled',
    });

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
    await Promise.all(pathsByWorker.map(async (paths) => {
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

        if (!flags.write && res.formatted !== res.original) {
          diagnosticsProcessor.addDiagnostic({
            description: descriptions.LINT.PENDING_FIXES(
              res.original,
              res.formatted,
            ),
            location: {
              filename: path.join(),
            },
          });
        } else {
          //await writeFile(path, res.formatted);
        }
      }
    }));

    diagnosticsProcessor.maybeThrowDiagnosticsError();

    if (flags.write) {
      reporter.success(
        `<number emphasis>${paths.size}</number> files formatted successfully`,
      );
    } else {
      reporter.success(
        `<number emphasis>${paths.size}</number> files formatted correctly`,
      );
    }
  },
});
