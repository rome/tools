/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

require('../_setup.cjs');

const path = require('path');
const fs = require('fs');

const {readGeneratedFile, write, getBuilderName} = require('../_utils.cjs');

const {
  formatterFolder,
  analysisFolder,
  astFolder,
} = require('../_constants.cjs');

let defs = [];

for (const category of fs.readdirSync(astFolder)) {
  const loc = path.join(astFolder, category);

  if (fs.statSync(loc).isFile()) {
    continue;
  }

  for (const basename of fs.readdirSync(loc)) {
    const nodeType = path.basename(basename, path.extname(basename));
    defs.push({
      category,
      builderName: getBuilderName(nodeType),
      nodeType,
    });
  }
}

defs = defs.sort((a, b) => {
  return a.nodeType.localeCompare(b.nodeType);
});

function readIndexFile(loc, handlers) {
  let file = readGeneratedFile(loc);

  for (const {iterator, wrapCallback} of handlers) {
    let buff = '';

    for (const def of defs) {
      const defBuff = iterator(def);
      if (defBuff) {
        buff += defBuff;
      }
    }

    if (wrapCallback) {
      buff = wrapCallback(buff);
    }

    file += buff;

    file = file.trim();
    file += '\n\n';
  }

  file = file.trim();
  file += '\n';

  write(loc, file);
}

// Add to ast index
readIndexFile(
  path.join(astFolder, 'index.ts'),
  [
    {
      iterator({category, nodeType}) {
        return `export * from './${category}/${nodeType}';\n`;
      },
    },
  ],
);

// Add to builders
readIndexFile(
  path.join(formatterFolder, 'index.ts'),
  [
    {
      iterator({category, nodeType}) {
        return `import ${nodeType} from './${category}/${nodeType}';\nbuilders.set('${nodeType}', ${nodeType});\n\n`;
      },
    },
  ],
);

// Add to analysis
readIndexFile(
  path.join(analysisFolder, 'index.ts'),
  [
    {
      iterator({category, nodeType}) {
        return `import ${nodeType} from './${category}/${nodeType}';\nevaluators.set('${nodeType}', ${nodeType});\n\n`;
      },
    },
  ],
);

// Update unions.ts
const unionsLoc = path.join(astFolder, 'unions.ts');
readIndexFile(
  unionsLoc,
  [
    /*{
    iterator(def) {
      if (def.category === 'typescript') {
        return `\n  | n.${def.nodeType}`;
      }
    },
    wrapCallback(buff) {
      return `export type AnyTS = ${buff};`;
    },
  },
  {
    iterator(def) {
      if (def.category === 'flow') {
        return `\n  | n.${def.nodeType}`;
      }
    },
    wrapCallback(buff) {
      return `export type AnyFlow = ${buff};`;
    },
  },*/
    {
      iterator(def) {
        return `\n  | n.${def.nodeType}`;
      },
      wrapCallback(buff) {
        return `export type AnyNode = ${buff};`;
      },
    },
  ],
);
