import {test} from 'rome';
import {fun1, fun1Async} from './a';

test(
  'throws sucess',
  (t) => {
    t.throws(() => fun1('hello'));
  },
);

test(
  'throwAsync1 success',
  async (t) => {
    await t.throwsAsync(async () => fun1Async('hello'));
  },
);

test(
  'notThrows2 success',
  (t) => {
    t.notThrows(() => fun1());
  },
);

test(
  'noThrowAsync2 success',
  async (t) => {
    t.notThrowsAsync(async () => fun1Async());
  },
);

test(
  'regex success',
  (t) => {
    t.regex(fun1(), /hello/i);
  },
);

test(
  'notRegex success',
  (t) => {
    t.notRegex(fun1(), /hello/);
  },
);

test(
  'throws fail',
  (t) => {
    t.throws(() => fun1());
  },
);

/*

test('throwsAsync2 fail', async t => {
  await t.throwsAsync(async() => fun1Async());
});

test('notThrows1 fail', t => {
  t.notThrows(() => fun1('hello'));
});

test('noThrowAsync1 fail', async t => {
  await t.notThrowsAsync(async() => await fun1Async('hello'))
});

test('regex fail', t => {
  t.regex(fun1(), /hello/);
})

test('notRegex fail', t => {
  t.notRegex(fun1(), /hello/i);
})
*/
