/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {number0, Number0, coerce0, inc, add} from '@romejs/ob1';
import {escapeMarkup} from '@romejs/string-markup';
import {PartialDiagnosticAdvice} from '@romejs/diagnostics';

type NormalizeNameUnexpected = (opts: {
  message: string;
  start?: Number0;
  end?: Number0;
  advice?: PartialDiagnosticAdvice;
  at?: 'prefix';
}) => void;

type ValidateNamePartOptions = {
  name: string;
  isOrg: boolean;
  isOrgPart: boolean;
  offset: Number0;
};

type NormalizeNameOptions = {
  name: string;
  loose: boolean;
  unexpected: NormalizeNameUnexpected;
};

function validateNamePart(
  {loose, unexpected}: NormalizeNameOptions,
  {name, isOrg, isOrgPart, offset}: ValidateNamePartOptions,
) {
  let normalizedName: string = '';

  for (let i = 0; i < name.length; i++) {
    const char = name[i];

    if (isOrg && char === '@' && i === 0) {
      unexpected({
        message: 'Redundant <emphasis>@</emphasis> in org name',
        start: add(offset, i),
      });
    } else if (!isOrgPart && char === '/') {
      /*unexpected({
        at: 'prefix',
        message: `cannot contain any slashes`,
        start: add(offset, i),
        advice: [
          {
            type: 'log',
            category: 'info',
            message:
              'Did you forget a <emphasis>@</emphasis> prefix to make this a scope?',
          },
        ],
      });*/
      normalizedName = `@${normalizedName}/`;
    } else if (!loose && char.match(/[A-Z]/)) {
      /*unexpected({
          at: 'prefix',
          message: `cannot contain uppercase letters`,
          start: add(offset, i),
        });*/
      normalizedName += char.toLowerCase();
    } else if (char.match(/[A-Za-z0-9\-_\.]/)) {
      normalizedName += char;
    } else {
      unexpected({
        message: `The character <emphasis>${escapeMarkup(
          char,
        )}</emphasis> isn't allowed`,
        start: add(offset, i),
      });
    }
  }

  return normalizedName;
}

export function normalizeName(opts: NormalizeNameOptions): string {
  const {unexpected} = opts;
  let {name} = opts;

  if (name.length > 214) {
    unexpected({
      at: 'prefix',
      message: `cannot exceed 214 characters`,
    });
    name = name.slice(0, 214);
  }

  if (name[0] === '.' || name[0] === '_') {
    unexpected({
      at: 'prefix',
      message: `cannot start with a dot or underscore`,
      start: number0,
    });
    name = name.slice(1);
  }

  if (name[0] === '@') {
    // Validate org and package name separately
    const [org, packageName, ...other] = name.slice(1).split('/');

    // Leading @
    let offset: Number0 = coerce0(1);

    // Org
    const sanitizedOrg = validateNamePart(opts, {
      isOrg: true,
      isOrgPart: true,
      name: org,
      offset,
    });
    offset = add(offset, org.length);

    if (packageName === undefined) {
      unexpected({
        at: 'prefix',
        message: `contains an org but no package name`,
        start: offset,
      });

      name = `@${sanitizedOrg}/unknown`;
    } else {
      // Forward slashSeparator
      offset = inc(offset);

      // Package name
      const sanitizedPackageName = validateNamePart(opts, {
        isOrg: false,
        isOrgPart: true,
        name: packageName,
        offset,
      });
      offset = add(offset, packageName.length);

      // Complain on excess separators
      if (other.length > 0) {
        unexpected({
          at: 'prefix',
          message: `contains too many name separators`,
          start: offset,
        });
      }

      name = `@${sanitizedOrg}/${sanitizedPackageName}`;
    }
  } else {
    name = validateNamePart(opts, {
      name,
      offset: number0,
      isOrg: false,
      isOrgPart: false,
    });
  }

  return name;
}
