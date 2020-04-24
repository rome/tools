import {createFixtureTests} from '@romejs/test-helpers';
import {parseJS} from '@romejs/js-parser';
import {ConstProgramSyntax} from '@romejs/js-ast';
import {removeCarriageReturn} from '@romejs/string-utils';
import {BuilderOptions, formatJS} from '.';

const promise = createFixtureTests(
  async (fixture, t) => {
    const {options, files} = fixture;
    // Get the input JS
    const inputFile = files.get('input.js') || files.get('input.mjs') ||
      files.get('input.ts') || files.get('input.tsx');
    if (inputFile === undefined) {
      throw new Error(
        `The fixture ${fixture.dir} did not have an input.(mjs|js|ts|tsx)`,
      );
    }

    const sourceTypeProp = options.get('sourceType');
    const sourceType = sourceTypeProp.asString('script');
    if (sourceType !== 'module' && sourceType !== 'script') {
      throw sourceTypeProp.unexpected();
    }

    const allowReturnOutsideFunction = options.get('allowReturnOutsideFunction').asBoolean(
      false,
    );
    const filename = inputFile.relative;

    const syntax: Array<ConstProgramSyntax> = options.get('syntax').asArray(true).map(
      (
        item,
      ) => {
        return item.asStringSet(['jsx', 'ts', 'flow']);
      },
    );

    const format = options.get('format').asStringOrVoid();

    const inputContent = removeCarriageReturn(inputFile.content.toString());

    const ast = parseJS({
      input: inputContent,
      path: filename,
      allowReturnOutsideFunction,
      sourceType,
      syntax,
    });

    const formatOptions: BuilderOptions = {
      typeAnnotations: true,
      sourceText: inputContent,
      sourceFileName: filename.join(),
      format: format === 'pretty' || format === 'compact' ? format : undefined,
      sourceMaps: false,
    };

    t.addToAdvice({
      type: 'log',
      category: 'info',
      message: 'Fomat options',
    });

    t.addToAdvice({
      type: 'inspect',
      data: {
        ...formatOptions,
      },
    });

    const printer = formatJS(ast, formatOptions);

    const outputFile = inputFile.absolute.getParent().append(
      `${inputFile.absolute.getExtensionlessBasename()}.test.md`,
    ).join();
    await t.snapshotNamed(
      'Javascript Input',
      inputContent,
      undefined,
      outputFile,
    );
    await t.snapshotNamed(
      'Javascript Output',
      printer.getCode(),
      undefined,
      outputFile,
    );
  },
);

// @ts-ignore allow top level await
await promise;
