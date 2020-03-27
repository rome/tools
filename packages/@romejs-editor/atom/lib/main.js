/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

const {AutoLanguageClient} = require('atom-languageclient');

class RomeLanguageClient extends AutoLanguageClient {
  getGrammarScopes() {
    return ['source.js', 'source.ts', 'source.tsx', 'source.flow'];
  }

  getLanguageName() {
    return 'JavaScript';
  }

  getServerName() {
    return 'Rome';
  }

  getConnectionType() {
    return 'stdio';
  }

  startServerProcess() {
    return super.spawnChildNode([`/tmp/rome-dev/index.js`, 'lsp']);
  }
}

module.exports = new RomeLanguageClient();
