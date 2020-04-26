/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {AbsoluteFilePath} from '@romejs/path';
import {exists} from '@romejs/fs';
import childProcess = require('child_process');

const TIMEOUT = 10_000;

async function exec(command: string, args: Array<string>): Promise<string> {
  return new Promise((resolve, reject) => {
    const proc = childProcess.spawn(command, args, {timeout: TIMEOUT});
    let stderr = '';
    let stdout = '';

    proc.stdout.on(
      'data',
      (data) => {
        stdout += data;
      },
    );

    proc.stderr.on(
      'data',
      (data) => {
        stderr += data;
      },
    );

    function error(message: string) {
      reject(
        new Error(
          `Error while running ${command} ${args.join(' ')}: ${message}. stderr: ${stderr}`,
        ),
      );
    }

    proc.on(
      'error',
      (err: NodeJS.ErrnoException) => {
        if (err.code === 'ETIMEDOUT') {
          error(`Timed out after ${TIMEOUT}ms`);
        } else {
          error(err.message);
        }
      },
    );

    proc.on(
      'close',
      (code) => {
        if (code === 0) {
          resolve(stdout);
        } else {
          error(`Exited with code ${code}`);
        }
      },
    );
  });
}

function extractFileList(out: string): Array<string> {
  const lines = out.trim().split('\n');

  const files: Array<string> = [];

  for (const line of lines) {
    const match = line.trim().match(/^(?:[AM]|\?\?)\s+(.*?)$/);
    if (match != null) {
      files.push(match[1]);
    }
  }

  return files;
}

export class VCSClient {
  constructor(root: AbsoluteFilePath) {
    this.root = root;
    this.trunkBranch = 'unknown';
  }

  root: AbsoluteFilePath;
  trunkBranch: string;

  getModifiedFiles(branch: string): Promise<Array<string>> {
    throw new Error('unimplemented');
  }

  getUncommittedFiles(): Promise<Array<string>> {
    throw new Error('unimplemented');
  }
}

class GitVCSClient extends VCSClient {
  constructor(root: AbsoluteFilePath) {
    super(root);
    this.trunkBranch = 'master';
  }

  async getUncommittedFiles(): Promise<Array<string>> {
    const out = await exec('git', ['status', '--short']);
    return extractFileList(out);
  }

  async getModifiedFiles(branch: string): Promise<Array<string>> {
    const out = await exec('git', ['diff', '--name-status', branch]);
    return extractFileList(out);
  }
}

export async function getVCSClient(
  root: AbsoluteFilePath,
): Promise<undefined | VCSClient> {
  if (await exists(root.append('.git'))) {
    return new GitVCSClient(root);
  }

  return undefined;
}
