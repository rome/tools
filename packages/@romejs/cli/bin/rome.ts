/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {catchDiagnostics} from '@romejs/diagnostics';
import {printDiagnostics} from '@romejs/cli-diagnostics';
import {sourceMapManager, getErrorStructure} from '@romejs/v8';
import {Reporter} from '@romejs/cli-reporter';
import {VERSION, BIN, MAP} from '@romejs/core';
import cli from '../cli';
import master from '../master';
import testWorker from '../testWorker';
import worker from '../worker';
import {readFileTextSync} from '@romejs/fs';
import {SourceMapConsumer} from '@romejs/codec-source-map';

async function main(): Promise<void> {
  switch (process.env.ROME_PROCESS_VERSION === VERSION &&
    process.env.ROME_PROCESS_TYPE) {
    case 'master':
      return master();

    case 'worker':
      return worker();

    case 'test-worker':
      return testWorker();

    default:
      return cli();
  }
}

sourceMapManager.init();
sourceMapManager.addSourceMap(BIN.join(), () => SourceMapConsumer.fromJSON(
  JSON.parse(readFileTextSync(MAP)),
));

catchDiagnostics(main).then(({diagnostics}) => {
  if (diagnostics !== undefined) {
    const reporter = Reporter.fromProcess();
    printDiagnostics({
      diagnostics,
      suppressions: [],
      printerOptions: {
        reporter,
      },
    });
    process.exit(1);
  }
}).catch((err: Error) => {
  console.error('Error thrown inside the CLI handler');
  console.error(getErrorStructure(err).stack);
});
