/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Context, Path} from '@romejs/js-compiler';
import {
  AnyNode,
  functionHead,
  bindingIdentifier,
  referenceIdentifier,
  staticMemberProperty,
  ReferenceIdentifier,
  AnyStatement,
  FunctionDeclaration,
  ClassDeclaration,
  ClassExpression,
} from '@romejs/js-ast';
import {
  classMethod,
  callExpression,
  thisExpression,
  memberExpression,
  functionDeclaration,
  functionExpression,
  arrowFunctionExpression,
  blockStatement,
  returnStatement,
  classDeclaration,
  identifier,
} from '@romejs/js-ast';
import {template} from '@romejs/js-ast-utils';
import {TransformExitResult} from '@romejs/js-compiler';

function transformClass(
  node: ClassDeclaration | ClassExpression,
  path: Path,
  context: Context,
): {
  constructor: FunctionDeclaration;
  prependDeclarations: Array<AnyStatement>;
  declarations: Array<AnyStatement>;
} {
  const {scope} = path;

  // declarations that we want to append and prepend, these include inheritance setup, method assignment, and other declarations
  const prependDeclarations = [];
  const declarations = [];

  // if the superClass is a global variable or a complex expression, then we should execute it once before the function is evaluated to ensure correct execution semantics
  let superClassRef: undefined | ReferenceIdentifier;
  const {superClass} = node.meta;
  if (superClass !== undefined) {
    if (
      superClass.type === 'ReferenceIdentifier' &&
      scope.hasBinding(superClass.name)
    ) {
      superClassRef = superClass;
    } else {
      superClassRef = referenceIdentifier.create({
        name: scope.generateUid('superClass'),
      });
      prependDeclarations.push(
        template.statement`const ${superClassRef} = ${superClass};`,
      );
    }
  }

  // get the class name, if there's no class id then generate a new name
  const className: string =
    node.id === undefined ? scope.generateUid('class') : node.id.name;

  // push on the superClass setup
  if (superClass !== undefined) {
    if (superClassRef === undefined) {
      throw new Error('Impossible');
    }

    // inherit static properties
    // technically this isn't correct, the fully spec compliant version is Object.setPrototypeOf(Class, SuperClass);
    declarations.push(
      template.statement`Object.assign(${className}, ${superClassRef});`,
    );

    // inherit prototype
    declarations.push(
      template.statement`${className}.prototype = Object.create(${superClassRef} && ${superClassRef}.prototype);`,
    );

    // set correct prototype.constructor
    declarations.push(
      template.statement`${className}.prototype.constructor = ${className};`,
    );

    // some weird property the old babel transform apparently adds, TODO: check the actual usage of this
    declarations.push(
      template.statement`${className}.__superConstructor__ = ${superClassRef};`,
    );
  }

  const newNode = classDeclaration.assert(
    path.reduce({
      name: 'classesSuperTransform',
      enter(path) {
        if (superClassRef === undefined) {
          throw new Error('Impossible');
        }

        const {node} = path;

        // TODO correctly support super() by using return value
        if (node.type === 'CallExpression' && node.callee.type === 'Super') {
          // replace super(...args); with Super.call(this, ...args);
          return callExpression.create({
            callee: memberExpression.create({
              object: superClassRef,
              property: staticMemberProperty.quick(identifier.quick('call')),
            }),
            arguments: [thisExpression.create({}), ...node.arguments],
          });
        }

        // TODO super.foo
        if (node.type === 'MemberExpression' && node.object.type === 'Super') {
          const classMethod2 = path.findAncestry(
            path => path.node.type === 'ClassMethod',
          );
          if (classMethod2 === undefined) {
            throw new Error('Expected to find class method here');
          }
          const isStatic =
            classMethod.assert(classMethod2.node).meta.static === true;

          const {property} = node;

          if (isStatic) {
            return memberExpression.create({
              object: superClassRef,
              property,
            });
          }

          const superProtoRef = memberExpression.create({
            object: superClassRef,
            property: staticMemberProperty.quick(identifier.quick('prototype')),
          });
          return memberExpression.create({
            object: superProtoRef,
            property,
          });
        }

        // super.foo();
        if (
          node.type === 'CallExpression' &&
          node.callee.type === 'MemberExpression' &&
          node.callee.object.type === 'Super'
        ) {
          const classMethod2 = path.findAncestry(
            path => path.node.type === 'ClassMethod',
          );
          if (classMethod2 === undefined) {
            throw new Error('Expected to find class method here');
          }
          const isStatic =
            classMethod.assert(classMethod2.node).meta.static === true;

          const args = node.arguments;
          const {property} = node.callee;

          // for static methods replace `super.foo(...args);` with `Super.foo.call(Class, ...args);`
          if (isStatic) {
            let methodRef;
            methodRef = memberExpression.create({
              object: superClassRef,
              property,
            });
            return callExpression.create({
              callee: memberExpression.create({
                object: methodRef,
                property: staticMemberProperty.quick(identifier.quick('call')),
              }),
              arguments: [referenceIdentifier.quick(className), ...args],
            });
          }

          // for instance methods replace `super.foo(...args)` with `Super.prototype.call(this, ...args)`
          let methodRef;
          let prototypeRef = memberExpression.create({
            object: superClassRef,
            property: staticMemberProperty.quick(identifier.quick('prototype')),
          });
          methodRef = memberExpression.create({
            object: prototypeRef,
            property,
          });
          return callExpression.create({
            callee: memberExpression.create({
              object: methodRef,
              property: staticMemberProperty.quick(identifier.quick('call')),
            }),
            arguments: [thisExpression.create({}), ...args],
          });
        }

        // TODO break when inside of functions

        return node;
      },
    }),
  );

  // setup method declarations
  let constructorMethod = undefined;
  for (const bodyNode of newNode.meta.body) {
    if (bodyNode.type !== 'ClassMethod') {
      context.addNodeDiagnostic(bodyNode, {
        category: 'compile/classes',
        message: "The classes transform doesn't know how to transform this",
      });
      continue;
    }

    // save the constructor if this is it, we'll process this later
    if (bodyNode.kind === 'constructor') {
      constructorMethod = bodyNode;
    }

    // disallow setters/getters, we could trivially support them here with Object.defineProperty if necessary
    if (bodyNode.kind === 'get' || bodyNode.kind === 'set') {
      context.addNodeDiagnostic(bodyNode, {
        category: 'compile/classes',
        message: `${bodyNode.kind}ter methods aren't supported in ES6 classes at FB`,
      });
    }

    if (bodyNode.kind === 'method') {
      // create the function expression to represent this method
      const functionNode = functionExpression.create({
        head: bodyNode.head,
        body: bodyNode.body,
      });

      // create the target node, for static methods this will be the base class, otherwise it's the prototype
      let target;
      if (bodyNode.meta.static === true) {
        target = identifier.quick(className);
      } else {
        target = template.expression`${className}.prototype`;
      }

      // use computed properties for computed methods
      if (bodyNode.key.type === 'ComputedPropertyKey') {
        declarations.push(
          template.statement`${target}[${bodyNode.key.value}] = ${functionNode}`,
        );
      } else {
        declarations.push(
          template.statement`${target}.${bodyNode.key.value} = ${functionNode}`,
        );
      }
    }
  }

  // create the constructor method
  let constructor: FunctionDeclaration;
  if (constructorMethod === undefined) {
    if (superClassRef === undefined) {
      constructor = functionDeclaration.assert(
        template.statement`function ${className}() {}`,
      );
    } else {
      constructor = functionDeclaration.assert(
        template.statement`function ${className}(...args) {${superClassRef}.apply(this, args);}`,
      );
    }
  } else {
    constructor = functionDeclaration.create({
      id: bindingIdentifier.quick(className),
      head: constructorMethod.head,
      body: constructorMethod.body,
    });
  }

  return {constructor, prependDeclarations, declarations};
}

export default {
  name: 'classes',
  enter(path: Path): TransformExitResult {
    const {node, scope, context} = path;

    // correctly replace an export class with the class node then append the declarations
    if (
      (node.type === 'ExportNamedDeclaration' ||
        node.type === 'ExportDefaultDeclaration') &&
      node.declaration !== undefined &&
      node.declaration.type === 'ClassDeclaration'
    ) {
      const {constructor, declarations, prependDeclarations} = transformClass(
        node.declaration,
        path.getChildPath('declaration'),
        context,
      );
      const nodes: Array<AnyNode> = [
        ...prependDeclarations,
        {
          ...node,
          declaration: constructor,
        },
        ...declarations,
      ];
      return nodes;
    }

    if (node.type === 'ClassDeclaration') {
      const {constructor, prependDeclarations, declarations} = transformClass(
        node,
        path,
        context,
      );
      return [...prependDeclarations, constructor, ...declarations];
    }

    // turn a class expression into an IIFE that returns a class declaration
    if (node.type === 'ClassExpression') {
      const className =
        node.id === undefined ? scope.generateUid('class') : node.id.name;

      return callExpression.create({
        callee: arrowFunctionExpression.create({
          head: functionHead.quick([]),
          body: blockStatement.create({
            body: [
              {
                ...node,
                type: 'ClassDeclaration',
                id: bindingIdentifier.quick(className),
              },
              returnStatement.create({
                argument: referenceIdentifier.quick(className),
              }),
            ],
          }),
        }),
        arguments: [],
      });
    }

    return node;
  },
};
