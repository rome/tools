import {Path} from '@romejs/js-compiler';
import {TransformExitResult} from '@romejs/js-compiler/types';

export default {
  name: 'noAsyncPromiseExecutor',
  enter(path: Path): TransformExitResult {
    const {node, context} = path;

    if (
      node.type === 'NewExpression' &&
      node.callee.type === 'ReferenceIdentifier' &&
      node.callee.name === 'Promise' &&
      node.arguments.length > 0 &&
      (node.arguments[0].type === 'ArrowFunctionExpression' ||
        node.arguments[0].type === 'FunctionExpression') &&
      node.arguments[0].head.async
    ) {
      context.addNodeDiagnostic(node.arguments[0], {
        category: 'lint/noAsyncPromiseExecutor',
        message: 'Promise executor functions should not be async.',
      });
    }

    return node;
  },
};
