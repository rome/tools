/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'prefer ES6 class',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        `var Hello = createReactClass({
          render: function() {
            return <div>Hello {this.props.name}</div>;
          }
        });`,
        `createReactClass({
          render: function() {
            return <div>Hello {this.props.name}</div>;
          }
        });`,
        `const Hello = createReactClass({
          render: function() {
            return <div>Hello {this.props.name}</div>;
          }
        });`,
        `let Hello = createReactClass({
          render: function() {
            return <div>Hello {this.props.name}</div>;
          }
        });`,
        // VALID
        `class Hello extends React.Component {
          render() {
            return <div>Hello {this.props.name}</div>;
          }
        }`
      ],
      {category: 'lint/preferES6Class'},
    );
  },
);
