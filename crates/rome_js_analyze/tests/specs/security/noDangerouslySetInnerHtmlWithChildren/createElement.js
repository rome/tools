import React from "react";

React.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' } }, ['children'])
React.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' } }, 'children')
React.createElement('div', { dangerouslySetInnerHTML: { __html: 'HTML' }, children: 'children' })
