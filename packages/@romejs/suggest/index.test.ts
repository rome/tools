import {suggestClosest} from '@romejs/suggest';
import test from '@romejs/test';

test('should suggest test', t => {
  const result = suggestClosest('tst', ['lint', 'ci', 'test', 'bundle']);

  t.is(result, 'test');
});

test('should suggest nothing', t => {
  const result = suggestClosest('not even close', [
    'lint',
    'ci',
    'test',
    'bundle',
  ]);

  t.is(result, undefined);
});
