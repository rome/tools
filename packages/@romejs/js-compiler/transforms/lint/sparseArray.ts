import {Path} from '@romejs/js-compiler';
import {AnyNode} from '@romejs/js-ast';

export default {
  name: 'sparseArray',
  enter(path: Path): AnyNode {
    const { node } = path;
    if (node.type === 'ArrayExpression' &&
      node.elements.includes(undefined)) {
        path.context.addNodeDiagnostic(node, {
          category: 'lint/sparseArray',
          message: 'Your array contains an empty slot'
        })
    }
    return node;
  }
}