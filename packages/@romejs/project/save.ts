/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {ProjectConfigMeta, ProjectConfigMetaHard} from './types';
import {Consumer} from '@romejs/consume';
import {readFileText, writeFile} from '@romejs/fs';
import {consumeJSONExtra, stringifyRJSONFromConsumer} from '@romejs/codec-json';
import {normalizeProjectConfig} from './load';
import {DiagnosticsError, catchDiagnosticsSync} from '@romejs/diagnostics';
import {assertHardMeta} from './utils';

export async function modifyProjectConfig(
  softMeta: ProjectConfigMeta,
  callbacks: {
    pre: (meta: ProjectConfigMetaHard) => void;
    modify: (consumer: Consumer) => void;
  },
) {
  const meta = assertHardMeta(softMeta);
  const {configPath, configSourceSubKey: consumerSubKey} = meta;

  await callbacks.pre(meta);

  // Load the config file again
  const configFile = await readFileText(configPath);
  const res = consumeJSONExtra({
    path: configPath,
    input: configFile,
  });

  const {consumer} = res;
  if (consumerSubKey === undefined) {
    await callbacks.modify(consumer);
  } else {
    await callbacks.modify(consumer.get(consumerSubKey));
  }

  // Stringify the config
  let stringified: string;
  if (res.hasExtensions) {
    stringified = stringifyRJSONFromConsumer(res);
  } else {
    stringified = JSON.stringify(consumer.asUnknown(), null, '  ');
  }

  // Test if this project config doesn't result in errors
  let {diagnostics} = catchDiagnosticsSync(() => {
    // Reconsume with new stringified config
    const res = consumeJSONExtra({
      path: configPath,
      input: stringified,
    });

    // Validate the new config
    normalizeProjectConfig(res, configPath, stringified, meta.projectFolder);
  });

  if (diagnostics !== undefined) {
    // Set the `code` property on relevant diagnostics since our changes don't exist on disk
    diagnostics = diagnostics.map((diag) => {
      return diag.location.filename === configPath.join()
        ? {
            ...diag,
            location: {
              ...diag.location,
              sourceText: stringified,
            },
          }
        : diag;
    });

    throw new DiagnosticsError(
      'Diagnostics produced while testing new project config',
      diagnostics,
    );
  }

  // Write it out
  await writeFile(configPath, stringified);
}
