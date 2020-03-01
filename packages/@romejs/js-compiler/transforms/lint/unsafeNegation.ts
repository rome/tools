import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';
import {markup} from '@romejs/string-markup';

export default {
  name: 'unsafeNegation',
  enter(path: Path): AnyNode {
    const {node} = path;
    if (
      node.type === 'BinaryExpression' &&
      (node.operator === 'in' || node.operator === 'instanceof') &&
      node.left.type === 'UnaryExpression' &&
      node.left.operator === '!'
    ) {
      path.context.addNodeDiagnostic(node, {
        category: 'lint/unsafeNegation',
        message: markup`Unsafe usage of negation operator in left side of binary expression`,
      });
    }
    return node;
  },
};
