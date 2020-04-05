import { createFixtureTests } from '@romejs/test';
import { formatJS, BuilderOptions } from '.';
import { parseJS } from '@romejs/js-parser';
import { ConstProgramSyntax } from '@romejs/js-ast';
import { writeFileSync } from '@romejs/fs';

const promise = createFixtureTests((fixture, t) => {
  const {options, files} = fixture; 
  const inputFile = files.get('input.js') || files.get('input.mjs') ||
    files.get('input.ts') || files.get('input.tsx');
  if (inputFile === undefined) {
    throw new Error(
      `The fixture ${fixture.dir} did not have an input.(mjs|js|ts|tsx)`,
    );
  }
  const filename = inputFile.relative;
  const extension = filename.join().slice(filename.join().lastIndexOf('.'));
  const outputFileName = `output${extension}`;

  const outputFile = files.get(outputFileName);

  let outputContent = undefined;
  if (outputFile !== undefined) {
    outputContent = outputFile.content.toString().replace(/\r/g, '');
  }

  const sourceTypeProp = options.get('sourceType');
  const sourceType = sourceTypeProp.asString('script');
  if (sourceType !== 'module' && sourceType !== 'script') {
    throw sourceTypeProp.unexpected('Expected either script or module');
  }
  const syntax: Array<ConstProgramSyntax> = options.get('syntax').asArray(true).map(
    (
      item,
    ) => {
      return item.asStringSet(['jsx', 'ts', 'flow']);
    },
  );
  const allowReturnOutsideFunction = options.get('allowReturnOutsideFunction').asBoolean(
    false,
  );
  
  const format = options.get('format').asStringOrVoid();
  
  const updateSnapshots = t.options.updateSnapshots;

  const inputContent = inputFile.content.toString().replace(/\r/g, '');

  const ast = parseJS({
    input: inputContent,
    sourceType,
    path: filename,
    syntax,
    allowReturnOutsideFunction,
  });
  
  const formatOptions: BuilderOptions = {
    typeAnnotations: true,
    sourceText: inputContent,
    sourceFileName: filename.join(),
    format: format === 'pretty' || format === 'compact' ? format : undefined,
    sourceMaps: false,
  }
  
  const printer = formatJS(ast, formatOptions);
  
  t.addToAdvice({
    type: 'log',
    category: 'info',
    message: 'Fomat options',
  });

  t.addToAdvice({
    type: 'inspect',
    data: {
      ...formatOptions
    },
  });

  if (outputContent !== undefined && !updateSnapshots) {
    t.is(printer.getCode(), outputContent);
  } else {
    writeFileSync(fixture.dir.append(outputFileName), printer.getCode());
  }
});

// @ts-ignore allow top level await
await promise;