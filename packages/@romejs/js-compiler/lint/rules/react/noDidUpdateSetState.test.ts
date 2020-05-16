/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from 'rome';
import {testLintMultiple} from '../testHelpers';

test(
  'no did update set state',
  async (t) => {
    await testLintMultiple(
      t,
      [
        // INVALID
        `
        var Hello = createReactClass({
          componentDidUpdate: function() {
            this.setState({
              name: 'John'
            });
          }
        });
        `,
        `
        var Hello = createReactClass({
          componentDidUpdate: function() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        });
        `,
        `
        class Hello extends React.Component {
          componentDidUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
        `
        class Hello extends React.Component {
          componentDidUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
        `
        class Hello extends Component {
          componentDidUpdate() {
            this.setState({
              name: 'John'
            });
          }
        }
        `,
        `
        class Hello extends Component {
          componentDidUpdate() {
            foo();
            this.setState({
              name: 'John'
            });
          }
        }
        `,
        // VALID
        `
        var Hello = createReactClass({
          componentDidUpdate: function() {
            if (condition) {
              this.setState({
                name: 'John'
              });
            }
          }
        });
        `,
        `
        class Hello extends React.Component {
          componentDidUpdate() {
            if (condition) {
              this.setState({
                name: 'John'
              });
            }
          }
        }
        `,
        `
        class Hello extends Component {
          componentDidUpdate() {
            if (condition) {
              this.setState({
                name: 'John'
              });
            }
          }
        }
        `,
        `
        var Hello = createReactClass({
          componentDidUpdate: function() {
            foo();
          }
        });
        `,
      ],
      {category: 'lint/noDidUpdateSetState'},
    );
  },
);

