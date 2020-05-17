/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {test} from "rome";
import {testLintMultiple} from "../testHelpers";

test(
	"no this.setState in componentDidUpdate",
	async (t) => {
		await testLintMultiple(
			t,
			[
        // INVALID
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
			],
			{category: "lint/noDidUpdateSetState"},
		);
	},
);
