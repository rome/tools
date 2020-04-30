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

type State = {
  flat: boolean;
  indent: Box<number>;
  pendingSpaces: Box<number>;
  buffer: StringBuffer;
  mappings: Array<Mapping>;
  lineSuffixes: Array<[Token, State]>;
};

class BreakError extends Error {
  constructor() {
    super(
      "This error represents a point in the formatter where we should line break. If you're seeing this something went wrong.",
    );
  }
}

class Box<T> {
  constructor(value: T) {
    this.value = value;
  }

  value: T;
}

function forkState(parent: State, callback: (state: State) => void): void {
  const mappingsLength = parent.mappings.length;
  const state: State = {
    ...parent,
    buffer: StringBuffer.from(parent.buffer),
    pendingSpaces: new Box(parent.pendingSpaces.value),
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
  parent.pendingSpaces.value = state.pendingSpaces.value;
}

function print(token: Token, state: State, options: PrinterOptions): void {
  const stack: Array<[Token, State]> = [[token, state]];

  while (stack.length > 0) {
    const [token, state] = stack.pop()!;

    if (typeof token === 'string') {
      if (token !== '') {
        if (ob1Get0(state.buffer.column) === 0) {
          state.buffer.push(' '.repeat(state.indent.value));
        } else {
          if (state.pendingSpaces.value > 0) {
            state.buffer.push(' '.repeat(state.pendingSpaces.value));
            state.pendingSpaces.value = 0;
          }
        }

        state.buffer.push(token);

        // If the line is too long, break the group if it is possible
        if (state.flat && ob1Get0(state.buffer.column) > options.printWidth) {
          throw new BreakError();
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
          if (token.shouldBreak) {
            if (state.flat) {
              throw new BreakError();
            } else {
              stack.push([token.contents, state]);
              break;
            }
          }

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
              if (err instanceof BreakError) {
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
            {
              ...state,
              indent: new Box(state.indent.value + options.indentWidth),
            },
          ]);
          break;
        }

        case 'Line': {
          if (state.flat) {
            switch (token.mode) {
              case 'space': {
                state.pendingSpaces.value++;
                break;
              }

              case 'soft':
                // Soft lines are not printed in flat mode.
                break;

              case 'hard':
                // Hard lines are always printed.
                // In flat mode, the current group be broken.
                throw new BreakError();
            }
          } else {
            if (state.lineSuffixes.length > 0) {
              stack.push([token, state]);
              while (state.lineSuffixes.length > 0) {
                stack.push(state.lineSuffixes.pop()!);
              }
              break;
            }

            state.buffer.push('\n');
            state.pendingSpaces.value = 0;
          }
          break;
        }

        case 'LineSuffix': {
          if (state.flat) {
            throw new BreakError();
          } else {
            state.lineSuffixes.push([token.contents, state]);
          }
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
          state.pendingSpaces.value++;
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
    flat: false,
    indent: new Box(options.rootIndent * options.indentWidth),
    pendingSpaces: new Box(0),
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
