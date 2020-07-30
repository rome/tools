#!/bin/sh

# Copyright (c) Facebook, Inc. and its affiliates.
#
# This source code is licensed under the MIT license found in the
# LICENSE file in the root directory of this source tree.

curl https://raw.githubusercontent.com/spdx/license-list-data/server/json/licenses.json -o src/data.json
