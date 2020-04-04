/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Mapping} from '@romejs/codec-source-map';
import {ob1Get0} from '@romejs/ob1';
import StringBuffer from './StringBuffer';
import {Token} from './tokens';

export type PrinterOptions = {
  indentWidth: number;
  printWidth: number;
  rootIndent: number;
};

export type PrinterOutput = {
  code: string;
  mappings: Array<Mapping>;
};

type Indent = {
  depth: number;
  value: string;
};

type State = {
  indent: Indent;
  flat: boolean;
  buffer: StringBuffer;
  mappings: Array<Mapping>;
  lineSuffixes: Array<[Token, State]>;
};

const BREAK_GROUP = Symbol('BREAK_GROUP');

function createIndent(depth: number, options: PrinterOptions): Indent {
  return {
    depth,
    value: ' '.repeat(depth * options.indentWidth),
  };
}

function forkState(parent: State, callback: (state: State) => void): void {
  const mappingsLength = parent.mappings.length;
  const state: State = {
    ...parent,
    buffer: StringBuffer.from(parent.buffer),
  };

  try {
    callback(state);
  } catch (err) {
    // Discard invalid mappings
    if (parent.mappings.length !== mappingsLength) {
      parent.mappings.length = mappingsLength;
    }

    throw err;
  }

  // Merge the states together
  parent.buffer.merge(state.buffer);
}

function print(token: Token, state: State, options: PrinterOptions): void {
  const stack: Array<[Token, State]> = [[token, state]];

  while (stack.length > 0) {
    const [token, state] = stack.pop()!;

    if (typeof token === 'string') {
      if (token !== '') {
        state.buffer.push(token);

        // If the line is too long, break the group if it is possible
        if (state.flat && ob1Get0(state.buffer.column) > options.printWidth) {
          throw BREAK_GROUP;
        }
      }
    } else {
      switch (token.type) {
        case 'Comment': {
          stack.push([token.value, state]);
          break;
        }

        case 'Concat': {
          for (let i = token.parts.length - 1; i >= 0; i--) {
            stack.push([token.parts[i], state]);
          }
          break;
        }

        case 'Group': {
          if (state.flat) {
            stack.push([token.contents, state]);
          } else {
            try {
              forkState(
                state,
                (next) => {
                  // Try to print the group contents on a single line.
                  // If it fails, break the group.
                  next.flat = true;
                  print(token.contents, next, options);
                },
              );
            } catch (err) {
              if (err === BREAK_GROUP) {
                stack.push([token.contents, state]);
              } else {
                // This should not happen!
                // Let the error propagate.
                throw err;
              }
            }
          }
          break;
        }

        case 'IfBreak': {
          if (state.flat) {
            if (token.flatContents) {
              stack.push([token.flatContents, state]);
            }
          } else {
            stack.push([token.breakContents, state]);
          }
          break;
        }

        case 'Indent': {
          stack.push([
            token.contents,
            {...state, indent: createIndent(state.indent.depth + 1, options)},
          ]);
          break;
        }

        case 'Line': {
          if (state.flat) {
            switch (token.mode) {
              case 'space': {
                state.buffer.push(' ');
                break;
              }

              case 'soft':
                // Soft lines are not printed in flat mode.
                break;

              case 'hard':
                // Hard lines are always printed.
                // In flat mode, the current group be broken.
                throw BREAK_GROUP;
            }
          } else {
            if (state.lineSuffixes.length > 0) {
              stack.push([token, state]);
              while (state.lineSuffixes.length > 0) {
                stack.push(state.lineSuffixes.pop()!);
              }
              break;
            }

            state.buffer.trim();
            state.buffer.push('\n');
            state.buffer.push(state.indent.value);
          }
          break;
        }

        case 'LineSuffix': {
          state.lineSuffixes.push([token.contents, state]);
          break;
        }

        case 'PositionMarker': {
          if (
            state.mappings.length > 0 &&
            state.mappings[state.mappings.length - 1].generated.index ===
            state.buffer.index
          ) {
            break;
          }

          state.mappings.push({
            generated: {
              line: state.buffer.line,
              column: state.buffer.column,
              index: state.buffer.index,
            },
            original: {
              line: token.loc[token.prop].line,
              column: token.loc[token.prop].column,
            },
            name: token.loc.identifierName,
            source: token.loc.filename,
          });
          break;
        }

        case 'Space': {
          state.buffer.push(' ');
          break;
        }
      }
    }
  }
}

export function printTokenToString(
  token: Token,
  options: PrinterOptions,
): PrinterOutput {
  const state: State = {
    indent: createIndent(options.rootIndent, options),
    flat: false,
    buffer: new StringBuffer(),
    mappings: [],
    lineSuffixes: [],
  };

  print(token, state, options);

  return {
    code: state.buffer.toString(),
    mappings: state.mappings,
  };
}
