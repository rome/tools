# js/arrays/empty.js
```diff
-const a =
-  someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || [];
-const b =
-  someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || {};
+const a = someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || [];
+const b = someVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeLong.Expression || {};
```
# js/arrays/holes-in-args.js
```diff
-new Test().test().test([, 0]).test();
+new Test()
+  .test()
+  .test([, 0])
+  .test();
```
# js/arrays/issue-10159.js
```diff
 {
   for (const srcPath of [src, `${src}.js`, `${src}/index`, `${src}/index.js`]) {
   }
 }
 {
   for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_43]) {
   }
 }
 {
-  for (const srcPath of [
-    123, 123_123_123, 123_123_123_1, 13_123_3123_31_432,
-  ]) {
+  for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_432]) {
   }
 }
 {
-  for (const srcPath of [
-    123, 123_123_123, 123_123_123_1, 13_123_3123_31_4321,
-  ]) {
+  for (const srcPath of [123, 123_123_123, 123_123_123_1, 13_123_3123_31_4321]) {
   }
 }
```
# js/arrays/nested.js
```diff
 [[]];
 [[], []];
 [[], [], []];
 [[], [0], []];
 [[], [0], [0]];
 [[], [0, 1], [0]];
 [[], [0, 1], [0, 1]];
 [[0]];
 [[0], []];
 [[0], [], []];
 [[0], [0], []];
 [[0], [0], [0]];
 [[0], [0, 1], [0]];
 [[0], [0, 1], [0, 1]];
 [[0, 1]];
 [[0, 1], []];
 [[0, 1], [], []];
 [[0, 1], [0], []];
 [[0, 1], [0], [0]];
 [[0, 1], [0, 1], [0]];
-[
-  [0, 1],
-  [0, 1],
-  [0, 1],
-];
+[[0, 1], [0, 1], [0, 1]];
 [[], [1, 2, 3]];
 [[1], [1]];
-[
-  [1, 2],
-  [1, 2, 3],
-];
-[
-  [1, 0],
-  [1, 0],
-];
+[[1, 2], [1, 2, 3]];
+[[1, 0], [1, 0]];
 [{}];
 [{}, {}];
 [{}, {}, {}];
 [{}, { a }];
 [{}, { a, b }];
 [{}, { a, b, c }];
 [{ a }];
 [{ a }, { a }];
 [{ a }, { a }, { a }];
 [{ a }, { a, b }];
 [{ a }, { a, b, c }];
 [{ a, b }];
 [{ a, b }, { a }];
 [{ a, b }, { a }, { a }];
-[
-  { a, b },
-  { a, b },
-];
-[
-  { a, b },
-  { a, b, c },
-];
+[{ a, b }, { a, b }];
+[{ a, b }, { a, b, c }];
```
# js/arrays/numbers-in-assignment.js
```diff
-bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers = [
-  1, 2, 3, 4, 5,
-];
+bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers =
+  [1, 2, 3, 4, 5];
 
-bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers2 = [
-  66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,
-  76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51, 33,
-  40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17, 91, 27,
-  83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43, 31, 86,
-  27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77, 35, 48,
-  78, 97,
-];
+bifornCringerMoshedPerplex.bifornCringerMoshedPerplexSawder.arrayOfNumbers2 =
+  [
+    66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,
+    76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51,
+    33, 40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17,
+    91, 27, 83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43,
+    31, 86, 27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77,
+    35, 48, 78, 97,
+  ];
```
# js/arrays/numbers-negative-comment-after-minus.js
```diff
 const numbers = [
   -2017,
   -506252,
   -744011292,
   -7224,
   -70.4,
   -83353.6,
   -708.4,
   -174023963.52,
   -40385,
-  -(
-    // comment1
-    380014
-  ),
+  -380014, // comment1
   -253951682,
   -728,
   -15.84,
   -2058467564.56,
   -43,
   -33,
   -85134845,
   -67092,
   -1,
   -78820379,
   -2371.6,
   -16,
   7,
   // comment2
   -62454,
   -4282239912,
   -10816495.36,
   0.88,
   -100622682,
   8.8,
   -67087.68000000001,
   -3758276,
   -25.5211,
   -54,
   -1184265243,
   -46073628,
   -280423.44,
   -41833463,
   -27961.12,
   -305.36,
   -199875.28,
 ];
 
-c = [
-  -(/**/ 66),
-  66,
-  57,
-  45,
-  47,
-  33,
-  53,
-  82,
-  81,
-  76,
-  66,
-  57,
-  45,
-  47,
-  33,
-  53,
-  82,
-  81,
-  223323,
-];
+c =
+  [
+    -66, /**/
+    66,
+    57,
+    45,
+    47,
+    33,
+    53,
+    82,
+    81,
+    76,
+    66,
+    57,
+    45,
+    47,
+    33,
+    53,
+    82,
+    81,
+    223323,
+  ];
```
# js/arrays/numbers-negative.js
```diff
 const numbers1 = [
-  -2017, -506252, -744011292, -7224, -70.4, -83353.6, -708.4, -174023963.52,
+  -2017,
+  -506252,
+  -744011292,
+  -7224,
+  -70.4,
+  -83353.6,
+  -708.4,
+  -174023963.52,
   -40385,
   // comment1
-  -380014, -253951682, -728, -15.84, -2058467564.56, -43, -33, -85134845,
-  -67092, -1, -78820379, -2371.6, -16, 7,
+  -380014,
+  -253951682,
+  -728,
+  -15.84,
+  -2058467564.56,
+  -43,
+  -33,
+  -85134845,
+  -67092,
+  -1,
+  -78820379,
+  -2371.6,
+  -16,
+  7,
   // comment2
-  -62454, -4282239912, -10816495.36, 0.88, -100622682, 8.8, -67087.68000000001,
-  -3758276, -25.5211, -54, -1184265243, -46073628, -280423.44, -41833463,
-  -27961.12, -305.36, -199875.28,
+  -62454,
+  -4282239912,
+  -10816495.36,
+  0.88,
+  -100622682,
+  8.8,
+  -67087.68000000001,
+  -3758276,
+  -25.5211,
+  -54,
+  -1184265243,
+  -46073628,
+  -280423.44,
+  -41833463,
+  -27961.12,
+  -305.36,
+  -199875.28,
 ];
 
 const numbers2 = [
   -234,
   -342, // comment3
   -223,
   -333333.33,
   12345,
 ];
```
# js/arrays/numbers-trailing-comma.js
```diff
 // --------------- print-width -------------------------------------------------
-c = [
-  66, 66, 57, 45, 47, 33, 53, 82, 81, 76, 66, 57, 45, 47, 33, 53, 82, 81,
-  223323,
-];
+c =
+  [
+    66, 66, 57, 45, 47, 33, 53, 82, 81, 76, 66, 57, 45, 47, 33, 53, 82, 81,
+    223323,
+  ];
```
# js/arrays/numbers-with-holes.js
```diff
 const numberWithHoles1 = [
   7234932941,
   7234932722,
   7234932312,
+  // comment before a hole 1
   ,
-  // comment before a hole 1
   7234932841,
   ,
   7234932843,
   ,
   // comment after a hole 1
   7234932436,
 ];
 
 const numberWithHoles2 = [
   0x234932941,
   0x234932722,
   0x234932312,
-
+  // comment before a hole 2
   ,
-  // comment before a hole 2
   0x234932841,
   ,
   0x234932843,
   ,
+
   // comment after a hole 2
   0x234932436,
 ];
```
# js/arrays/numbers-with-tricky-comments.js
```diff
 const lazyCatererNumbers = [
-  1, 2, 4, 7, 11, 16, 22, 29, 37, 46, 56, 67, 79, 92, 106, 121, 137, 154, 172,
-  191, 211, 232, 254, 277, 301, 326, 352, 379, 407, 436, 466 /*block*/,
+  1,
+  2,
+  4,
+  7,
+  11,
+  16,
+  22,
+  29,
+  37,
+  46,
+  56,
+  67,
+  79,
+  92,
+  106,
+  121,
+  137,
+  154,
+  172,
+  191,
+  211,
+  232,
+  254,
+  277,
+  301,
+  326,
+  352,
+  379,
+  407,
+  436,
+  466, /*block*/
   // line
-  497, 529, 562, 596, 631, 667, 704, 742, 781, 821, 862, 904, 947, 991, 1036,
-  1082, 1129, 1177, 1226,
+  497,
+  529,
+  562,
+  596,
+  631,
+  667,
+  704,
+  742,
+  781,
+  821,
+  862,
+  904,
+  947,
+  991,
+  1036,
+  1082,
+  1129,
+  1177,
+  1226,
   // line 2
-  1276, 1327, 1379,
+  1276,
+  1327,
+  1379,
 ];
```
# js/arrays/numbers2.js
```diff
 const userIds1 = [7234932941, 7234932722, 7234932312, 7234932933];
 
 const userIds2 = [
   7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,
   7234932843, 7234932978, 7234932436,
 ];
 
 const userIds3 = [
-  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,
+  7234932941,
+  7234932722,
+  7234932312,
+  7234932933,
+  7234932841,
+  7234932166,
   7234932843,
 
-  7234932978, 7234932436,
+  7234932978,
+  7234932436,
 ];
 
 const userIds4 = [
-  7234932941, 7234932722, 7234932312, 7234932933, 7234932841, 7234932166,
+  7234932941,
+  7234932722,
+  7234932312,
+  7234932933,
+  7234932841,
+  7234932166,
   // comment 1
   7234932843,
 
   7234932978,
 
   // comment 2
   7234932436,
   // comment 3
 ];
```
# js/arrays/numbers3.js
```diff
 let test_case = [
   [
     66, 57, 45, 47, 33, 53, 82, 81, 76, 78, 10, 78, 15, 98, 24, 29, 32, 27, 28,
     76, 41, 65, 84, 35, 97, 90, 75, 24, 88, 45, 23, 75, 63, 86, 24, 39, 9, 51,
     33, 40, 58, 17, 49, 86, 63, 59, 97, 91, 98, 99, 5, 69, 51, 44, 34, 69, 17,
     91, 27, 83, 26, 34, 93, 29, 66, 88, 49, 33, 49, 73, 9, 81, 4, 36, 5, 14, 43,
     31, 86, 27, 39, 75, 98, 99, 55, 19, 39, 21, 85, 86, 46, 82, 11, 44, 48, 77,
     35, 48, 78, 97,
   ],
   [
-    41, 83, 31, 62, 15, 70, 10, 90, /*21,*/ 48, 39, 76, 14, 48, 63, 62, 16, 17,
-    61, 97, 86, 80, 34, 27, 39, 53, 90, 80, 56, 71, 31, 22, 29, 7, 71, 90, 65,
-    17, 48, 85, 14, 94, 16, 32, 4, 96, 49, 97, 53, 87, 54, 2, 78, 37, 21, 3, 97,
-    62, 93, 62, 11, 27, 14, 29, 64, 44, 11, 5, 39, 43, 94, 52, 0, 4, 86, 58, 63,
-    42, 97, 54, 2, 1, 53, 17, 92, 79, 52, 47, 81, 93, 34, 17, 93, 20, 61, 68,
-    58, 49, 27, 45,
+    41,
+    83,
+    31,
+    62,
+    15,
+    70,
+    10,
+    90, /*21,*/
+    48,
+    39,
+    76,
+    14,
+    48,
+    63,
+    62,
+    16,
+    17,
+    61,
+    97,
+    86,
+    80,
+    34,
+    27,
+    39,
+    53,
+    90,
+    80,
+    56,
+    71,
+    31,
+    22,
+    29,
+    7,
+    71,
+    90,
+    65,
+    17,
+    48,
+    85,
+    14,
+    94,
+    16,
+    32,
+    4,
+    96,
+    49,
+    97,
+    53,
+    87,
+    54,
+    2,
+    78,
+    37,
+    21,
+    3,
+    97,
+    62,
+    93,
+    62,
+    11,
+    27,
+    14,
+    29,
+    64,
+    44,
+    11,
+    5,
+    39,
+    43,
+    94,
+    52,
+    0,
+    4,
+    86,
+    58,
+    63,
+    42,
+    97,
+    54,
+    2,
+    1,
+    53,
+    17,
+    92,
+    79,
+    52,
+    47,
+    81,
+    93,
+    34,
+    17,
+    93,
+    20,
+    61,
+    68,
+    58,
+    49,
+    27,
+    45,
   ],
 ];
```
# js/arrays/preserve_empty_lines.js
```diff
-a = [
-  1, 2,
+a =
+  [
+    1,
+    2,
 
-  3,
+    3,
 
-  4,
-];
+    4,
+  ];
```
# js/arrow-call/arrow_call.js
```diff
-const testResults = results.testResults.map((testResult) =>
-  formatResult(testResult, formatter, reporter)
+const testResults = results.testResults.map(
+  (testResult) => formatResult(testResult, formatter, reporter),
 );
 
-it("mocks regexp instances", () => {
-  expect(() =>
-    moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/))
-  ).not.toThrow();
-});
+it(
+  "mocks regexp instances",
+  () => {
+    expect(
+      () => moduleMocker.generateFromMetadata(moduleMocker.getMetadata(/a/)),
+    ).not.toThrow();
+  },
+);
 
 expect(() => asyncRequest({ url: "/test-endpoint" })).toThrowError(
-  /Required parameter/
+  /Required parameter/,
 );
 
-expect(() =>
-  asyncRequest({ url: "/test-endpoint-but-with-a-long-url" })
-).toThrowError(/Required parameter/);
+expect(() => asyncRequest({ url: "/test-endpoint-but-with-a-long-url" })).toThrowError(
+  /Required parameter/,
+);
 
-expect(() =>
-  asyncRequest({ url: "/test-endpoint-but-with-a-suuuuuuuuper-long-url" })
+expect(
+  () => asyncRequest({ url: "/test-endpoint-but-with-a-suuuuuuuuper-long-url" }),
 ).toThrowError(/Required parameter/);
 
-expect(() =>
-  asyncRequest({ type: "foo", url: "/test-endpoint" })
-).not.toThrowError();
+expect(() => asyncRequest({ type: "foo", url: "/test-endpoint" })).not.toThrowError();
 
-expect(() =>
-  asyncRequest({ type: "foo", url: "/test-endpoint-but-with-a-long-url" })
+expect(
+  () => asyncRequest({ type: "foo", url: "/test-endpoint-but-with-a-long-url" }),
 ).not.toThrowError();
 
 const a = Observable.fromPromise(axiosInstance.post("/carts/mine")).map(
-  (response) => response.data
+  (response) => response.data,
 );
 
 const b = Observable.fromPromise(axiosInstance.get(url)).map(
-  (response) => response.data
+  (response) => response.data,
 );
 
 func(
   veryLoooooooooooooooooooooooongName,
   (veryLooooooooooooooooooooooooongName) =>
-    veryLoooooooooooooooongName.something()
+    veryLoooooooooooooooongName.something(),
 );
 
-promise.then((result) =>
-  result.veryLongVariable.veryLongPropertyName > someOtherVariable
-    ? "ok"
-    : "fail"
+promise.then(
+  (result) =>
+    result.veryLongVariable.veryLongPropertyName > someOtherVariable ? "ok" : "fail",
 );
```
# js/arrows-bind/arrows-bind.js
```diff
-(a) => ({}::b()``[""].c++ && 0 ? 0 : 0);
-((a) => b)::c;
-a::((b) => c);
+a => ({}
+::b()``[''].c++ && 0 ? 0 : 0)
+((a) => b);
+::c
+a:
+:(b => c)
```
# js/arrows/newline-before-arrow/newline-before-arrow.js
```diff
-async (x) => x;
+async;
+x;
+=> x
```
# js/assignment-comments/function.js
```diff
-f1 = (
-  //comment
-  a = b
-) => {};
+f1 =
+  (
+    a =
+    //comment
+    b,
+  ) => {};
 
-f2 = (
-  a = b //comment
-) => {};
+f2 =
+  (
+    a = b, //comment
+  ) => {};
 
-f3 = (
-  a = b //comment
-) => {};
+f3 =
+  (
+    a = b, //comment
+  ) => {};
 
 f4 = () => {}; // Comment
 
 f5 =
   // Comment
 
   () => {};
 
 f6 =
   /* comment */
-
   // Comment
 
   () => {};
 
 let f1 = (
+  a =
   //comment
-  a = b
+  b,
 ) => {};
 
 let f2 = (
-  a = b //comment
+  a = b, //comment
 ) => {};
 
 let f3 = (
-  a = b //comment
+  a = b, //comment
 ) => {};
 
 let f4 = () => {}; // Comment
 
 let f5 =
-  // Comment
+// Comment
 
-  () => {};
-
-let f6 =
-  /* comment */
+() => {};
 
-  // Comment
+let f6 = /* comment */
+// Comment
 
-  () => {};
+() => {};
```
# js/assignment-comments/identifier.js
```diff
 const kochabCooieGameOnOboleUnweave = annularCooeedSplicesWalksWayWay; // ???
 
-const bifornCringerMoshedPerplexSawder = // !!!
-  glimseGlyphsHazardNoopsTieTie +
-  averredBathersBoxroomBuggyNurl -
-  anodyneCondosMalateOverateRetinol;
+const bifornCringerMoshedPerplexSawder = glimseGlyphsHazardNoopsTieTie + averredBathersBoxroomBuggyNurl - anodyneCondosMalateOverateRetinol; // !!!
```
# js/assignment-comments/number.js
```diff
 fnNumber =
   // Comment
   3;
 
 fnNumber =
   // Comment
 
   3;
 
 fnNumber =
   // Comment0
   // Comment1
   3;
 
 fnNumber = /* comment */ 3;
 
 fnNumber =
   /* comments0 */
   /* comments1 */
   3;
 
 fnNumber =
   // Comment
   3;
 
 var fnNumber =
-  // Comment
+// Comment
 
-  3;
+3;
 
 var fnNumber =
-  // Comment0
-  // Comment1
-  3;
+// Comment0
+// Comment1
+3;
 
 var fnNumber = /* comment */ 3;
 
-var fnNumber =
-  /* comments0 */
-  /* comments1 */
-  3;
+var fnNumber = /* comments0 */
+/* comments1 */
+3;
```
# js/assignment-comments/string.js
```diff
 fnString =
   // Comment
   "some" + "long" + "string";
 
 fnString =
   // Comment
 
   "some" + "long" + "string";
 
 fnString =
   // Comment
 
   "some" + "long" + "string";
 
 fnString =
   /* comment */
   "some" + "long" + "string";
 
 fnString =
   /**
    * multi-line
    */
   "some" + "long" + "string";
 
 fnString =
   /* inline */ "some" +
-  "long" +
-  "string" +
-  "some" +
-  "long" +
-  "string" +
-  "some" +
-  "long" +
-  "string" +
-  "some" +
-  "long" +
-  "string";
+    "long" +
+    "string" +
+    "some" +
+    "long" +
+    "string" +
+    "some" +
+    "long" +
+    "string" +
+    "some" +
+    "long" +
+    "string";
 
-fnString = // Comment0
+fnString =
+  // Comment0
   // Comment1
   "some" + "long" + "string";
 
 fnString = "some" + "long" + "string"; // Comment
 
 fnString =
   // Comment
   "some" + "long" + "string";
 
 var fnString =
   // Comment
 
   "some" + "long" + "string";
 
 var fnString =
   // Comment
 
   "some" + "long" + "string";
 
 var fnString =
   /* comment */
   "some" + "long" + "string";
 
 var fnString =
   /**
    * multi-line
    */
   "some" + "long" + "string";
 
 var fnString =
   /* inline */ "some" +
   "long" +
   "string" +
   "some" +
   "long" +
   "string" +
   "some" +
   "long" +
   "string" +
   "some" +
   "long" +
   "string";
 
-var fnString = // Comment0
+var fnString =
+  // Comment0
   // Comment1
   "some" + "long" + "string";
 
-var fnString = "some" + "long" + "string"; // Comment
+var fnString =
+  // Comment
+  "some" + "long" + "string";
```
# js/assignment/binaryish.js
```diff
 const computedDescriptionLines =
   (showConfirm && descriptionLinesConfirming) ||
   (focused && !loading && descriptionLinesFocused) ||
   descriptionLines;
 
 const computedDescriptionLines2 =
-  (showConfirm && // comment
-    descriptionLinesConfirming) || // comment
+  (
+    showConfirm && // comment
+    descriptionLinesConfirming
+  ) || // comment
   (focused && !loading && descriptionLinesFocused) || // comment
   descriptionLines; // comment
 
 computedDescriptionLines =
   (focused && !loading && descriptionLinesFocused) || descriptionLines;
```
# js/assignment/call-with-template.js
```diff
-const result = template(`
+const result = template(
+  `
   if (SOME_VAR === "") {}
-`)({
-  SOME_VAR: value,
-});
+`,
+)({ SOME_VAR: value });
 
-const output = template(`function f() %%A%%`)({
-  A: t.blockStatement([]),
-});
+const output = template(`function f() %%A%%`)({ A: t.blockStatement([]) });
```
# js/assignment/chain-two-segments.js
```diff
-tt.parenR.updateContext = tt.braceR.updateContext = function () {
-  if (this.state.context.length === 1) {
-    return;
-  }
-};
+tt.parenR.updateContext =
+  tt.braceR.updateContext =
+    function () {
+      if (this.state.context.length === 1) {
+        return;
+      }
+    };
```
# js/assignment/chain.js
```diff
-let bifornCringerMoshedPerplexSawder =
-  (askTrovenaBeenaDependsRowans =
+let bifornCringerMoshedPerplexSawder = askTrovenaBeenaDependsRowans =
   glimseGlyphsHazardNoopsTieTie =
-  averredBathersBoxroomBuggyNurl =
-  anodyneCondosMalateOverateRetinol =
-  annularCooeedSplicesWalksWayWay =
-    kochabCooieGameOnOboleUnweave);
+    averredBathersBoxroomBuggyNurl =
+      anodyneCondosMalateOverateRetinol =
+        annularCooeedSplicesWalksWayWay = kochabCooieGameOnOboleUnweave;
 
 bifornCringerMoshedPerplexSawder =
   askTrovenaBeenaDependsRowans =
-  glimseGlyphsHazardNoopsTieTie =
-  x =
-  averredBathersBoxroomBuggyNurl =
-  anodyneCondosMal(
-    sdsadsa,
-    dasdas,
-    asd(() => sdf)
-  ).ateOverateRetinol =
-  annularCooeedSplicesWalksWayWay =
-    kochabCooieGameOnOboleUnweave;
+    glimseGlyphsHazardNoopsTieTie =
+      x =
+        averredBathersBoxroomBuggyNurl =
+          anodyneCondosMal(sdsadsa, dasdas, asd(() => sdf)).ateOverateRetinol =
+            annularCooeedSplicesWalksWayWay = kochabCooieGameOnOboleUnweave;
 
 bifornCringerMoshedPerplexSawder =
   askTrovenaBeenaDependsRowans =
-  glimseGlyphsHazardNoopsTieTie =
-  x =
-  averredBathersBoxroomBuggyNurl =
-  anodyneCondosMal(
-    sdsadsa,
-    dasdas,
-    asd(() => sdf)
-  ).ateOverateRetinol =
-  annularCooeedSplicesWalksWayWay =
-    kochabCooieGameOnOboleUnweave + kochabCooieGameOnOboleUnweave;
+    glimseGlyphsHazardNoopsTieTie =
+      x =
+        averredBathersBoxroomBuggyNurl =
+          anodyneCondosMal(sdsadsa, dasdas, asd(() => sdf)).ateOverateRetinol =
+            annularCooeedSplicesWalksWayWay =
+              kochabCooieGameOnOboleUnweave + kochabCooieGameOnOboleUnweave;
 
 a = b = c;
```
# js/assignment/destructuring-array.js
```diff
-const [width = nextWidth, height = nextHeight, baseline = nextBaseline] =
-  measureText(nextText, getFontString(element));
+const [width = nextWidth, height = nextHeight, baseline = nextBaseline] = measureText(
+  nextText,
+  getFontString(element),
+);
```
# js/assignment/destructuring-heuristic.js
```diff
 {
   {
-    const {
-      id,
-      static: isStatic,
-      method: isMethod,
-      methodId,
-      getId,
-      setId,
-    } = privateNamesMap.get(name);
+    const { id, static: isStatic, method: isMethod, methodId, getId, setId } = privateNamesMap.get(
+      name,
+    );
 
     const { id1, method: isMethod1, methodId1 } = privateNamesMap.get(name);
 
-    const {
-      id2,
-      method: isMethod2,
-      methodId2,
-    } = privateNamesMap.get(bifornCringerMoshedPerplexSawder);
+    const { id2, method: isMethod2, methodId2 } = privateNamesMap.get(
+      bifornCringerMoshedPerplexSawder,
+    );
 
-    const {
-      id3,
-      method: isMethod3,
-      methodId3,
-    } = anodyneCondosMalateOverateRetinol.get(bifornCringerMoshedPerplexSawder);
+    const { id3, method: isMethod3, methodId3 } = anodyneCondosMalateOverateRetinol.get(
+      bifornCringerMoshedPerplexSawder,
+    );
   }
 }
```
# js/assignment/issue-10218.js
```diff
-const _id1 =
-  data.createTestMessageWithAReallyLongName.someVeryLongProperty
-    .thisIsAlsoALongProperty._id;
+const _id1 = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty._id;
 
-const { _id2 } =
-  data.createTestMessageWithAReallyLongName.someVeryLongProperty
-    .thisIsAlsoALongProperty;
+const { _id2 } = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty;
 
-const { _id: id3 } =
-  data.createTestMessageWithAReallyLongName.someVeryLongProperty
-    .thisIsAlsoALongProperty;
+const { _id: id3 } = data.createTestMessageWithAReallyLongName.someVeryLongProperty.thisIsAlsoALongProperty;
```
# js/assignment/issue-1419.js
```diff
 someReallyLongThingStoredInAMapWithAReallyBigName[pageletID] =
-  _someVariableThatWeAreCheckingForFalsiness
-    ? Date.now() - _someVariableThatWeAreCheckingForFalsiness
-    : 0;
+  _someVariableThatWeAreCheckingForFalsiness ? Date.now() - _someVariableThatWeAreCheckingForFalsiness : 0;
```
# js/assignment/issue-1966.js
```diff
-const aVeryLongNameThatGoesOnAndOn =
-  this.someOtherObject.someOtherNestedObject.someLongFunctionName();
+const aVeryLongNameThatGoesOnAndOn = this.someOtherObject.someOtherNestedObject.someLongFunctionName();
 
 this.someObject.someOtherNestedObject =
   this.someOtherObject.whyNotNestAnotherOne.someLongFunctionName();
 
 this.isaverylongmethodexpression.withmultiplelevels =
   this.isanotherverylongexpression.thatisalsoassigned = 0;
```
# js/assignment/issue-2184.js
```diff
 const areaPercentageDiff = (
-  topRankedZoneFit.areaPercentageRemaining -
-  previousZoneFitNow.areaPercentageRemaining
+  topRankedZoneFit.areaPercentageRemaining - previousZoneFitNow.areaPercentageRemaining
 ).toFixed(2);
```
# js/assignment/issue-2482-2.js
```diff
 class foo {
   bar() {
-    const median =
-      dates.length % 2
-        ? dates[half].getTime()
-        : (dates[half - 1].getTime() + dates[half].getTime()) / 2.0;
+    const median = dates.length % 2 ? dates[half].getTime() : (
+      dates[half - 1].getTime() + dates[half].getTime()
+    ) / 2.0;
   }
 }
```
# js/assignment/issue-2540.js
```diff
-manifestCache[templateId] = readFileSync(
-  `${MANIFESTS_PATH}/${templateId}.json`,
-  { encoding: "utf-8" }
-);
+manifestCache[templateId] =
+  readFileSync(`${MANIFESTS_PATH}/${templateId}.json`, { encoding: "utf-8" });
```
# js/assignment/issue-3819.js
```diff
 this.dummy.type1.dummyPropertyFunction =
   this.dummy.type2.dummyPropertyFunction =
-  this.dummy.type3.dummyPropertyFunction =
-  this.dummy.type4.dummyPropertyFunction =
-  this.dummy.type5.dummyPropertyFunction =
-  this.dummy.type6.dummyPropertyFunction =
-  this.dummy.type7.dummyPropertyFunction =
-  this.dummy.type8.dummyPropertyFunction =
-    () => {
-      return "dummy";
-    };
+    this.dummy.type3.dummyPropertyFunction =
+      this.dummy.type4.dummyPropertyFunction =
+        this.dummy.type5.dummyPropertyFunction =
+          this.dummy.type6.dummyPropertyFunction =
+            this.dummy.type7.dummyPropertyFunction =
+              this.dummy.type8.dummyPropertyFunction =
+                () => {
+                  return "dummy";
+                };
```
# js/assignment/issue-4094.js
```diff
 if (something) {
-  const otherBrandsWithThisAdjacencyCount123 = Object.values(
-    edge.to.edges
-  ).length;
+  const otherBrandsWithThisAdjacencyCount123 = Object.values(edge.to.edges).length;
 }
```
# js/assignment/issue-5610.js
```diff
 // Function call wrapping is not optimal for readability:
 // Function names tend to get pushed to the right, whereas arguments end up on the left,
 // creating a wide gap that the eyes have to cross in order to read the call.
 const { qfwvfkwjdqgz, bctsyljqucgz, xuodxhmgwwpw } = qbhtcuzxwedz(
   yrwimwkjeeiu,
   njwvozigdkfi,
-  alvvjgkmnmhd
+  alvvjgkmnmhd,
 );
```
# js/assignment/issue-6922.js
```diff
 async function f() {
   const { data, status } = await request.delete(
     `/account/${accountId}/documents/${type}/${documentNumber}`,
-    { validateStatus: () => true }
+    { validateStatus: () => true },
   );
   return { data, status };
 }
 
-const data1 = request.delete("----------------------------------------------", {
-  validateStatus: () => true,
-});
+const data1 = request.delete(
+  "----------------------------------------------",
+  { validateStatus: () => true },
+);
 
 const data2 = request.delete(
   "----------------------------------------------x",
-  { validateStatus: () => true }
+  { validateStatus: () => true },
 );
 
 const data3 = request.delete(
   "----------------------------------------------xx",
-  { validateStatus: () => true }
+  { validateStatus: () => true },
 );
 
 const data4 = request.delete(
   "----------------------------------------------xxx",
-  { validateStatus: () => true }
+  { validateStatus: () => true },
 );
```
# js/assignment/issue-7091.js
```diff
-const { imStore, showChat, customerServiceAccount } =
-  store[config.reduxStoreName];
+const { imStore, showChat, customerServiceAccount } = store[
+  config.reduxStoreName
+];
```
# js/assignment/issue-7572.js
```diff
 const t = {
-  hello: world(),
-  "this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line":
-    orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig(),
+  "hello": world(),
+  "this-is-a-very-long-key-and-the-assignment-should-be-put-on-the-next-line": orMaybeIAmMisunderstandingAndIHaveSetSomethingWrongInMyConfig(),
   "can-someone-explain": this(),
 };
```
# js/assignment/issue-8218.js
```diff
-const pendingIndicators =
-  shield.alarmGeneratorConfiguration.getPendingVersionColumnValues;
+const pendingIndicators = shield.alarmGeneratorConfiguration.getPendingVersionColumnValues;
 
-const pendingIndicatorz =
-  shield.alarmGeneratorConfiguration.getPendingVersionColumnValues();
+const pendingIndicatorz = shield.alarmGeneratorConfiguration.getPendingVersionColumnValues();
```
# js/assignment/lone-arg.js
```diff
-let vgChannel = pointPositionDefaultRef({
-  model,
-  defaultPos,
-  channel,
-})();
+let vgChannel = pointPositionDefaultRef({ model, defaultPos, channel })();
 
 let vgChannel2 = pointPositionDefaultRef({ model, defaultPos, channel })();
 
-const bifornCringerMoshedPerplexSawderGlyphsHa =
-  someBigFunctionName("foo")("bar");
+const bifornCringerMoshedPerplexSawderGlyphsHa = someBigFunctionName("foo")(
+  "bar",
+);
 
 if (true) {
-  node.id = this.flowParseTypeAnnotatableIdentifier(
-    /*allowPrimitiveOverride*/ true
-  );
+  node.id =
+    this.flowParseTypeAnnotatableIdentifier( /*allowPrimitiveOverride*/ true);
 }
 
-const bifornCringerMoshedPerplexSawderGlyphsHb = someBigFunctionName(`foo
-`)("bar");
+const bifornCringerMoshedPerplexSawderGlyphsHb = someBigFunctionName(
+  `foo
+`,
+)("bar");
```
# js/assignment/sequence.js
```diff
-for (i = 0, len = arr.length; i < len; i++) {
+for ((i = 0), (len = arr.length); i < len; i++) {
   console.log(arr[i]);
 }
 
 for (i = 0, len = arr.length; i < len; i++) {
   console.log(arr[i]);
 }
```
# js/assignment/unary.js
```diff
-const loooooooooooooooooooooooooong1 =
-  void looooooooooooooong.looooooooooooooong.loooooong;
-const loooooooooooooooooooooooooong2 =
-  void "looooooooooooooooooooooooooooooooooooooooooog";
-const loooooooooooooooooooooooooong3 =
-  !looooooooooooooong.looooooooooooooong.loooooong;
-const loooooooooooooooooooooooooong4 =
-  !"looooooooooooooooooooooooooooooooooooooooooog";
-const loooooooooooooooooooooooooong5 =
-  void void looooooooooooooong.looooooooooooooong.loooooong;
-const loooooooooooooooooooooooooong6 =
-  void void "looooooooooooooooooooooooooooooooooooooooooog";
-const loooooooooooooooooooooooooong7 =
-  !!looooooooooooooong.looooooooooooooong.loooooong;
-const loooooooooooooooooooooooooong8 =
-  !!"looooooooooooooooooooooooooooooooooooooooooog";
+const loooooooooooooooooooooooooong1 = void looooooooooooooong.looooooooooooooong.loooooong;
+const loooooooooooooooooooooooooong2 = void "looooooooooooooooooooooooooooooooooooooooooog";
+const loooooooooooooooooooooooooong3 = !looooooooooooooong.looooooooooooooong.loooooong;
+const loooooooooooooooooooooooooong4 = !"looooooooooooooooooooooooooooooooooooooooooog";
+const loooooooooooooooooooooooooong5 = void void looooooooooooooong.looooooooooooooong.loooooong;
+const loooooooooooooooooooooooooong6 = void void "looooooooooooooooooooooooooooooooooooooooooog";
+const loooooooooooooooooooooooooong7 = !!looooooooooooooong.looooooooooooooong.loooooong;
+const loooooooooooooooooooooooooong8 = !!"looooooooooooooooooooooooooooooooooooooooooog";
```
# js/async-do-expressions/async-do-expressions.js
```diff
-(async do {
+async;
+do {
   1;
-});
+};
 
-(async do {});
+(async
+do {});
 
-let x = async do {
-  if (foo()) {
-    f();
-  } else if (bar()) {
-    g();
-  } else {
-    h();
-  }
+let x = async;
+do {
+  if (foo()) { f() }
+  else if (bar()) { g() }
+  else { h() }
 };
 
-(async do {
-  await 42;
-});
+async;
+do {
+  await 42
+}
 
 function iter() {
   return async do {
     return 1;
-  };
-}
+  }
+};
 
-let x = async do {
+let x = async;
+do {
   let tmp = f();
-  tmp * tmp + 1;
+  tmp * tmp + 1
 };
```
# js/async/async-shorthand-method.js
```diff
-({
-  async get() {},
-  async set() {},
-});
+({ async get() {}, async set() {} });
```
# js/async/await-parse.js
```diff
 async function f1() {
   (await f()).length;
 }
 async function g() {
   invariant((await driver.navigator.getUrl()).substr(-7));
 }
 function* f2() {
   !(yield a);
 }
 async function f3() {
-  a = !(await f());
+  a = !await f();
 }
 async () => {
   new A(await x);
   obj[await x];
 };
```
# js/async/conditional-expression.js
```diff
 async function f() {
   const result = typeof fn === "function" ? await fn() : null;
 }
 
 (async function () {
   console.log(await (true ? Promise.resolve("A") : Promise.resolve("B")));
 })();
 
 async function f2() {
   await (spellcheck && spellcheck.setChecking(false));
-  (await spellcheck) && spellcheck.setChecking(false);
+  await spellcheck && spellcheck.setChecking(false);
 }
```
# js/async/inline-await.js
```diff
 async function f() {
   const admins = (
-    await db
-      .select("*")
-      .from("admins")
-      .leftJoin("bla")
-      .where("id", "in", [1, 2, 3, 4])
+    await (
+      db
+        .select("*")
+        .from("admins")
+        .leftJoin("bla")
+        .where("id", "in", [1, 2, 3, 4])
+    )
   ).map(({ id, name }) => ({ id, name }));
 }
```
# js/async/nested.js
```diff
 const getAccountCount = async () =>
   (
     await (
-      await (
-        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)
-      ).findItem("My bookmarks")
+      await (await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)).findItem(
+        "My bookmarks",
+      )
     ).getChildren()
   ).length;
```
# js/babel-plugins/async-do-expressions.js
```diff
-(async do {
-  await requestAPI().json();
-});
+async;
+do { await requestAPI().json() };
```
# js/babel-plugins/bigint.js
```diff
 // https://github.com/tc39/proposal-bigint
 
 const previousMaxSafe = BigInt(Number.MAX_SAFE_INTEGER);
 // ↪ 9007199254740991
 
 const maxPlusOne = previousMaxSafe + 1n;
 // ↪ 9007199254740992n
 
 const theFuture = previousMaxSafe + 2n;
 // ↪ 9007199254740993n, this works now!
 
 const multi = previousMaxSafe * 2n;
 // ↪ 18014398509481982n
 
 // `–` is not minus sign,
 // SIC https://github.com/tc39/proposal-bigint#operators
 // const subtr = multi – 10n;
 // ↪ 18014398509481972n
 
 const mod = multi % 10n;
 // ↪ 2n
 
 const bigN = 2n ** 54n;
 // ↪ 18014398509481984n
 
 bigN * -1n;
 // ↪ –18014398509481984n
 
 0n === 0;
 // ↪ false
 
 0n == 0;
 // ↪ true
 
 1n < 2;
 // ↪ true
 
 2n > 1;
 // ↪ true
 
 2 > 2;
 // ↪ false
 
 2n > 2;
 // ↪ false
 
 2n >= 2;
 // ↪ true
 
 const mixed = [4n, 6, -12n, 10, 4, 0, 0n];
 // ↪  [4n, 6, -12n, 10, 4, 0, 0n]
 
 mixed.sort();
 // ↪ [-12n, 0, 0n, 10, 4n, 4, 6]
 
 if (0n) {
   console.log("Hello from the if!");
 } else {
   console.log("Hello from the else!");
 }
 
 // ↪ "Hello from the else!"
 
 0n || 12n;
 // ↪ 12n
 
 0n && 12n;
 // ↪ 0n
 
 Boolean(0n);
 // ↪ false
 
 Boolean(12n);
 // ↪ true
 
 !12n;
 // ↪ false
 
 !0n;
 // ↪ true
 
 const view = new BigInt64Array(4);
 // ↪ [0n, 0n, 0n, 0n]
 view.length;
 // ↪ 4
 view[0];
 // ↪ 0n
 view[0] = 42n;
 view[0];
 // ↪ 42n
 
 // Highest possible BigInt value that can be represented as a
 // signed 64-bit integer.
 const max = 2n ** (64n - 1n) - 1n;
 view[0] = max;
 view[0];
 // ↪ 9_223_372_036_854_775_807n
 view[0] = max + 1n;
 view[0];
 // ↪ -9_223_372_036_854_775_808n
 //   ^ negative because of overflow
 
 1n + 2;
 // ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions
 
-1n * 2 +
-  // ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions
+(1n * 2)
+// ↪ TypeError: Cannot mix BigInt and other types, use explicit conversions
 
-  1n;
++ 1n;
 // ↪ TypeError: Cannot convert a BigInt value to a number
 
 Number(1n);
 // ↪ 1
 
 1n + "2";
 // ↪ "12"
 
 "2" + 1n;
 // ↪ "21"
 
 const badPrecision = BigInt(9007199254740993);
 // ↪9007199254740992n
 
 const goodPrecision = BigInt("9007199254740993");
 // ↪9007199254740993n
 
 const alsoGoodPrecision = 9007199254740993n;
 // ↪9007199254740993n
```
# js/babel-plugins/decimal.js
```diff
 // https://github.com/babel/babel/pull/11640
 
-100m;
-9223372036854775807m;
-0m;
-3.1415926535897932m;
-100.0m;
-0.1m;
-({ 0m: 0, 0.1m() {}, get 0.2m() {}, set 3m(_) {}, async 4m() {}, *0.5m() {} });
-1m;
-100m;
-9223372036854775807m;
-100m;
+100m
+9223372036854775807m
+0.m
+3.1415926535897932m
+100.000m
+.1m
+({ 0m: 0, .1m() {}, get
+0.2m()
+{
+}
+, set 3m(_)
+{
+}
+, async 4m()
+{
+}
+, *.5m()
+{
+}
+})
+1.m
+100m
+9223372036854775807m
+100.m
 
 // Invalid decimal
-2e9m;
-016432m;
-089m;
+2e9m
+016432m
+089m
 
 // https://github.com/tc39/proposal-decimal
-0.1m + 0.2m === 0.3m;
-2.0m;
+.1m + .2m === .3m
+2.00m
 -0m;
 typeof 1m === "bigdecimal";
 typeof 1m === "decimal128";
```
# js/babel-plugins/do-expressions.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-do-expressions
 
-let a = do {
-  if (x > 10) {
-    ("big");
+let a =
+do {
+  if(x > 10) {
+    'big';
   } else {
-    ("small");
+    'small';
   }
 };
 // is equivalent to:
 let a = x > 10 ? "big" : "small";
```
# js/babel-plugins/export-default-from.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-export-default-from
 
-export v from "mod";
+
+export
+v;
+from;
+("mod");
```
# js/babel-plugins/function-bind.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-function-bind
 
-obj::func;
+obj:
+:func
 // is equivalent to:
-func.bind(obj)::obj.func;
+func.bind(obj)
+
+::obj.func
 // is equivalent to:
-obj.func.bind(obj);
+obj.func.bind(obj)
 
-obj::func(val);
+obj::func(val)
 // is equivalent to:
-func
-  .call(obj, val)
+func.call(obj, val)
 
-  ::obj.func(val);
+::obj.func(val)
 // is equivalent to:
-obj.func.call(obj, val);
+obj.func.call(obj, val)
```
# js/babel-plugins/jsx.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-syntax-jsx
 
-var profile = (
-  <div>
-    <img src="avatar.png" className="profile" />
-    <h3>{[user.firstName, user.lastName].join(" ")}</h3>
-  </div>
-);
+var profile = <div>
+  <img src="avatar.png" className="profile" />
+  <h3>{[user.firstName, user.lastName].join(' ')}</h3>
+</div>;
```
# js/babel-plugins/module-blocks.js
```diff
-let m = module {
+let m = module;
+{
   export let m = 2;
   export let n = 3;
-};
+}
```
# js/babel-plugins/optional-chaining.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-optional-chaining
 
-const obj = {
-  foo: {
-    bar: {
-      baz: 42,
-    },
-  },
-};
+const obj = { foo: { bar: { baz: 42 } } };
 
 const baz = obj?.foo?.bar?.baz; // 42
 
 const safe = obj?.qux?.baz; // undefined
 
 // Optional chaining and normal chaining can be intermixed
 obj?.foo.bar?.baz; // Only access `foo` if `obj` exists, and `baz` if
 // `bar` exists
 
 // Example usage with bracket notation:
 obj?.["foo"]?.bar?.baz; // 42
 
 const obj2 = {
   foo: {
     bar: {
       baz() {
         return 42;
       },
     },
   },
 };
 
 const baz2 = obj?.foo?.bar?.baz(); // 42
 
 const safe3 = obj?.qux?.baz(); // undefined
 const safe4 = obj?.foo.bar.qux?.(); // undefined
 
 const willThrow = obj?.foo.bar.qux(); // Error: not a function
 
 // Top function can be called directly, too.
 function test() {
   return 42;
 }
 test?.(); // 42
 
 exists?.(); // undefined
 
-const obj3 = {
-  foo: {
-    bar: {
-      baz: class {},
-    },
-  },
-};
+const obj3 = { foo: { bar: { baz: class {} } } };
 
-const obj4 = {
-  foo: {
-    bar: {},
-  },
-};
+const obj4 = { foo: { bar: {} } };
 
 const ret = delete obj?.foo?.bar?.baz; // true
```
# js/babel-plugins/partial-application.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-partial-application
 
 function add(x, y) {
   return x + y;
 }
 
 const addOne = add(1, ?); // apply from the left
 addOne(2); // 3
 
 const addTen = add(?, 10); // apply from the right
 addTen(2); // 12
 
-let newScore = player.score |> add(7, ?) |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.
+let newScore = player.score
+  |> add(7, ?)
+  |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.
 
 f(x, ?); // partial application from left
 f(?, x); // partial application from right
 f(?, x, ?); // partial application for any arg
 o.f(x, ?); // partial application from left
 o.f(?, x); // partial application from right
 o.f(?, x, ?); // partial application for any arg
 super.f(?); // partial application allowed for call on |SuperProperty|
```
# js/babel-plugins/pipeline-operator-fsharp.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator
 // https://github.com/valtech-nyc/proposal-fsharp-pipelines
 
 promise
   |> await
-  |> (x) => doubleSay(x, ", ")
+  |> x
+=> doubleSay(x, ', ')
   |> capitalize
-  |> (x) => x + "!"
-  |> (x) => new User.Message(x)
-  |> (x) => stream.write(x)
+  |> x => x + '!'
+  |> x => new User.Message(x)
+  |> x => stream.write(x)
   |> await
-  |> console.log;
+  |> console.log
 
 const result = exclaim(capitalize(doubleSay("hello")));
 result; //=> "Hello, hello!"
 
-const result = "hello" |> doubleSay |> capitalize |> exclaim;
+const result = "hello"
+  |> doubleSay
+  |> capitalize
+  |> exclaim;
 
 result; //=> "Hello, hello!"
 
 const person = { score: 25 };
 
-const newScore =
-  person.score |> double |> (n) => add(7, n) |> (n) => boundScore(0, 100, n);
+const newScore = person.score
+  |> double
+  |> n
+=> add(7, n)
+  |> n => boundScore(0, 100, n)
 
 newScore; //=> 57
 
 // As opposed to:
 let newScore = boundScore(0, 100, add(7, double(person.score)));
```
# js/babel-plugins/pipeline-operator-hack.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator
 // https://github.com/js-choi/proposal-hack-pipes
 
-return list |> take(prefix.length, %) |> equals(%, prefix);
-
+return list
+ |> take(prefix.length, %)
+ |> equals(%, prefix);
 // (The % token isn't final; it might instead be @ or ? or #.)
```
# js/babel-plugins/pipeline-operator-minimal.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-pipeline-operator
 // https://github.com/tc39/proposal-pipeline-operator/
 
 let result = exclaim(capitalize(doubleSay("hello")));
 result; //=> "Hello, hello!"
 
-let result = "hello" |> doubleSay |> capitalize |> exclaim;
+let result = "hello"
+  |> doubleSay
+  |> capitalize
+  |> exclaim;
 
 result; //=> "Hello, hello!"
```
# js/babel-plugins/private-fields-in-in.js
```diff
 // https://github.com/tc39/proposal-private-fields-in-in
 
 class C {
   #brand;
 
   static isC(obj) {
     try {
       obj.#brand;
       return true;
     } catch {
       return false;
     }
   }
 }
 
 class C2 {
   #data = null; // populated later
 
   get #getter() {
     if (!this.#data) {
       throw new Error("no data yet!");
     }
     return this.#data;
   }
 
   static isC(obj) {
     try {
       obj.#getter;
       return true;
     } catch {
       return false; // oops! might have gotten here because `#getter` threw :-(
     }
   }
 }
 
 class C3 {
   #brand;
 
   #method() {}
 
   get #getter() {}
 
   static isC(obj) {
-    return #brand in obj && #method in obj && #getter in obj;
+    return (#brand in obj) && (#method in obj) && (#getter in obj);
   }
 }
-
 // Invalid https://github.com/tc39/proposal-private-fields-in-in#try-statement
 // class C {
 //   #brand;
 
 //   static isC(obj) {
 //     return try obj.#brand;
 //   }
 // }
```
# js/babel-plugins/record-tuple-record.js
```diff
-const record1 = #{
-  a: 1,
-  b: 2,
-  c: 3,
-};
+const record1 = #
+{
+  a: 1, b;
+  : 2,
+    c: 3,
+}
 
-const record2 = #{ ...record1, b: 5 };
+const record2 = #
+{
+  ...record1, b: 5
+}
```
# js/babel-plugins/record-tuple-tuple.js
```diff
-const tuple1 = #[1, 2, 3];
+const tuple1 = #
+[1, 2, 3];
```
# js/babel-plugins/throw-expressions.js
```diff
 // https://babeljs.io/docs/en/babel-plugin-proposal-throw-expressions
 
-function test(param = throw new Error("required!")) {
-  const test = param === true || throw new Error("Falsy!");
+function test(param = throw new Error('required!')
+)
+{
+  const test = param === true ||
+  throw new Error("Falsy!");
 }
```
# js/babel-plugins/v8intrinsic.js
```diff
 // https://github.com/babel/babel/pull/10148
 
-%DebugPrint(foo);
+%DebugPrint(foo)
 
 // Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-bind-expression/options.json
 // ::%DebugPrint(null)
 
 // Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/in-member-expression/options.json
 // a.%DebugPrint();
 
 // Invalid code https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json
 // const i = %DebugPrint;
 // i(foo);
 
 // https://github.com/JLHwung/babel/blob/c1a3cbfd65e08b7013fd6f8c62add8cb10b4b169/packages/babel-parser/test/fixtures/v8intrinsic/_errors/not-in-call-expression/options.json
 // %DebugPrint?.(null)
 
 new %DebugPrint(null);
 
 function* foo() {
-  yield %StringParseInt("42", 10);
+  yield;
+  %StringParseInt("42", 10)
 }
 
 foo % bar();
```
# js/binary-expressions/arrow.js
```diff
 function f() {
   const appEntities = getAppEntities(loadObject).filter(
     (entity) =>
       entity &&
       entity.isInstallAvailable() &&
       !entity.isQueue() &&
-      entity.isDisabled()
+      entity.isDisabled(),
   );
 }
 
 function f2() {
   const appEntities = getAppEntities(loadObject).map(
     (entity) =>
       entity &&
       entity.isInstallAvailable() &&
       !entity.isQueue() &&
-      entity.isDisabled() && {
-        id: entity.id,
-      }
+      entity.isDisabled() &&
+      { id: entity.id },
   );
 }
 
 ((x) => x) + "";
 "" + ((x) => x);
```
# js/binary-expressions/call.js
```diff
 (
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
-  bbbbbbbbbbbbbbbbbbbbbbbbb &&
-  ccccccccccccccccccccccccc &&
-  ddddddddddddddddddddddddd &&
-  eeeeeeeeeeeeeeeeeeeeeeeee
+    bbbbbbbbbbbbbbbbbbbbbbbbb &&
+    ccccccccccccccccccccccccc &&
+    ddddddddddddddddddddddddd &&
+    eeeeeeeeeeeeeeeeeeeeeeeee
 )();
 
 (aa && bb && cc && dd && ee)();
 
 (
   aaaaaaaaaaaaaaaaaaaaaaaaa +
-  bbbbbbbbbbbbbbbbbbbbbbbbb +
-  ccccccccccccccccccccccccc +
-  ddddddddddddddddddddddddd +
-  eeeeeeeeeeeeeeeeeeeeeeeee
+    bbbbbbbbbbbbbbbbbbbbbbbbb +
+    ccccccccccccccccccccccccc +
+    ddddddddddddddddddddddddd +
+    eeeeeeeeeeeeeeeeeeeeeeeee
 )();
 
 (aa + bb + cc + dd + ee)();
 
 (
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
-  bbbbbbbbbbbbbbbbbbbbbbbbb &&
-  ccccccccccccccccccccccccc &&
-  ddddddddddddddddddddddddd &&
-  eeeeeeeeeeeeeeeeeeeeeeeee
+    bbbbbbbbbbbbbbbbbbbbbbbbb &&
+    ccccccccccccccccccccccccc &&
+    ddddddddddddddddddddddddd &&
+    eeeeeeeeeeeeeeeeeeeeeeeee
 )()()();
 
 (
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
-  bbbbbbbbbbbbbbbbbbbbbbbbb &&
-  ccccccccccccccccccccccccc &&
-  ddddddddddddddddddddddddd &&
-  eeeeeeeeeeeeeeeeeeeeeeeee
+    bbbbbbbbbbbbbbbbbbbbbbbbb &&
+    ccccccccccccccccccccccccc &&
+    ddddddddddddddddddddddddd &&
+    eeeeeeeeeeeeeeeeeeeeeeeee
 )(
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
     bbbbbbbbbbbbbbbbbbbbbbbbb &&
     ccccccccccccccccccccccccc &&
     ddddddddddddddddddddddddd &&
-    eeeeeeeeeeeeeeeeeeeeeeeee
+    eeeeeeeeeeeeeeeeeeeeeeeee,
 )(
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
     bbbbbbbbbbbbbbbbbbbbbbbbb &&
     ccccccccccccccccccccccccc &&
     ddddddddddddddddddddddddd &&
-    eeeeeeeeeeeeeeeeeeeeeeeee
+    eeeeeeeeeeeeeeeeeeeeeeeee,
 )(
   aaaaaaaaaaaaaaaaaaaaaaaaa &&
     bbbbbbbbbbbbbbbbbbbbbbbbb &&
     ccccccccccccccccccccccccc &&
     ddddddddddddddddddddddddd &&
-    eeeeeeeeeeeeeeeeeeeeeeeee
+    eeeeeeeeeeeeeeeeeeeeeeeee,
 );
```
# js/binary-expressions/comment.js
```diff
 a =
-  // Comment 1
-  Math.random() * (yRange * (1 - minVerticalFraction)) +
-  minVerticalFraction * yRange -
-  offset;
+  (
+    // Comment 1
+    (Math.random() * (yRange * (1 - minVerticalFraction))) + (
+      minVerticalFraction * yRange
+    )
+  ) - offset;
 
 a +
   a +
   a + // comment
   a +
   a;
 
 a &&
   longLongLongLongLongLongLongLongLong &&
   longLongLongLongLongLongLongLongLong && // comment
   longLongLongLongLongLongLongLongLong &&
   longLongLongLongLongLongLongLongLong;
 
 a ||
   longLongLongLongLongLongLongLongLong ||
   longLongLongLongLongLongLongLongLong || // comment
   longLongLongLongLongLongLongLongLong ||
   longLongLongLongLongLongLongLongLong;
 
 var a = x(
   abifornCringerMoshedPerplexSawder +
     kochabCooieGameOnOboleUnweave + // f
     glimseGlyphsHazardNoopsTieTie +
-    bifornCringerMoshedPerplexSawder
+    bifornCringerMoshedPerplexSawder,
 );
 
 foo[
   a +
     a + // comment
     a +
     bar[
       b +
         b +
         b + // comment
         b +
         b
     ]
 ];
 
 !(
   a +
-  a + // comment
-  a +
-  !(
-    b +
-    b +
-    b + // comment
-    b +
-    b
-  )
+    a + // comment
+    a +
+    !(
+      b +
+        b +
+        b + // comment
+        b +
+        b
+    )
 );
```
# js/binary-expressions/equality.js
```diff
-(x == y) == z;
-(x != y) == z;
-(x == y) != z;
-(x != y) != z;
+x == y == z;
+x != y == z;
+x == y != z;
+x != y != z;
 
-(x === y) === z;
-(x !== y) === z;
-(x === y) !== z;
-(x !== y) !== z;
+x === y === z;
+x !== y === z;
+x === y !== z;
+x !== y !== z;
```
# js/binary-expressions/exp.js
```diff
-a ** (b ** c);
+a ** b ** c;
 (a ** b) ** c;
 a.b ** c;
 (-a) ** b;
 a ** -b;
 -(a ** b);
 (a * b) ** c;
 a ** (b * c);
 (a % b) ** c;
```
# js/binary-expressions/if.js
```diff
 if (this.hasPlugin("dynamicImports") && this.lookahead().type) {
 }
 
-if (
-  this.hasPlugin("dynamicImports") &&
-  this.lookahead().type === tt.parenLeft
-) {
+if (this.hasPlugin("dynamicImports") && this.lookahead().type === tt.parenLeft) {
 }
 
 if (
-  this.hasPlugin("dynamicImports") &&
-  this.lookahead().type === tt.parenLeft.right
+  this.hasPlugin("dynamicImports") && this.lookahead().type === tt.parenLeft.right
 ) {
 }
 
 if (
   VeryVeryVeryVeryVeryVeryVeryVeryLong === VeryVeryVeryVeryVeryVeryVeryVeryLong
 ) {
 }
```
# js/binary-expressions/inline-jsx.js
```diff
-const user = renderedUser || (
-  <div>
-    <User name={this.state.user.name} age={this.state.user.age} />
-  </div>
-);
+const user = renderedUser || <div><User name={this.state.user.name} age={this.state.user.age} /></div>;
 
-const user2 =
-  renderedUser ||
-  (shouldRenderUser && (
-    <div>
-      <User name={this.state.user.name} age={this.state.user.age} />
-    </div>
-  ));
+const user2 = renderedUser || (
+  shouldRenderUser && <div><User name={this.state.user.name} age={this.state.user.age} /></div>
+);
 
 const avatar = hasAvatar && <Gravatar user={author} size={size} />;
 
-const avatar2 = (hasAvatar || showPlaceholder) && (
-  <Gravatar user={author} size={size} />
-);
+const avatar2 = (hasAvatar || showPlaceholder) && <Gravatar
+  user={author}
+  size={size}
+/>;
```
# js/binary-expressions/inline-object-array.js
```diff
-prevState = prevState || {
-  catalogs: [],
-  loadState: LOADED,
-  opened: false,
-  searchQuery: "",
-  selectedCatalog: null,
-};
-
-prevState = prevState ||
-  defaultState || {
+prevState =
+  prevState || {
     catalogs: [],
     loadState: LOADED,
     opened: false,
     searchQuery: "",
     selectedCatalog: null,
   };
 
 prevState =
   prevState ||
-  (defaultState && {
-    catalogs: [],
-    loadState: LOADED,
-    opened: false,
-    searchQuery: "",
-    selectedCatalog: null,
-  });
+    defaultState ||
+    {
+      catalogs: [],
+      loadState: LOADED,
+      opened: false,
+      searchQuery: "",
+      selectedCatalog: null,
+    };
+
+prevState =
+  prevState || (
+    defaultState && {
+      catalogs: [],
+      loadState: LOADED,
+      opened: false,
+      searchQuery: "",
+      selectedCatalog: null,
+    }
+  );
 
-prevState = prevState ||
-  (useDefault && defaultState) || {
-    catalogs: [],
-    loadState: LOADED,
-    opened: false,
-    searchQuery: "",
-    selectedCatalog: null,
-  };
+prevState =
+  prevState ||
+    (useDefault && defaultState) ||
+    {
+      catalogs: [],
+      loadState: LOADED,
+      opened: false,
+      searchQuery: "",
+      selectedCatalog: null,
+    };
 
-this.steps = steps || [
-  {
-    name: "mock-module",
-    path: "/nux/mock-module",
-  },
-];
+this.steps = steps || [{ name: "mock-module", path: "/nux/mock-module" }];
 
 this.steps =
-  steps ||
-  (checkStep && [
-    {
-      name: "mock-module",
-      path: "/nux/mock-module",
-    },
-  ]);
+  steps || (checkStep && [{ name: "mock-module", path: "/nux/mock-module" }]);
 
-this.steps = (steps && checkStep) || [
-  {
-    name: "mock-module",
-    path: "/nux/mock-module",
-  },
-];
+this.steps =
+  (steps && checkStep) || [{ name: "mock-module", path: "/nux/mock-module" }];
 
 const create = () => {
   const result = doSomething();
   return (
     shouldReturn &&
-    result.ok && {
-      status: "ok",
-      createdAt: result.createdAt,
-      updatedAt: result.updatedAt,
-    }
+      result.ok &&
+      { status: "ok", createdAt: result.createdAt, updatedAt: result.updatedAt }
   );
 };
 
 const create2 = () => {
   const result = doSomething();
   return (
     (shouldReturn && result.ok && result) || {
       status: "ok",
       createdAt: result.createdAt,
       updatedAt: result.updatedAt,
     }
   );
 };
 
 const obj = {
-  state: shouldHaveState &&
-    stateIsOK && {
-      loadState: LOADED,
-      opened: false,
-    },
-  loadNext: (stateIsOK && hasNext) || {
-    skipNext: true,
-  },
+  state: shouldHaveState && stateIsOK && { loadState: LOADED, opened: false },
+  loadNext: (stateIsOK && hasNext) || { skipNext: true },
   loaded: true,
 };
```
# js/binary-expressions/jsx_parent.js
```diff
 <div
   src={
     !isJellyfishEnabled &&
-    diffUpdateMessageInput != null &&
-    this.state.isUpdateMessageEmpty
+      diffUpdateMessageInput != null &&
+      this.state.isUpdateMessageEmpty
   }
 />;
 
 <div>
   {!isJellyfishEnabled &&
     diffUpdateMessageInput != null &&
     this.state.isUpdateMessageEmpty}
 </div>;
 
 <div
   style={
     !isJellyfishEnabled &&
-    diffUpdateMessageInput && {
-      fontSize: 14,
-      color: "#fff",
-    }
+      diffUpdateMessageInput &&
+      { fontSize: 14, color: "#fff" }
   }
 />;
 
 <div>
-  {!isJellyfishEnabled && diffUpdateMessageInput != null && (
-    <div>
-      <span>Text</span>
-    </div>
-  )}
+  {!isJellyfishEnabled &&
+    diffUpdateMessageInput != null && <div><span>Text</span></div>}
 </div>;
 
 <div>
-  {(!isJellyfishEnabled && diffUpdateMessageInput != null && child) || (
-    <div>
-      <span>Text</span>
-    </div>
-  )}
+  {!isJellyfishEnabled &&
+    diffUpdateMessageInput != null && child || <div><span>Text</span></div>}
 </div>;
```
# js/binary-expressions/math.js
```diff
-x + y / z;
-x / y + z;
+x + (y / z);
+(x / y) + z;
 
-(x * y) % z;
-(x / y) % z;
-(x % y) * z;
-(x % y) / z;
+x * y % z;
+x / y % z;
+x % y * z;
+x % y / z;
 
-(x % y) % z;
+x % y % z;
 
-(x << y) >> z;
-(x >>> y) << z;
-(x >>> y) >>> z;
-(x + y) >> z;
+x << y >> z;
+x >>> y << z;
+x >>> y >>> z;
+x + y >> z;
 
-x | (y & z);
-(x & y) | z;
+x | y & z;
+x & y | z;
 x ^ y ^ z;
 x & y & z;
 x | y | z;
-x & (y >> z);
-(x << y) | z;
+x & y >> z;
+x << y | z;
```
# js/binary-expressions/return.js
```diff
 function foo() {
-  return (
-    this.hasPlugin("dynamicImports") &&
-    this.lookahead().type === tt.parenLeft.right
-  );
+  return this.hasPlugin("dynamicImports") && this.lookahead().type === tt.parenLeft.right;
 }
 
 function foo2() {
-  return this.hasPlugin("dynamicImports") &&
-    this.lookahead().type === tt.parenLeft.right
-    ? true
-    : false;
+  return this.hasPlugin("dynamicImports") && this.lookahead().type === tt.parenLeft.right ? true : false;
 }
 
 function foo3() {
-  return this.calculate().compute().first.numberOfThings >
-    this.calculate().compute().last.numberOfThings
-    ? true
-    : false;
+  return this.calculate().compute().first.numberOfThings > this.calculate().compute().last.numberOfThings ? true : false;
 }
```
# js/binary-expressions/short-right.js
```diff
-this._cumulativeHeights &&
-  Math.abs(
-    this._cachedItemHeight(this._firstVisibleIndex + i) -
-      this._provider.fastHeight(i + this._firstVisibleIndex)
-  ) > 1;
+this._cumulativeHeights && Math.abs(
+  this._cachedItemHeight(this._firstVisibleIndex + i) - this._provider.fastHeight(
+    i + this._firstVisibleIndex,
+  ),
+) > 1;
 
-foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo(
-  aaaaaaaaaaaaaaaaaaa
-) + a;
+foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo(aaaaaaaaaaaaaaaaaaa) + a;
 
-const isPartOfPackageJSON =
-  dependenciesArray.indexOf(dependencyWithOutRelativePath.split("/")[0]) !== -1;
+const isPartOfPackageJSON = dependenciesArray.indexOf(
+  dependencyWithOutRelativePath.split("/")[0],
+) !== -1;
 
-defaultContent.filter((defaultLocale) => {
-  // ...
-})[0] || null;
+defaultContent.filter(
+  (defaultLocale) => {
+    // ...
+  },
+)[0] || null;
```
# js/binary-expressions/test.js
```diff
 // It should always break the highest precedence operators first, and
 // break them all at the same time.
 
 const x = longVariable + longVariable + longVariable;
-const x1 =
-  longVariable +
-  longVariable +
-  longVariable +
-  longVariable -
-  longVariable +
-  longVariable;
-const x2 =
-  longVariable +
-  longVariable * longVariable +
-  longVariable -
-  longVariable +
-  longVariable;
+const x1 = longVariable + longVariable + longVariable + longVariable - longVariable + longVariable;
+const x2 = longVariable + (longVariable * longVariable) + longVariable - longVariable + longVariable;
 const x3 =
   longVariable +
-  (longVariable * longVariable * longVariable) / longVariable +
+  (longVariable * longVariable * longVariable / longVariable) +
   longVariable;
 
 const x4 =
   longVariable &&
   longVariable &&
   longVariable &&
   longVariable &&
   longVariable &&
   longVariable;
 const x5 =
   (longVariable && longVariable) ||
   (longVariable && longVariable) ||
   (longVariable && longVariable);
 const x6 =
   firstItemWithAVeryLongNameThatKeepsGoing ||
   firstItemWithAVeryLongNameThatKeepsGoing ||
   {};
 const x7 =
   firstItemWithAVeryLongNameThatKeepsGoing ||
   firstItemWithAVeryLongNameThatKeepsGoing ||
   [];
-const x8 =
-  call(
-    firstItemWithAVeryLongNameThatKeepsGoing,
-    firstItemWithAVeryLongNameThatKeepsGoing
-  ) || [];
+const x8 = call(
+  firstItemWithAVeryLongNameThatKeepsGoing,
+  firstItemWithAVeryLongNameThatKeepsGoing,
+) || [];
 
 const x9 =
   longVariable * longint && longVariable >> 0 && longVariable + longVariable;
 
-const x10 =
-  longVariable > longint && longVariable === 0 + longVariable * longVariable;
+const x10 = longVariable > longint && longVariable === (
+  0 + (longVariable * longVariable)
+);
 
 foo(
-  obj.property * new Class() && obj instanceof Class && longVariable
-    ? number + 5
-    : false
+  obj.property * new Class() && (obj instanceof Class) && longVariable ? number + 5 : false,
 );
```
# js/binary-expressions/unary.js
```diff
 const anyTestFailures = !(
-  aggregatedResults.numFailedTests === 0 &&
-  aggregatedResults.numRuntimeErrorTestSuites === 0
+  aggregatedResults.numFailedTests === 0 && aggregatedResults.numRuntimeErrorTestSuites === 0
 );
```
# js/binary_math/parens.js
```diff
 const result = (a + b) >>> 1;
 var sizeIndex = ((index - 1) >>> level) & MASK;
 var from = offset > left ? 0 : (left - offset) >> level;
 var to = ((right - offset) >> level) + 1;
-if (rawIndex < 1 << (list._level + SHIFT)) {
+if (rawIndex < (1 << (list._level + SHIFT))) {
 }
-var res = size < SIZE ? 0 : ((size - 1) >>> SHIFT) << SHIFT;
-sign = 1 - 2 * (b[3] >> 7);
+var res = size < SIZE ? 0 : (((size - 1) >>> SHIFT) << SHIFT);
+sign = 1 - (2 * (b[3] >> 7));
 exponent = (((b[3] << 1) & 0xff) | (b[2] >> 7)) - 127;
 mantissa = ((b[2] & 0x7f) << 16) | (b[1] << 8) | b[0];
 
-((2 / 3) * 10) / 2 + 2;
-const rotateX =
-  ((RANGE / rect.height) * refY - RANGE / 2) * getXMultiplication(rect.width);
-const rotateY =
-  ((RANGE / rect.width) * refX - RANGE / 2) * getYMultiplication(rect.width);
+(2 / 3 * 10 / 2) + 2;
+const rotateX = (((RANGE / rect.height) * refY) - (RANGE / 2)) * getXMultiplication(
+  rect.width,
+);
+const rotateY = (((RANGE / rect.width) * refX) - (RANGE / 2)) * getYMultiplication(
+  rect.width,
+);
 
 (a % 10) - 5;
-(a * b) % 10;
-a % 10 > 5;
-a % 10 == 0;
+a * b % 10;
+(a % 10) > 5;
+(a % 10) == 0;
```
# js/bind-expressions/await.js
```diff
 const doBothThings = async () => {
   const request = doAsyncThing();
-  return (await request)::doSyncThing();
+  return (await request);
+  ::doSyncThing()
 };
```
# js/bind-expressions/bind_parens.js
```diff
-(a || b)::c;
-a || b::c;
-::obj.prop;
-(void 0)::func();
-(+0)::is(-0);
-a::b.c;
-a::(b.c());
-a::b.c();
-a::(b.c()());
-a::(b.c()());
-a::(b.c())();
-a::(b.c().d);
-a::(c().d.e);
-a::(b());
-a::(b::c());
-a::(b()::c);
-a::(b().c::d);
-a::(b.c::d);
-a::(b::c.d);
-a::(b.c::d::e);
-a::(b::c::d);
-a::(b::c::d.e);
-a::(b::c::d).e;
-a::(void 0);
-a::(b.c()::d.e);
-a::(b.c::d.e);
-a::(b.c::d.e)::f.g;
-b.c::d.e;
-(b.c::d).e;
-(b::c::d).e;
-new (a::b)();
+(a || b);
+::c
+a || (b
+::c)
+::obj.prop
+(void 0);
+::func()
+(+0);
+::is(-0)
+a:
+:(b.c)
+a:
+:(b.c())
+a:
+:b.c()
+a:
+:(b.c()())
+a:
+:((b.c())())
+a:
+:(b.c())()
+a:
+:(b.c().d)
+a:
+:(c().d.e)
+a:
+:(b())
+a:
+:(b::c())
+a:
+:(b()::c)
+a:
+:(b().c::d)
+a:
+:(b.c::d)
+a:
+:(b::c.d)
+a:
+:(b.c::d::e)
+a:
+:(b::c::d)
+a:
+:(b::c::d.e)
+a:
+:((b::c::d).e)
+a:
+:(void 0)
+a:
+:(b.c()::d.e)
+a:
+:(b.c::d.e)
+a:
+:(b.c::d.e)::f.g
+b.c;
+::d.e
+(b.c
+::d).e
+(b::c::d)
+.e
+new (a
+::b)()
 new f(a::b);
-f[a::b];
-f[a::b()];
+f[a
+::b]
+f[a
+::b()]
```
# js/bind-expressions/long_name_method.js
```diff
 class X {
   constructor() {
     this.testLongNameMethodAndSomethingElseLallala =
-      ::this.testLongNameMethodAndSomethingElseLallala;
+    ::this.testLongNameMethodAndSomethingElseLallala
   }
 
   testLongNameMethodAndSomethingElseLallala() {
     return true;
   }
 }
```
# js/bind-expressions/method_chain.js
```diff
 import { interval } from "rxjs/observable/interval";
 import { filter } from "rxjs/operator/filter";
 import { take } from "rxjs/operator/take";
 import { map } from "rxjs/operator/map";
 import { throttle } from "rxjs/operator/throttle";
 import { takeUntil } from "rxjs/operator/takeUntil";
 
 function test(observable) {
-  return observable
-    ::filter((data) => data.someTest)
-    ::throttle(() =>
-      interval(10)
-        ::take(1)
-        ::takeUntil(observable::filter((data) => someOtherTest))
-    )
-    ::map(someFunction);
+  return observable;
+  ::filter(data => data.someTest)
+        ::throttle(() =>
+            interval(10)
+                ::take(1)
+                ::takeUntil(observable::filter(data => someOtherTest))
+        )
+        ::map(someFunction)
 }
```
# js/bind-expressions/short_name_method.js
```diff
 class X {
   constructor() {
-    this.shortMethod = ::this.shortMethod;
+    this.shortMethod =
+    ::this.shortMethod
   }
 
   shortMethod() {
     return true;
   }
 }
```
# js/bind-expressions/unary.js
```diff
-!x::y;
-!(x::y /* foo */);
-!(/* foo */ x::y);
+!x;
+::y
+!(x
+::y /* foo */)
+!(/* foo */ x
+::y)
 !(
   /* foo */
-  x::y
-);
+  x
+::y
+)
 !(
-  x::y
+  x
+::y
   /* foo */
-);
+)
 !(
-  x::y // foo
-);
+  x
+::y // foo
+)
```
# js/break-calls/break.js
```diff
 h(
   f(
     g(() => {
       a;
-    })
-  )
+    }),
+  ),
 );
 
 deepCopyAndAsyncMapLeavesA(
   { source: sourceValue, destination: destination[sourceKey] },
-  { valueMapper, overwriteExistingKeys }
+  { valueMapper, overwriteExistingKeys },
 );
 
 deepCopyAndAsyncMapLeavesB(
   1337,
   { source: sourceValue, destination: destination[sourceKey] },
-  { valueMapper, overwriteExistingKeys }
+  { valueMapper, overwriteExistingKeys },
 );
 
 deepCopyAndAsyncMapLeavesC(
   { source: sourceValue, destination: destination[sourceKey] },
   1337,
-  { valueMapper, overwriteExistingKeys }
+  { valueMapper, overwriteExistingKeys },
 );
 
 function someFunction(url) {
   return get(url).then(
     (json) => dispatch(success(json)),
-    (error) => dispatch(failed(error))
+    (error) => dispatch(failed(error)),
   );
 }
 
 const mapChargeItems = fp.flow(
-  (l) => (l < 10 ? l : 1),
-  (l) => Immutable.Range(l).toMap()
+  (l) => l < 10 ? l : 1,
+  (l) => Immutable.Range(l).toMap(),
 );
 
-expect(
-  new LongLongLongLongLongRange([0, 0], [0, 0])
-).toEqualAtomLongLongLongLongRange(new LongLongLongRange([0, 0], [0, 0]));
+expect(new LongLongLongLongLongRange([0, 0], [0, 0])).toEqualAtomLongLongLongLongRange(
+  new LongLongLongRange([0, 0], [0, 0]),
+);
 
 ["red", "white", "blue", "black", "hotpink", "rebeccapurple"].reduce(
   (allColors, color) => {
     return allColors.concat(color);
   },
-  []
+  [],
 );
```
# js/break-calls/parent.js
```diff
 runtimeAgent.getProperties(
   objectId,
   false, // ownProperties
   false, // accessorPropertiesOnly
   false, // generatePreview
   (error, properties, internalProperties) => {
     return 1;
-  }
+  },
 );
```
# js/break-calls/react.js
```diff
 function helloWorld() {
-  useEffect(() => {
-    // do something
-  }, [props.value]);
-  useEffect(() => {
-    // do something
-  }, [
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-  ]);
+  useEffect(
+    () => {
+      // do something
+    },
+    [props.value],
+  );
+  useEffect(
+    () => {
+      // do something
+    },
+    [
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+    ],
+  );
 }
 
 function helloWorldWithReact() {
-  React.useEffect(() => {
-    // do something
-  }, [props.value]);
-  React.useEffect(() => {
-    // do something
-  }, [
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-    props.value,
-  ]);
+  React.useEffect(
+    () => {
+      // do something
+    },
+    [props.value],
+  );
+  React.useEffect(
+    () => {
+      // do something
+    },
+    [
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+      props.value,
+    ],
+  );
 }
 
 function MyComponent(props) {
   useEffect(
     () => {
       console.log("some code", props.foo);
     },
-
     // We need to disable the eslint warning here,
     // because of some complicated reason.
     // eslint-disable line react-hooks/exhaustive-deps
-    []
+    [],
   );
 
   return null;
 }
 
 function Comp1() {
   const { firstName, lastName } = useMemo(
     () => parseFullName(fullName),
-    [fullName]
+    [fullName],
   );
 }
 
 function Comp2() {
   const { firstName, lastName } = useMemo(
     () => func(),
     [
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
       props.value,
-    ]
+    ],
   );
 }
 
 function Comp3() {
   const { firstName, lastName } = useMemo(
     (aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk) =>
       func(aaa, bbb, ccc, ddd, eee, fff, ggg, hhh, iii, jjj, kkk),
-    [foo, bar, baz]
+    [foo, bar, baz],
   );
 }
 
 function Comp4() {
   const { firstName, lastName } = useMemo(
     () =>
       (foo && bar && baz) ||
       baz ||
       (foo && baz(foo) + bar(foo) + foo && bar && baz) ||
       baz ||
       (foo && baz(foo) + bar(foo)),
-    [foo, bar, baz]
+    [foo, bar, baz],
   );
 }
 
 function Comp5() {
   const { firstName, lastName } = useMemo(() => func(), [foo]);
 }
```
# js/break-calls/reduce.js
```diff
 const [first1] = array.reduce(
   () => [accumulator, element, accumulator, element],
-  [fullName]
+  [fullName],
 );
 
 const [first2] = array.reduce(
   (accumulator, element) => [accumulator, element],
-  [fullName]
+  [fullName],
 );
```
# js/call/no-argument/special-cases.js
```diff
-require(/* comment */);
-new require(/* comment */);
-define(/* comment */);
-new define(/* comment */);
-it(/* comment */);
-new it(/* comment */);
+require( /* comment */ );
+new require( /* comment */ );
+define( /* comment */ );
+new define( /* comment */ );
+it( /* comment */ );
+new it( /* comment */ );
```
# js/class-comment/class-property.js
```diff
 class X {
   TEMPLATE =
-    // tab index is needed so we can focus, which is needed for keyboard events
-    '<div class="ag-large-text" tabindex="0">' +
+  // tab index is needed so we can focus, which is needed for keyboard events
+  '<div class="ag-large-text" tabindex="0">' +
     '<div class="ag-large-textarea"></div>' +
     "</div>";
 }
```
# js/class-comment/misc.js
```diff
 class x {
-  focus() { // comment 1
+  focus() {
+    // comment 1
     // comment 2
   }
 }
```
# js/class-comment/superclass.js
```diff
 class A // comment 1
-  // comment 2
-  extends B {}
+// comment 2
+extends B {}
 
-class A1 extends B {
-  // comment1
-  // comment2
-  // comment3
-}
+class A1 extends B // comment1
+// comment2
+// comment3
+{}
 
 class A2 /* a */ extends B {}
 class A3 extends B /* a */ {}
-class A4 /* a */ extends B {}
+class A4 extends /* a */ B {}
 
-(class A5 // comment 1
+(
+  class A5 // comment 1
   // comment 2
-  extends B {});
+  extends B {}
+);
 
-(class A6 extends B {
-  // comment1
+(
+  class A6 extends B // comment1
   // comment2
   // comment3
-});
+  {}
+);
 
 (class A7 /* a */ extends B {});
 (class A8 extends B /* a */ {});
-(class A9 /* a */ extends B {});
+(class A9 extends /* a */ B {});
 
 class a extends b {
   // comment
   constructor() {}
 }
 
-class c extends d {
-  // comment2
+class c extends d
+// comment2
+{
   constructor() {}
 }
 
-class C2 // comment
-  extends Base
-{
+class C2 extends Base {
+  // comment
   foo() {}
 }
```
# js/class-extends/complex.js
```diff
 class loooooooooooooooooooong1 extends foooooooo(
-  foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo()))))))
+  foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo(foooooooo())))))),
 ) {}
 
 class loooooooooooooooooooong2 extends function (make, model, year, owner) {
   this.make = make;
   this.model = model;
   this.year = year;
   this.owner = owner;
 } {}
 
 class loooooooooooooooooooong3 extends class {
   cconstructor(make, model, year, owner) {
     this.make = make;
     this.model = model;
     this.year = year;
     this.owner = owner;
   }
 } {}
```
# js/class-extends/extends.js
```diff
 // "ArrowFunctionExpression"
 class a1 extends (() => {}) {}
 
 // "AssignmentExpression"
 class a2 extends (b = c) {}
 
 // "AwaitExpression"
 async function f() {
   class a extends (await b) {}
 }
 
 // "BinaryExpression"
 class a3 extends (b + c) {}
 
 // "CallExpression"
 class a4 extends b() {}
 
 // "ClassExpression"
 class a5 extends class {} {}
 
 // "ConditionalExpression"
 class a6 extends (b ? c : d) {}
 
 // "FunctionExpression"
-class a7 extends function () {} {}
+class a7 extends (function () {}) {}
 
 // "LogicalExpression"
 class a8 extends (b || c) {}
 
 // "MemberExpression"
 class a9 extends b.c {}
 
 // "NewExpression"
 class a10 extends (new B()) {}
 
 // "ObjectExpression"
 class a11 extends ({}) {}
 
 // "SequenceExpression"
 class a12 extends (b, c) {}
 
 // "TaggedTemplateExpression"
 class a13 extends `` {}
 
 // "UnaryExpression"
 class a14 extends (void b) {}
 
 // "UpdateExpression"
 class a15 extends (++b) {}
 
 // "YieldExpression"
 function* f2() {
   // Flow has a bug parsing it.
   // class a extends (yield 1) {}
 }
 
 x = class extends (++b) {};
```
# js/classes-private-fields/with_comments.js
```diff
 class A {
   #foobar =
-    // comment to break
-    1 +
+  // comment to break
+  1 +
     // comment to break again
     2;
 }
```
# js/classes/assignment.js
```diff
-aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends (
-  aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg1
-) {
-  method() {
-    console.log("foo");
-  }
-};
+aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 =
+  class extends (
+    aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg1
+  ) {
+    method() {
+      console.log("foo");
+    }
+  };
 
-foo = class extends bar {
-  method() {
-    console.log("foo");
-  }
-};
+foo =
+  class extends bar {
+    method() {
+      console.log("foo");
+    }
+  };
 
-aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 = class extends (
-  bar
-) {
-  method() {
-    console.log("foo");
-  }
-};
+aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 =
+  class extends bar {
+    method() {
+      console.log("foo");
+    }
+  };
 
-foo = class extends (
-  aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2
-) {
-  method() {
-    console.log("foo");
-  }
-};
+foo =
+  class extends aaaaaaaa.bbbbbbbb.cccccccc.dddddddd.eeeeeeee.ffffffff.gggggggg2 {
+    method() {
+      console.log("foo");
+    }
+  };
 
-module.exports = class A extends B {
-  method() {
-    console.log("foo");
-  }
-};
+module.exports =
+  class A extends B {
+    method() {
+      console.log("foo");
+    }
+  };
```
# js/classes/binary.js
```diff
-(class {} + 1);
-(class a {} + 1);
-(class extends b {} + 1);
-(class a extends b {} + 1);
+(class {}) + 1;
+(class a {}) + 1;
+(class extends b {}) + 1;
+(class a extends b {}) + 1;
```
# js/classes/call.js
```diff
-(class {}(class {}));
+(class {})(class {});
```
# js/classes/member.js
```diff
-(class {}[1]);
-(class {}.a);
+(class {})[1];
+(class {}).a;
```
# js/classes/method.js
```diff
 class C {
-  name /*comment*/() {}
+  name /*comment*/ () {}
 }
 
-({
-  name /*comment*/() {},
-});
+({ name /*comment*/ () {} });
```
# js/classes/new.js
```diff
-new (class {})();
+new class {}();
 new Ctor(class {});
```
# js/classes/property.js
```diff
 class A {
   foobar =
-    // comment to break
-    1 +
+  // comment to break
+  1 +
     // comment to break again
     2;
 }
 
 class B {
-  someInstanceProperty =
-    this.props.foofoofoofoofoofoo && this.props.barbarbarbar;
+  someInstanceProperty = this.props.foofoofoofoofoofoo && this.props.barbarbarbar;
 
   someInstanceProperty2 = {
     foo: this.props.foofoofoofoofoofoo && this.props.barbarbarbar,
   };
 
   someInstanceProperty3 = "foo";
 }
```
# js/classes/ternary.js
```diff
-if (1) (class {} ? 1 : 2);
+if (1) {
+  (class {}) ? 1 : 2;
+}
```
# js/comments-closure-typecast/binary-expr.js
```diff
-var a = b || /** @type {string} */ (c);
+var a =
+  b || /** @type {string} */
+  (c);
```
# js/comments-closure-typecast/closure-compiler-type-cast.js
```diff
 // test to make sure comments are attached correctly
-let inlineComment = /* some comment */ someReallyLongFunctionCall(
-  withLots,
-  ofArguments
+let inlineComment = /* some comment */ (
+  someReallyLongFunctionCall(withLots, ofArguments)
 );
 
 let object = {
-  key: /* some comment */ someReallyLongFunctionCall(withLots, ofArguments),
+  key: (someReallyLongFunctionCall(withLots, ofArguments)), /* some comment */
 };
 
 // preserve parens only for type casts
 let assignment = /** @type {string} */ (getValue());
 let value = /** @type {string} */ (this.members[0]).functionCall();
 
-functionCall(1 + /** @type {string} */ (value), /** @type {!Foo} */ ({}));
+functionCall(
+  1 + /** @type {string} */
+  (value), /** @type {!Foo} */
+  ({}),
+);
 
 function returnValue() {
-  return /** @type {!Array.<string>} */ (["hello", "you"]);
+  return (["hello", "you"]); /** @type {!Array.<string>} */
 }
 
 // Only numberOrString is typecast
 var newArray = /** @type {array} */ (numberOrString).map((x) => x);
-var newArray = /** @type {array} */ (numberOrString).map((x) => x);
-var newArray = test(/** @type {array} */ (numberOrString).map((x) => x));
-var newArray = test(/** @type {array} */ (numberOrString).map((x) => x));
+var newArray = /** @type {array} */ ((numberOrString)).map((x) => x);
+var newArray = test( /** @type {array} */ (numberOrString).map((x) => x));
+var newArray = test( /** @type {array} */ ((numberOrString)).map((x) => x));
 
 // The numberOrString.map CallExpression is typecast
 var newArray = /** @type {array} */ (numberOrString.map((x) => x));
-var newArray = /** @type {array} */ (numberOrString.map((x) => x));
-var newArray = test(/** @type {array} */ (numberOrString.map((x) => x)));
-var newArray = test(/** @type {array} */ (numberOrString.map((x) => x)));
+var newArray = /** @type {array} */ ((numberOrString).map((x) => x));
+var newArray = test( /** @type {array} */ (numberOrString.map((x) => x)));
+var newArray = test( /** @type {array} */ ((numberOrString).map((x) => x)));
 
-test(/** @type {number} */ (num) + 1);
-test(/** @type {!Array} */ (arrOrString).length + 1);
-test(/** @type {!Array} */ (arrOrString).length + 1);
+test( /** @type {number} */ (num) + 1);
+test( /** @type {!Array} */ (arrOrString).length + 1);
+test( /** @type {!Array} */ ((arrOrString)).length + 1);
 
 const data = functionCall(
   arg1,
   arg2,
-  /** @type {{height: number, width: number}} */ (arg3)
+  /** @type {{height: number, width: number}} */ (arg3),
 );
 
 const style = /** @type {{
   width: number,
   height: number,
   marginTop: number,
   marginLeft: number,
   marginRight: number,
   marginBottom: number,
-}} */ ({
-  width,
-  height,
-  ...margins,
-});
+}} */ ({ width, height, ...margins });
 
 const style2 = /**
  * @type {{
  *   width: number,
  * }}
- */ ({
-  width,
-});
+*/ ({ width });
```
# js/comments-closure-typecast/comment-in-the-middle.js
```diff
 var a =
-  /**
-   * bla bla bla
-   * @type {string |
-   * number
-   * }
-   * bla bla bla
-   */
-  //2
-  (window["s"]).toString();
+/**
+ * bla bla bla
+ * @type {string |
+  * number
+ * }
+* bla bla bla
+ */
+//2
+((window["s"])).toString();
 console.log(a.foo());
```
# js/comments-closure-typecast/comment-placement.js
```diff
 const foo1 = /** @type {string} */ (value);
 
 const foo2 =
-  /** @type {string} */
-  (value);
+/** @type {string} */
+(value);
 
 const foo3 =
-  /** @type {string} */
-  (value);
+/** @type {string} */
+(value);
 
-const foo4 = /** @type {string} */ (value);
+const foo4 =
+/** @type {string} */ (value);
 
-const foo5 = /** @type {string} */ (value);
+const foo5 =
+/** @type {string} */ (value);
```
# js/comments-closure-typecast/iife.js
```diff
 const helpers1 = /** @type {Helpers} */ (((helpers = {}) => helpers)());
 
 const helpers2 = /** @type {Helpers} */ (
   (function () {
     return something;
   })()
 );
 
 // TODO: @param is misplaced https://github.com/prettier/prettier/issues/5850
 const helpers = /** @type {Helpers} */ (
-  /** @param {Partial<Helpers>} helpers */
-  ((helpers = {}) => helpers)()
+  (
+    /** @param {Partial<Helpers>} helpers */
+    (helpers = {}) => helpers
+  )()
 );
```
# js/comments-closure-typecast/issue-4124.js
```diff
 /** @type {Object} */ (myObject.property).someProp = true;
-/** @type {Object} */ (myObject.property).someProp = true;
+( /** @type {Object} */ (myObject.property)).someProp = true;
 
 const prop = /** @type {Object} */ (myObject.property).someProp;
 
-const test =
-  /** @type (function (*): ?|undefined) */
-  (goog.partial(NewThing.onTemplateChange, rationaleField, typeField));
+const test = /** @type (function (*): ?|undefined) */ (
+  goog.partial(NewThing.onTemplateChange, rationaleField, typeField)
+);
 
 const test = /** @type (function (*): ?|undefined) */ (
   goog.partial(NewThing.onTemplateChange, rationaleField, typeField)
 );
 
 const model = /** @type {?{getIndex: Function}} */ (model);
 
 const foo = /** @type {string} */ (bar);
 
 const test = /** @type (function (*): ?|undefined) */ (foo);
```
# js/comments-closure-typecast/issue-8045.js
```diff
-const myLongVariableName =
-  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (fooBarBaz);
+const myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (
+  fooBarBaz
+);
 
 function jsdocCastInReturn() {
-  return /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (
-    fooBarBaz
-  );
+  return (fooBarBaz); /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */
 }
 
-const myLongVariableName =
-  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */
-  (fooBarBaz);
+const myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (
+  fooBarBaz
+);
 
 function jsdocCastInReturn() {
   return (
     /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */
     (fooBarBaz)
   );
 }
 
-const myLongVariableName =
-  /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */
-  (fooBarBaz);
+const myLongVariableName = /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */ (
+  fooBarBaz
+);
 
 function jsdocCastInReturn() {
   return (
     /** @type {ThisIsAVeryLongTypeThatShouldTriggerLineWrapping} */
     (fooBarBaz)
   );
 }
```
# js/comments-closure-typecast/issue-9358.js
```diff
-const fooooba1 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (
-  fooobaarbazzItems || foo
-);
-const fooooba2 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (
-  fooobaarbazzItems + foo
-);
+const fooooba1 = /** @type {Array.<fooo.barr.baaaaaaz>} */ fooobaarbazzItems || foo;
+const fooooba2 = /** @type {Array.<fooo.barr.baaaaaaz>} */ fooobaarbazzItems + foo;
 const fooooba3 = /** @type {Array.<fooo.barr.baaaaaaz>} */ (
   fooobaarbazzItems || foo
-)
-  ? foo
-  : bar;
+) ? foo : bar;
```
# js/comments-closure-typecast/member.js
```diff
-foo = /** @type {!Baz} */ (baz).bar;
+foo = ( /** @type {!Baz} */ (baz).bar);
```
# js/comments-closure-typecast/nested.js
```diff
-foo = /** @type {!Foo} */ (/** @type {!Baz} */ (baz).bar);
+foo = /** @type {!Foo} */ ( /** @type {!Baz} */ (baz).bar);
 
 const BarImpl = /** @type {BarConstructor} */ (
   /** @type {unknown} */
-  (
-    function Bar() {
-      throw new Error("Internal error: Illegal constructor");
-    }
-  )
+  (function Bar() {
+    throw new Error("Internal error: Illegal constructor");
+  })
 );
```
# js/comments-closure-typecast/non-casts.js
```diff
 /* @type { } */
-z((x) => {
-  foo(bar(2 + 3));
-  return 1;
-});
+z(
+  (x) => {
+    (foo)((bar)(2 + (3)));
+    return (1);
+  },
+);
 
 /** @type { } */
-z((x) => {
-  foo(bar(2 + 3));
-  return 1;
-});
+z(
+  (x) => {
+    (foo)((bar)(2 + (3)));
+    return (1);
+  },
+);
 
 /** @type {number} */
-let q = z((x) => {
-  return 1;
-});
+let q = z(
+  (x) => {
+    return (1);
+  },
+);
 
-const w1 = /** @typefoo Foo */ value;
+const w1 = /** @typefoo Foo */ (value);
```
# js/comments-closure-typecast/object-with-comment.js
```diff
 const objectWithComment = /** @type MyType */ (
   /* comment */
-  {
-    foo: bar,
-  }
+  { foo: bar }
 );
 
-const objectWithComment2 = /** @type MyType */ (
-  /* comment */ {
-    foo: bar,
-  }
-);
+const objectWithComment2 = /** @type MyType */ ( /* comment */ { foo: bar });
```
# js/comments-closure-typecast/styled-components.js
```diff
 const OverlapWrapper =
-  /** @type {import('styled-components').ThemedStyledFunction<'div',null,{overlap: boolean}>} */
-  (styled.div)`
-    position: relative;
+/** @type {import('styled-components').ThemedStyledFunction<'div',null,{overlap: boolean}>} */
+(styled.div)`
+position:relative;
     > {
-      position: absolute;
-      bottom: ${(p) => p.overlap === "previous" && 0};
-      top: ${(p) => p.overlap === "next" && 0};
-    }
-  `;
+  position: absolute;
+  bottom: ${(p) => p.overlap === "previous" && 0};
+top: ${(p) => p.overlap === "next" && 0};
+}
+`;
```
# js/comments-pipeline-own-line/pipeline_own_line.js
```diff
 function pipeline() {
-  0 |>
-    // Comment
-    x;
+  0
+	// Comment
+	|> x
 }
 
 bifornCringerMoshedPerplexSawder(
   askTrovenaBeenaDependsRowans,
   glimseGlyphsHazardNoopsTieTie,
   averredBathersBoxroomBuggyNurl
 ) // comment
-  |> kochabCooieGameOnOboleUnweave
-  |> glimseGlyphsHazardNoopsTieTie;
+|> kochabCooieGameOnOboleUnweave
+|> glimseGlyphsHazardNoopsTieTie;
 
 bifornCringerMoshedPerplexSawder(
   askTrovenaBeenaDependsRowans,
   glimseGlyphsHazardNoopsTieTie
 )
-  |> foo // comment
-  |> kochabCooieGameOnOboleUnweave
-  |> glimseGlyphsHazardNoopsTieTie;
+|> foo // comment
+|> kochabCooieGameOnOboleUnweave
+|> glimseGlyphsHazardNoopsTieTie;
 
 bifornCringerMoshedPerplexSawder[
   askTrovenaBeenaDependsRowans +
-    glimseGlyphsHazardNoopsTieTie +
-    averredBathersBoxroomBuggyNurl
+  glimseGlyphsHazardNoopsTieTie +
+  averredBathersBoxroomBuggyNurl
 ] // comment
-  |> kochabCooieGameOnOboleUnweave
-  |> glimseGlyphsHazardNoopsTieTie;
+|> kochabCooieGameOnOboleUnweave
+|> glimseGlyphsHazardNoopsTieTie;
 
 bifornCringerMoshedPerplexSawder[
   askTrovenaBeenaDependsRowans +
-    glimseGlyphsHazardNoopsTieTie +
-    averredBathersBoxroomBuggyNurl
+  glimseGlyphsHazardNoopsTieTie +
+  averredBathersBoxroomBuggyNurl
 ]
-  |> foo // comment
-  |> kochabCooieGameOnOboleUnweave
-  |> glimseGlyphsHazardNoopsTieTie;
+|> foo // comment
+|> kochabCooieGameOnOboleUnweave
+|> glimseGlyphsHazardNoopsTieTie;
```
# js/comments/arrow.js
```diff
-const fn = (/*event, data*/) => doSomething();
+const fn = ( /*event, data*/ ) => doSomething();
 
-const fn2 = (/*event, data*/) => doSomething(anything);
+const fn2 = ( /*event, data*/ ) => doSomething(anything);
```
# js/comments/before-comma.js
```diff
 const foo = {
-  a: "a" /* comment for this line */,
+  a: "a", /* comment for this line */
 
   /* Section B */
   b: "b",
 };
```
# js/comments/binary-expressions-block-comments.js
```diff
-a = b /** Comment */ || c;
+a =
+  b || /** Comment */
+  c;
 
 a = b /** Comment */ || c;
 
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||
+  b || /** TODO this is a very very very very long comment that makes it go > 80 columns */
   c;
 
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ ||
+  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ || c;
+
+a =
+  b || /** TODO this is a very very very very long comment that makes it go > 80 columns */
   c;
 
 a =
-  b ||
-  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;
+  b && /** Comment */
+  c;
 
 a = b /** Comment */ && c;
 
-a = b /** Comment */ && c;
-
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&
+  b && /** TODO this is a very very very very long comment that makes it go > 80 columns */
   c;
 
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ &&
+  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ && c;
+
+a =
+  b && /** TODO this is a very very very very long comment that makes it go > 80 columns */
   c;
 
 a =
-  b &&
-  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;
-
-a = b /** Comment */ + c;
+  b + /** Comment */
+  c;
 
 a = b /** Comment */ + c;
 
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +
+  b + /** TODO this is a very very very very long comment that makes it go > 80 columns */
   c;
 
 a =
-  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ +
-  c;
+  b /** TODO this is a very very very very long comment that makes it go > 80 columns */ + c;
 
 a =
-  b +
-  /** TODO this is a very very very very long comment that makes it go > 80 columns */ c;
+  b + /** TODO this is a very very very very long comment that makes it go > 80 columns */
+  c;
```
# js/comments/binary-expressions-parens.js
```diff
 Math.min(
-  /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the
-   * deployment of v0.38.0. To see the error, remove this comment and
-   * run flow */
-  document.body.scrollHeight -
-    (window.scrollY + window.innerHeight) -
-    devsite_footer_height,
-  0
+  (
+    /* $FlowFixMe(>=0.38.0 site=www) - Flow error detected during the
+     * deployment of v0.38.0. To see the error, remove this comment and
+     * run flow */
+    document.body.scrollHeight - (window.scrollY + window.innerHeight)
+  ) - devsite_footer_height,
+  0,
 );
```
# js/comments/binary-expressions.js
```diff
 function addition() {
-  0 +
-    // Comment
-    x;
+  0
+  // Comment
+  + x;
 }
 
 function multiplication() {
-  0 *
-    // Comment
-    x;
+  0
+  // Comment
+  * x;
 }
 
 function division() {
-  0 /
-    // Comment
-    x;
+  0
+  // Comment
+  / x;
 }
 
 function substraction() {
-  0 -
-    // Comment
-    x;
+  0
+  // Comment
+  - x;
 }
 
 function remainder() {
-  0 %
-    // Comment
-    x;
+  0
+  // Comment
+  % x;
 }
 
 function exponentiation() {
-  0 **
-    // Comment
-    x;
+  0
+  // Comment
+  ** x;
 }
 
 function leftShift() {
-  0 <<
-    // Comment
-    x;
+  0
+  // Comment
+  << x;
 }
 
 function rightShift() {
-  0 >>
-    // Comment
-    x;
+  0
+  // Comment
+  >> x;
 }
 
 function unsignedRightShift() {
-  0 >>>
-    // Comment
-    x;
+  0
+  // Comment
+  >>> x;
 }
 
 function bitwiseAnd() {
-  0 &
-    // Comment
-    x;
+  0
+  // Comment
+  & x;
 }
 
 function bitwiseOr() {
-  0 |
-    // Comment
-    x;
+  0
+  // Comment
+  | x;
 }
 
 function bitwiseXor() {
-  0 ^
-    // Comment
-    x;
+  0
+  // Comment
+  ^ x;
 }
```
# js/comments/break-continue-statements.js
```diff
 for (;;) {
   break; /* comment */
   continue; /* comment */
 }
 
 loop: for (;;) {
-  break /* comment */ loop;
-  break loop /* comment */;
-  continue /* comment */ loop;
-  continue loop /* comment */;
+  break loop; /* comment */
+  break loop; /* comment */
+  continue loop; /* comment */
+  continue loop; /* comment */
 }
```
# js/comments/call_comment.js
```diff
 render(
   // Warm any cache
   <ChildUpdates renderAnchor={true} anchorClassOn={true} />,
-  container
+  container,
 );
 
 React.render(
   // Warm any cache
   <ChildUpdates renderAnchor={true} anchorClassOn={true} />,
-  container
+  container,
 );
 
 render?.(
   // Warm any cache
   <ChildUpdates renderAnchor={true} anchorClassOn={true} />,
-  container
+  container,
 );
```
# js/comments/dangling.js
```diff
-var a = {
-  /* dangling */
-};
+var a = { /* dangling */ };
 var b = {
   // dangling
 };
-var b = [
-  /* dangling */
-];
+var b = [ /* dangling */ ];
 function d() {
   /* dangling */
 }
-new Thing(/* dangling */);
-Thing(/* dangling */);
-export /* dangling */ {};
+new Thing( /* dangling */ );
+Thing( /* dangling */ );
+export {}; /* dangling */
```
# js/comments/dangling_array.js
```diff
-expect(() => {}).toTriggerReadyStateChanges([
-  // Nothing.
-]);
+expect(() => {})
+  .toTriggerReadyStateChanges([
+    // Nothing.
+  ]);
 
-[1 /* first comment */, 2 /* second comment */, 3];
+[1 /* first comment */ , 2 /* second comment */ , 3];
```
# js/comments/dangling_for.js
```diff
-// comment
-for (;;);
+for (;;); // comment
 
-/* comment */
-for (;;);
+for /* comment */ (;;);
```
# js/comments/dynamic_imports.js
```diff
-import(/* Hello */ "something");
+import( /* Hello */ "something");
 
-import("something" /* Hello */);
+import("something" /* Hello */ );
 
-import(/* Hello */ "something" /* Hello */);
+import( /* Hello */ "something" /* Hello */ );
 
 import("something" /* Hello */ + "else");
 
 import(
   /* Hello */
-  "something"
+  "something",
   /* Hello */
 );
 
-wrap(import(/* Hello */ "something"));
+wrap(import( /* Hello */ "something"));
```
# js/comments/export.js
```diff
-export //comment
- {};
+export {}; //comment
 
-export /* comment */ {};
+export {}; /* comment */
 
 const foo = "";
 export {
   foo, // comment
 };
 
 const bar = "";
 export {
   // comment
   bar,
 };
 
 const fooo = "";
 const barr = "";
 export {
   fooo, // comment
   barr, // comment
 };
```
# js/comments/function-declaration.js
```diff
-function a(/* comment */) {} // comment
+function a( /* comment */ ) {} // comment
 function b() {} // comment
-function c(/* comment */ argA, argB, argC) {} // comment
-call((/*object*/ row) => {});
+function c( /* comment */ argA, argB, argC) {} // comment
+call(( /*object*/ row) => {});
 KEYPAD_NUMBERS.map(
-  (
-    num // Buttons 0-9
-  ) => <div />
+  (num) => (
+    // Buttons 0-9
+    <div />
+  ),
 );
 
-function f1 /* f */() {}
-function f2(/* args */) {}
+function f1 /* f */ () {}
+function f2( /* args */ ) {}
 function f3() /* returns */ {}
-function f4 /* f */(/* args */) /* returns */ {}
+function f4 /* f */ ( /* args */ ) /* returns */ {}
 
-function f5 /* f */(/* a */ a) {}
-function f6 /* f */(a /* a */) {}
-function f7 /* f */(/* a */ a) /* returns */ {}
+function f5 /* f */ ( /* a */ a) {}
+function f6 /* f */ (a /* a */ ) {}
+function f7 /* f */ ( /* a */ a) /* returns */ {}
 
 const obj = {
-  f1 /* f */() {},
-  f2(/* args */) {},
+  f1 /* f */ () {},
+  f2( /* args */ ) {},
   f3() /* returns */ {},
-  f4 /* f */(/* args */) /* returns */ {},
+  f4 /* f */ ( /* args */ ) /* returns */ {},
 };
 
-(function f /* f */() {})();
-(function f(/* args */) {})();
+(function f /* f */ () {})();
+(function f( /* args */ ) {})();
 (function f() /* returns */ {})();
-(function f /* f */(/* args */) /* returns */ {})();
+(function f /* f */ ( /* args */ ) /* returns */ {})();
 
 class C1 {
-  f /* f */() {}
+  f /* f */ () {}
 }
 class C2 {
-  f(/* args */) {}
+  f( /* args */ ) {}
 }
 class C3 {
   f() /* returns */ {}
 }
 class C4 {
-  f /* f */(/* args */) /* returns */ {}
+  f /* f */ ( /* args */ ) /* returns */ {}
 }
 
-function foo1() {
-  // this is a function
+function foo1()
+// this is a function
+{
   return 42;
 }
 
 function foo2() {
   // this is a function
   return 42;
 }
 
 function foo3() {
   // this is a function
   return 42;
 }
 
 function foo4() {
   // this is a function
   return 42;
 }
```
# js/comments/html-like/comment.js
```diff
 <!--
-alert(1); 
+alert(1)
 -->
```
# js/comments/if.js
```diff
-if (1) {
-  // comment
+if (1)
+// comment
+{
   false;
 }
 // comment
-else if (2) true;
+else if (2) {
+  true;
+}
 // multi
 // ple
 // lines
-else if (3)
+else if (3) {
   // existing comment
   true;
+}
 // okay?
 else if (4) {
   // empty with existing comment
 }
 // comment
 else {
 }
 
-if (5)
+if (5) {
   // comment
   true;
+}
 
 if (6) {
   // comment
   true;
-} else if (7)
+} else if (7) {
   // comment
   true;
-// comment
-else {
+} else {
+  // comment
   true;
 }
 
-if (8) {
-  // comment
-  // comment
+if (8) // comment
+// comment
+{
   true;
-} else if (9)
+} else if (9) {
   // comment
   // comment
   true;
-// comment
+} else // comment
 // comment
-else {
+{
   true;
 }
 
-if (10) {
-  /* comment */ // comment
+if (10) /* comment */ {
+  // comment
+  true;
+} else if (11) /* comment */ {
   true;
-} else if (11) /* comment */ true;
-else if (12)
+} else if (12) {
   // comment /* comment */ // comment
   true;
-else if (13)
-  /* comment */ /* comment */ // comment
+} else if (13) /* comment */ /* comment */ {
+  // comment
   true;
-/* comment */ else {
+} else /* comment */ {
   true;
 }
 
-if (14) {
-  // comment
-  /* comment */
-  // comment
+if (14) // comment
+/* comment */
+// comment
+{
   true;
-} else if (15)
+} else if (15) {
   // comment
   /* comment */
   /* comment */ // comment
   true;
+}
```
# js/comments/issue-3532.js
```diff
 import React from "react";
 
 /*
 import styled from 'react-emotion';
 
 const AspectRatioBox = styled.div`
   &::before {
     content: '';
     width: 1px;
     margin-left: -1px;
     float: left;
     height: 0;
     padding-top: ${props => 100 / props.aspectRatio}%;
   }
 
   &::after {
     /* To clear float */ /*
     content: '';
     display: table;
     clear: both;
   }
 `;
 */
 
 const AspectRatioBox = ({ aspectRatio, children, ...props }) => (
   <div
     className={`height: 0;
   overflow: hidden;
-  padding-top: ${(props) => 100 / props.aspectRatio}%;
+  padding-top: ${props => 100 / props.aspectRatio}%;
   background: white;
   position: relative;`}
   >
     <div>{children}</div>
   </div>
 );
 
 export default AspectRatioBox;
```
# js/comments/issues.js
```diff
 // Does not need to break as it fits in 80 columns
 this.call(a, /* comment */ b);
 
 // Comments should either stay at the end of the line or always before, but
 // not one before and one after.
 throw new ProcessSystemError({
   code: acc.error.code, // Alias of errno
   originalError: acc.error, // Just in case.
 });
 
 // Missing one level of indentation because of the comment
-const rootEpic = (actions, store) =>
+const rootEpic = (actions, store) => (
   combineEpics(...epics)(actions, store)
     // Log errors and continue.
-    .catch((err, stream) => {
-      getLogger().error(err);
-      return stream;
-    });
+    .catch(
+      (err, stream) => {
+        getLogger().error(err);
+        return stream;
+      },
+    )
+);
 
 // optional trailing comma gets moved all the way to the beginning
 const regex = new RegExp(
   "^\\s*" + // beginning of the line
     "name\\s*=\\s*" + // name =
-    "['\"]" + // opening quotation mark
+    '[\'"]' + // opening quotation mark
     escapeStringRegExp(target.name) + // target name
-    "['\"]" + // closing quotation mark
-    ",?$" // optional trailing comma
+    '[\'"]' + // closing quotation mark
+    ",?$", // optional trailing comma
 );
 
 // The comment is moved and doesn't trigger the eslint rule anymore
 import path from "path"; // eslint-disable-line nuclide-internal/prefer-nuclide-uri
 
 // Comments disappear in-between MemberExpressions
 Observable.of(process)
   // Don't complete until we say so!
   .merge(Observable.never())
   // Get the errors.
   .takeUntil(throwOnError ? errors.flatMap(Observable.throw) : errors)
   .takeUntil(exit);
 
 // Comments disappear inside of JSX
-<div>{/* Some comment */}</div>;
+<div>
+  {/* Some comment */}
+</div>;
 
 // Comments in JSX tag are placed in a non optimal way
 <div
 // comment
 />;
 
 // Comments disappear in empty blocks
 if (1) {
   // Comment
 }
 
 // Comments trigger invalid JavaScript in-between else if
 if (1) {
 }
 // Comment
 else {
 }
 
 // The comment makes the line break in a weird way
 const result = asyncExecute("non_existing_command", /* args */ []);
 
 // The closing paren is printed on the same line as the comment
 foo(
-  {}
+  {},
   // Hi
 );
```
# js/comments/jsdoc.js
```diff
 /** @type {any} */
 const x = (
   <div>
-    <div />
-  </div>
+        <div />
+    </div>
 );
 
 /**
  * @type {object}
  */
 () => (
   <div>
-    sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp
-  </div>
+        sajdfpoiasdjfpoiasdjfpoiasdjfpoiadsjfpaoisdjfapsdiofjapioisadfaskfaspiofjp
+    </div>
 );
 
 /**
  * @type {object}
  */
 function HelloWorld() {
   return (
     <div>
-      <span>Test</span>
-    </div>
+           <span>Test</span>
+        </div>
   );
 }
```
# js/comments/jsx.js
```diff
-<div>{/* comment */}</div>;
+<div>
+  {
+    /* comment */
+  }
+</div>;
 
-<div>{/* comment */}</div>;
+<div>
+  {/* comment */
+  }
+</div>;
 
 <div>
   {/* comment
-   */}
+*/
+  }
 </div>;
 
 <div>
-  {
-    a
-    /* comment
-     */
+  {a/* comment
+*/
   }
 </div>;
 
 <div>
-  {
-    /* comment
-     */
-    a
+  {/* comment
+*/
+  a
   }
 </div>;
 
-<div>{/* comment */}</div>;
+<div>
+  {/* comment */
+  }
+</div>;
 
-<div>{/* comment */}</div>;
+<div>
+  {/* comment */}
+</div>;
 
 <div>
   {
     // single line comment
   }
 </div>;
 
 <div>
   {
     // multiple line comments 1
     // multiple line comments 2
   }
 </div>;
 
 <div>
   {
     // multiple mixed comments 1
     /* multiple mixed comments 2 */
     /* multiple mixed comments 3 */
     // multiple mixed comments 4
   }
 </div>;
 
 <div>
   {
     // Some very v  ery very very merry (xmas) very very long line to break line width limit
   }
 </div>;
 
-<div>
-  {/*<div>  Some very v  ery very very long line to break line width limit </div>*/}
-</div>;
+<div>{/*<div>  Some very v  ery very very long line to break line width limit </div>*/}</div>;
 
 <div>
   {/**
    * JSDoc-y comment in JSX. I wonder what will happen to it?
-   */}
+  */}
 </div>;
 
 <div>
-  {/**
+  {
+    /**
    * Another JSDoc comment in JSX.
-   */}
+  */
+  }
 </div>;
 
 <div
   /**
-   * Handles clicks.
-   */
-  onClick={() => {}}
-></div>;
+ * Handles clicks.
+*/
+onClick={() => {}}>
+
+</div>;
 
 <div
-// comment
+  // comment
 >
   {foo}
 </div>;
 
 <div
   className="foo" // comment
 >
   {foo}
 </div>;
 
 <div
   className="foo"
   // comment
 >
   {foo}
 </div>;
 
 <div // comment
   id="foo"
 >
   {children}
 </div>;
 
 <Wrapper>
   {}
   <Component />
 </Wrapper>;
```
# js/comments/last-arg.js
```diff
 class Foo {
-  a(lol /*string*/) {}
+  a(lol /*string*/ ) {}
 
-  b(lol /*string*/) {}
+  b(lol /*string*/ ) {}
 
-  d(lol /*string*/, lol2 /*string*/, lol3 /*string*/, lol4 /*string*/) {}
+  d(lol /*string*/ , lol2 /*string*/ , lol3 /*string*/ , lol4 /*string*/ ) {}
 
-  d(
-    lol /*string*/,
-    lol2 /*string*/,
-    lol3 /*string*/,
-    lol4 /*string*/
-  ) /*string*/ {}
+  d(lol /*string*/ , lol2 /*string*/ , lol3 /*string*/ , lol4 /*string*/ ) /*string*/ {}
 
   // prettier-ignore
   c(lol /*string*/
   ) {}
 
   // prettier-ignore
   d(
     lol /*string*/,
     lol2 /*string*/,
     lol3 /*string*/,
     lol4 /*string*/
   ) {}
 
   // prettier-ignore
   e(
     lol /*string*/,
     lol2 /*string*/,
     lol3 /*string*/,
     lol4 /*string*/
   ) {} /* string*/
 }
```
# js/comments/multi-comments-on-same-line.js
```diff
 /*========= All on same line =========*/
 a;
 /*1*/ /*2*/ /*3*/
 b;
 
 a; /*1*/ /*2*/ /*3*/
 b;
 
 a;
 /*1*/ /*2*/ /*3*/ b;
 
 a;
 /*
 1*/ /*2*/ /*3
- */
+*/
 b;
 
 a; /*
-1*/ /*2*/
-/*3
- */
+1*/ /*2*/ /*3
+*/
 b;
 
-a;
-/*
+a; /*
 1*/ /*2*/ /*3
- */ b;
+*/
+b;
 
 /*========= First two on same line =========*/
 a;
 /*1*/ /*2*/
 /*3*/
 b;
 
 a; /*1*/ /*2*/
 /*3*/
 b;
 
 a;
 /*1*/ /*2*/
 /*3*/ b;
 
 a;
 /*
 1*/ /*2*/
 /*3
- */
+*/
 b;
 
 a; /*
 1*/ /*2*/
 /*3
- */
+*/
 b;
 
 a; /*
 1*/ /*2*/
 /*3
- */ b;
+*/ b;
 
 /*========= Last two on same line =========*/
 a;
 /*1*/
 /*2*/ /*3*/
 b;
 
 a; /*1*/
 /*2*/ /*3*/
 b;
 
 a;
 /*1*/
 /*2*/ /*3*/ b;
 
 a;
 /*
 1*/
 /*2*/ /*3
- */
+*/
 b;
 
 a; /*
 1*/
 /*2*/ /*3
- */
+*/
 b;
 
 a; /*
 1*/
 /*2*/ /*3
- */ b;
+*/ b;
```
# js/comments/preserve-new-line-last.js
```diff
 function f() {
   a;
   /* eslint-disable */
 }
 
 function f() {
   a;
-
   /* eslint-disable */
 }
 
 function name() {
   // comment1
   func1();
 
   // comment2
   func2();
-
   // comment3 why func3 commented
   // func3()
 }
```
# js/comments/return-statement.js
```diff
 function jsx() {
   return (
     // Comment
     <div />
   );
 }
 
 function unary() {
   return (
     // Comment
     !!x
   );
 }
 
 function numericLiteralNoParen() {
   return 1337; // Comment
 }
 
 function logical() {
   return (
     // Reason for 42
-    42 && 84
-  );
+    42
+  ) && 84;
 }
 
 function binary() {
   return (
     // Reason for 42
-    42 * 84
-  );
+    42
+  ) * 84;
 }
 
 function binaryInBinaryLeft() {
   return (
-    // Reason for 42
-    42 *
-      84 +
-    2
-  );
+    (
+      // Reason for 42
+      42
+    ) * 84
+  ) + 2;
 }
 
 function binaryInBinaryRight() {
   return (
     // Reason for 42
-    42 +
-    84 * 2
-  );
+    42
+  ) + (84 * 2);
 }
 
 function conditional() {
   return (
     // Reason for 42
     42
-      ? 1
-      : 2
-  );
+  ) ? 1 : 2;
 }
 
 function binaryInConditional() {
   return (
     // Reason for 42
-    42 * 3
-      ? 1
-      : 2
-  );
+    42
+  ) * 3 ? 1 : 2;
 }
 
 function call() {
   return (
     // Reason for a
-    a()
-  );
+    a
+  )();
 }
 
 function memberInside() {
   return (
     // Reason for a.b
-    a.b.c
-  );
+    a.b
+  ).c;
 }
 
 function memberOutside() {
   return (
     // Reason for a
-    a.b.c
-  );
+    a
+  ).b.c;
 }
 
 function memberInAndOutWithCalls() {
   return (
-    aFunction
-      .b// Reason for a
-      ()
-      .c.d()
-  );
+    // Reason for a
+    aFunction.b()
+  ).c.d();
 }
 
 function excessiveEverything() {
   return (
     // Reason for stuff
-    a.b() * 3 + 4 ? ((a`hi`, 1) ? 1 : 1) : 1
+    (a.b() * 3) + 4
+      ? (a`hi`, 1)
+        ? 1
+        : 1
+      : 1
   );
 }
 
 // See https://github.com/prettier/prettier/issues/2392
 // function sequenceExpression() {
 //   return (
 //     // Reason for a
 //     a
 //   ), b
 // }
 
 function sequenceExpressionInside() {
   return (
     // Reason for a
     a, b
   );
 }
 
 function taggedTemplate() {
   return (
     // Reason for a
-    a`b`
-  );
+    a
+  )`b`;
 }
 
 function inlineComment() {
-  return /* hi */ 42 || 42;
+  return (
+    /* hi */ 42
+  ) || 42;
 }
```
# js/comments/single-star-jsdoc.js
```diff
 /*
  * Looking good!
  */
 
 if (true) {
   /*
-   * Oh no
-   */
+     * Oh no
+     */
 }
-
 /** first line
- * second line
- * third line */
+* second line
+   * third line */
 
 /* first line
- * second line
- * third line */
+* second line
+   * third line */
 
 /*! first line
- *second line
- *  third line */
+*second line
+   *  third line */
 
 /*!
- * Extracted from vue codebase
- * https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js
- * HTML Parser By John Resig (ejohn.org)
- * Modified by Juriy "kangax" Zaytsev
- * Original code by Erik Arvidsson, Mozilla Public License
- * http://erik.eae.net/simplehtmlparser/simplehtmlparser.js
- */
+* Extracted from vue codebase
+* https://github.com/vuejs/vue/blob/cfd73c2386623341fdbb3ac636c4baf84ea89c2c/src/compiler/parser/html-parser.js
+* HTML Parser By John Resig (ejohn.org)
+* Modified by Juriy "kangax" Zaytsev
+* Original code by Erik Arvidsson, Mozilla Public License
+* http://erik.eae.net/simplehtmlparser/simplehtmlparser.js
+*/
```
# js/comments/switch.js
```diff
 switch (node && node.type) {
   case "Property":
   case "MethodDefinition":
     prop = node.key;
     break;
 
   case "MemberExpression":
     prop = node.property;
     break;
-
   // no default
 }
 
 switch (foo) {
   case "bar":
     doThing();
-
   // no default
 }
 
 switch (foo) {
-  case "bar": //comment
+  case "bar":
+    //comment
     doThing(); //comment
 
   case "baz":
     doOtherThing(); //comment
 }
 
 switch (foo) {
   case "bar": {
     doThing();
   } //comment
 
   case "baz": {
     doThing();
   } //comment
 }
```
# js/comments/tagged-template-literal.js
```diff
 foo``; // comment
 
-foo // comment
-``;
+foo``; // comment
 
-foo // comment
-`
-`;
+foo`
+`; // comment
 
-foo/* comment */ `
+foo /* comment */ `
 `;
 
-foo /* comment */`
+foo /* comment */ `
 `;
```
# js/comments/template-literal.js
```diff
 `
 ${
   a // comment
 }
 
-${b /* comment */}
+${b /* comment */ }
 
-${/* comment */ c /* comment */}
+${ /* comment */ c /* comment */ }
 
 ${
   // comment
   d //comment
 };
 `;
```
# js/comments/trailing-jsdocs.js
```diff
-const CONNECTION_STATUS = (exports.CONNECTION_STATUS = {
-  CLOSED: Object.freeze({ kind: "CLOSED" }),
-  CONNECTED: Object.freeze({ kind: "CONNECTED" }),
-  CONNECTING: Object.freeze({ kind: "CONNECTING" }),
-  NOT_CONNECTED: Object.freeze({ kind: "NOT_CONNECTED" }),
-});
-
-/* A comment */
-/**
- * A type that can be written to a buffer.
- */
-/**
- * Describes the connection status of a ReactiveSocket/DuplexConnection.
- * - NOT_CONNECTED: no connection established or pending.
- * - CONNECTING: when `connect()` has been called but a connection is not yet
- *   established.
- * - CONNECTED: when a connection is established.
- * - CLOSED: when the connection has been explicitly closed via `close()`.
- * - ERROR: when the connection has been closed for any other reason.
- */
-/**
- * A contract providing different interaction models per the [ReactiveSocket protocol]
- * (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).
- */
-/**
- * A single unit of data exchanged between the peers of a `ReactiveSocket`.
- */
+const CONNECTION_STATUS = exports.CONNECTION_STATUS =
+  {
+    CLOSED: Object.freeze({ kind: "CLOSED" }),
+    CONNECTED: Object.freeze({ kind: "CONNECTED" }),
+    CONNECTING: Object.freeze({ kind: "CONNECTING" }),
+    NOT_CONNECTED: Object.freeze({ kind: "NOT_CONNECTED" }),
+  };
+/* A comment */ /**
+* A type that can be written to a buffer.
+*/ /**
+* Describes the connection status of a ReactiveSocket/DuplexConnection.
+* - NOT_CONNECTED: no connection established or pending.
+* - CONNECTING: when `connect()` has been called but a connection is not yet
+*   established.
+* - CONNECTED: when a connection is established.
+* - CLOSED: when the connection has been explicitly closed via `close()`.
+* - ERROR: when the connection has been closed for any other reason.
+*/ /**
+* A contract providing different interaction models per the [ReactiveSocket protocol]
+* (https://github.com/ReactiveSocket/reactivesocket/blob/master/Protocol.md).
+*/ /**
+* A single unit of data exchanged between the peers of a `ReactiveSocket`.
+*/
```
# js/comments/trailing_space.js
```diff
-#!/there/is-space-here->
+#!/there/is-space-here->         
 
 // Do not trim trailing whitespace from this source file!
 
 // There is some space here ->
```
# js/comments/try.js
```diff
 // Comment 1
 try {
   // Comment 2
   // Comment 3
-} catch (e) {
-  // Comment 4
+}
+// Comment 4
+catch (e) {
   // Comment 5
   // Comment 6
-} finally {
-  // Comment 7
+}
+// Comment 7
+finally {
   // Comment 8
   // Comment 9
 }
 // Comment 10
```
# js/comments/variable_declarator.js
```diff
-let obj1 =
-  // Comment
-  {
-    key: "val",
-  };
+let obj1 = { key: "val" }; // Comment
 
-let obj2 =
-  // Comment
-  {
-    key: "val",
-  };
+let obj2 = { key: "val" }; // Comment
 
 let obj3 = {
   // Comment
   key: "val",
 };
 
 let obj4 = {
   // Comment
   key: "val",
 };
 
-let obj5 =
-  // Comment
-  ["val"];
+let obj5 = ["val"]; // Comment
 
-let obj6 =
-  // Comment
-  ["val"];
+let obj6 = ["val"]; // Comment
 
 let obj7 = [
   // Comment
   "val",
 ];
 
 let obj8 = [
   // Comment
   "val",
 ];
 
-let obj9 =
-  // Comment
-  `val`;
+let obj9 = `val`; // Comment
 
-let obj10 =
-  // Comment
-  `
+let obj10 = `
 val
 val
-`;
+`; // Comment
 
-let obj11 =
-  // Comment
-  tag`val`;
+let obj11 = tag`val`; // Comment
 
-let obj12 =
-  // Comment
-  tag`
+let obj12 = tag`
 val
 val
-`;
+`; // Comment
 
-let // Comment
-  foo1 = "val";
+let foo1 = "val"; // Comment
 
-let // Comment
-  foo2 = "val",
-  bar = "val";
+let foo2 = "val", bar = "val"; // Comment
 
-const foo3 = 123;
+const foo3 = 123
 // Nothing to see here.
+;
 ["2", "3"].forEach((x) => console.log(x));
```
# js/comments/while.js
```diff
 while (
   true
   // Comment
 ) {}
 
-while (true) {
-  // Comment
-}
+while (true) {} // Comment
 
 while (true) {} // Comment
 
-while (true) {
-  /*Comment*/
-}
+while (true) /*Comment*/ {}
 
 while (
-  true && // Comment
-  true // Comment
+  true && true // Comment // Comment
 ) {}
 
 while (true) {} // comment
 
 while (true) /* comment */ ++x;
 
-while (1)
-  // Comment
-  foo();
+while (1) foo(); // Comment
```
# js/conditional/comments.js
```diff
-var inspect =
-  4 === util.inspect.length
-    ? // node <= 0.8.x
-      function (v, colors) {
-        return util.inspect(v, void 0, void 0, colors);
-      }
-    : // node > 0.8.x
-      function (v, colors) {
-        return util.inspect(v, { colors: colors });
-      };
+var inspect = 4 === util.inspect.length ? (
+  // node <= 0.8.x
+  function (v, colors) {
+    return util.inspect(v, void 0, void 0, colors);
+  }
+) : (
+  // node > 0.8.x
+  function (v, colors) {
+    return util.inspect(v, { colors: colors });
+  }
+);
 
-var inspect =
-  4 === util.inspect.length
-    ? // node <= 0.8.x
-      function (v, colors) {
-        return util.inspect(v, void 0, void 0, colors);
-      }
-    : // node > 0.8.x
-      function (v, colors) {
-        return util.inspect(v, { colors: colors });
-      };
+var inspect = 4 === util.inspect.length ? (
+  // node <= 0.8.x
+  function (v, colors) {
+    return util.inspect(v, void 0, void 0, colors);
+  }
+) : (
+  // node > 0.8.x
+  function (v, colors) {
+    return util.inspect(v, { colors: colors });
+  }
+);
 
 const extractTextPluginOptions = shouldUseRelativeAssetPaths
-  ? // Making sure that the publicPath goes back to to build folder.
-    { publicPath: Array(cssFilename.split("/").length).join("../") }
-  : {};
+// Making sure that the publicPath goes back to to build folder.
+? { publicPath: Array(cssFilename.split("/").length).join("../") } : {};
 
-const extractTextPluginOptions2 = shouldUseRelativeAssetPaths
-  ? // Making sure that the publicPath goes back to to build folder.
-    { publicPath: Array(cssFilename.split("/").length).join("../") }
-  : {};
+const extractTextPluginOptions2 = shouldUseRelativeAssetPaths ? {
+  // Making sure that the publicPath goes back to to build folder.
+  publicPath: Array(cssFilename.split("/").length).join("../"),
+} : {};
 
-const extractTextPluginOptions3 = shouldUseRelativeAssetPaths // Making sure that the publicPath goes back to to build folder.
-  ? { publicPath: Array(cssFilename.split("/").length).join("../") }
-  : {};
+const extractTextPluginOptions3 = shouldUseRelativeAssetPaths ? {
+  // Making sure that the publicPath goes back to to build folder.
+  publicPath: Array(cssFilename.split("/").length).join("../"),
+} : {};
 
-const { configureStore } =
-  process.env.NODE_ENV === "production"
-    ? require("./configureProdStore") // a
-    : require("./configureDevStore"); // b
+const { configureStore } = process.env.NODE_ENV === "production" ? require(
+  "./configureProdStore",
+) : require("./configureDevStore"); // a // b
 
-test /* comment
+test ? foo : bar; /* comment
   comment
       comment
 */
-  ? foo
-  : bar;
 
-test
-  ? /* comment
+test ? foo : bar; /* comment
           comment
     comment
           comment
   */
-    foo
-  : bar;
 
 test
-  ? /* comment
+  ? foo /* comment
        comment
        comment
        comment
     */
-    foo
   : test
-  ? /* comment
+    ? foo /* comment
   comment
     comment */
-    foo
-  : bar;
+    : bar;
 
 test ? /* comment */ foo : bar;
 
-test
-  ? foo
-  : /* comment
+test ? foo : bar; /* comment
          comment
      comment
            comment
     */
-    bar;
 
 test
   ? foo
-  : /* comment
+  : test
+    /* comment
          comment
      comment
            comment
     */
-  test
-  ? foo
-  : /* comment
+    ? foo
+    : bar; /* comment
   comment
     comment
    */
-    bar;
 
 test ? foo : /* comment */ bar;
 
 test
-  ? test /* c
+  ? test
+    /* c
 c */
     ? foo
     : bar
   : bar;
```
# js/conditional/new-expression.js
```diff
 const testConsole = new TestConsole(
-  config.useStderr ? process.stderr : process.stdout
+  config.useStderr ? process.stderr : process.stdout,
 );
```
# js/conditional/no-confusing-arrow.js
```diff
 // no-confusing-arrow
-var x = (a) => (1 ? 2 : 3);
+var x = (a) => 1 ? 2 : 3;
 var x = a <= 1 ? 2 : 3;
```
# js/cursor/cursor-5.js
```diff
-const /* hi */ y = 5;
+const y = 5; /* hi */
```
# js/cursor/range-0.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
-    thisWillBeFormatted(2, 3);
+    thisWillBeFormatted(2, 3)
 
     thisWontBeFormatted  (2, 90  ,)
     
```
# js/cursor/range-1.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
-    thisWillBeFormatted(2, 3);
+    thisWillBeFormatted(2, 3)
 
     thisWontBeFormatted  (2, 90  ,)
     
```
# js/cursor/range-2.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
-    thisWillBeFormatted(2, 3);
+    thisWillBeFormatted(2, 3)
 
     thisWontBeFormatted  (2, 90  ,)
     
```
# js/cursor/range-3.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
-    thisWillBeFormatted(2, 3);
+    thisWillBeFormatted(2, 3)
 
     thisWontBeFormatted  (2, 90  ,)
     
```
# js/cursor/range-4.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
     thisWillBeFormatted(2, 3);
 
-    thisWontBeFormatted  (2, 90  ,)
+thisWontBeFormatted  (2, 90  ,)
     
```
# js/cursor/range-5.js
```diff
-const myVar = aFunction;
+const myVar = aFunction
```
# js/cursor/range-8.js
```diff
 thisWontBeFormatted  ( 1  ,3)
 
-    thisWillBeFormatted(2, 3);
+    thisWillBeFormatted(2, 3)
 
     thisWontBeFormatted  (2, 90  ,)
```
# js/decorator-comments/comments.js
```diff
 class Something {
   @Annotateme()
-  // comment
-  static property: Array<string>;
+    // comment
+    static property: Array<string>;
 }
```
# js/decorators-export/after_export.js
```diff
 export
-@decorator
-class Foo {}
+@decorator class Foo {}
 
 export default
-@decorator
-class {}
+@decorator class {}
```
# js/decorators/classes.js
```diff
-@deco
-class Foo {}
+@deco class Foo {}
 
-@deco
-export class Bar {}
+@deco export class Bar {}
 
-@deco
-export default class Baz {}
+@deco export default class Baz {}
 
 const foo =
-  @deco
-  class {
-    //
-  };
+@deco class {
+  //
+}
 
 const bar =
-  @deco
+
+@deco
   class {
     //
-  };
+  }
```
# js/decorators/comments.js
```diff
 var x = 100;
 
 @Hello({
-  a: "a", // Comment is in the wrong place
+  a: 'a', // Comment is in the wrong place
   // test
-  b: "2",
+  b: '2'
 })
 class X {}
 
 @NgModule({
   // Imports.
   imports: [
     // Angular modules.
     BrowserModule,
 
     // App modules.
     CoreModule,
     SharedModule,
   ],
 })
 export class AppModule {}
 
 // A
 @Foo()
 // B
 @Bar()
 // C
 export class Bar {}
```
# js/decorators/methods.js
```diff
 class Yo {
   @foo("hello")
   async plop() {}
 
   @anotherDecoratorWithALongName("and a very long string as a first argument")
   async plip() {}
 
-  @anotherDecoratorWithALongName("another very long string, but now inline")
-  async plip() {}
+  @anotherDecoratorWithALongName("another very long string, but now inline") async plip() {}
 }
```
# js/decorators/mobx.js
```diff
 import { observable } from "mobx";
 
-@observer
-class OrderLine {
-  @observable price: number = 0;
-  @observable amount: number = 1;
+@observer class OrderLine {
+  @observable price:number = 0;
+  @observable amount:number = 1;
 
   constructor(price) {
     this.price = price;
   }
 
   @computed get total() {
     return this.price * this.amount;
   }
 
   @action.bound setPrice(price) {
     this.price = price;
   }
 
   @computed
   get total() {
     return this.price * this.amount;
   }
 
   @action.bound
   setPrice(price) {
     this.price = price;
   }
 
-  @computed
-  @computed
-  @computed
-  @computed
-  @computed
-  @computed
-  @computed
-  get total() {
+  @computed @computed @computed @computed @computed @computed @computed get total() {
     return this.price * this.amount;
   }
 
   @action handleDecrease = (event: React.ChangeEvent<HTMLInputElement>) =>
     this.count--;
 
   @action handleSomething = (event: React.ChangeEvent<HTMLInputElement>) =>
     doSomething();
 }
```
# js/decorators/multiline.js
```diff
 class Foo {
-  @deco([foo, bar]) prop = value;
+  @deco([
+    foo,
+    bar
+  ]) prop = value;
 
   @decorator([]) method() {}
 
-  @decorator([]) method() {}
+  @decorator([
+  ]) method() {}
 
   @decorator({}) method() {}
 
-  @decorator({}) method() {}
+  @decorator({
+  }) method() {}
 }
```
# js/decorators/multiple.js
```diff
 const dog = {
-  @readonly
+
+@readonly
   @nonenumerable
   @doubledValue
   legs: 4,
 
-  @readonly
+@readonly
   @nonenumerable
   @doubledValue
-  eyes: 2,
-};
+eyes: 2;
+}
 
 const foo = {
-  @multipleDecorators
-  @inline
-  @theyWontAllFitInOneline
-  aVeryLongPropName: "A very long string as value",
-};
+@multipleDecorators @inline @theyWontAllFitInOneline aVeryLongPropName: ("A very long string as value");
+}
```
# js/decorators/redux.js
```diff
 @connect(mapStateToProps, mapDispatchToProps)
 export class MyApp extends React.Component {}
 
-@connect((state) => ({ todos: state.todos }))
+@connect(state => ({ todos: state.todos }))
 export class Home extends React.Component {}
```
# js/destructuring-ignore/ignore.js
```diff
 const {
   // prettier-ignore
   bar =           1,
 } = foo;
 
 const {
   _,
   // prettier-ignore
   bar2 =           1,
 } = foo;
 
 /* comments */
 const {
   // prettier-ignore
   bar3 =           1, // comment
 } = foo;
 
 const {
   // prettier-ignore
-  bar4 =           1 /* comment */,
+  bar4 =           1, /* comment */
 } = foo;
 
 const {
   // prettier-ignore
   bar5 =           /* comment */          1,
 } = foo;
 
 /* RestElement */
 const {
   // prettier-ignore
   ...bar6
 } = foo;
 
 // Nested
 const {
   baz: {
     // prettier-ignore
-    foo2 = [1, 2,    3],
+  foo2 = [1, 2,    3],
   },
   // prettier-ignore
   bar7 =            1,
 } = foo;
```
# js/destructuring/destructuring.js
```diff
 const [one, two = null, three = null] = arr;
 a = ([s = 1]) => 1;
 const { children, ...props } = this.props;
 
-const {
-  user: { firstName, lastName },
-} = this.props;
+const { user: { firstName, lastName } } = this.props;
 
 const {
   name: { first, last },
-  organisation: {
-    address: { street: orgStreetAddress, postcode: orgPostcode },
-  },
+  organisation: { address: { street: orgStreetAddress, postcode: orgPostcode } },
 } = user;
 
 function f({ data: { name } }) {}
 
-const UserComponent = function ({
-  name: { first, last },
-  organisation: {
-    address: { street: orgStreetAddress, postcode: orgPostcode },
+const UserComponent = function (
+  {
+    name: { first, last },
+    organisation: {
+      address: { street: orgStreetAddress, postcode: orgPostcode },
+    },
   },
-}) {
+) {
   return;
 };
 
-const {
-  a,
-  b,
-  c,
-  d: { e },
-} = someObject;
+const { a, b, c, d: { e } } = someObject;
 
 try {
   // code
 } catch ({ data: { message } }) {
   // code
 }
 
 try {
   // code
-} catch ({
-  data: {
-    message: { errors },
-  },
-}) {
+} catch ({ data: { message: { errors } } }) {
   // code
 }
 
 const obj = {
   func(id, { blog: { title } }) {
     return id + title;
   },
 };
 
 class A {
   func(id, { blog: { title } }) {
     return id + title;
   }
 }
```
# js/destructuring/issue-5988.js
```diff
-const {
-  foo,
-  bar: bazAndSomething,
-  quxIsLong,
-} = someBigFunctionName("foo")("bar");
+const { foo, bar: bazAndSomething, quxIsLong } = someBigFunctionName("foo")(
+  "bar",
+);
```
# js/directives/escaped.js
```diff
 // Unnecessary escapes. (adapted from tests/quotes/strings.js)
 // Note that in directives, unnecessary escapes should be preserved.
 // See https://github.com/prettier/prettier/issues/1555
-'\'';
+"\'";
 '\"';
 "\'";
 "\"";
 "\\";
 "\a";
 "hol\a";
 "hol\a";
 "hol\\a (the a is not escaped)";
 "hol\\a (the a is not escaped)";
 "multiple \a unnecessary \a escapes";
 "multiple \a unnecessary \a escapes";
 "unnecessarily escaped character preceded by escaped backslash \\\a";
 "unnecessarily escaped character preceded by escaped backslash \\\a";
 "unescaped character preceded by two escaped backslashes       \\\\a";
 "unescaped character preceded by two escaped backslashes       \\\\a";
 "\a\a"; // consecutive unnecessarily escaped characters
 "\a\a"; // consecutive unnecessarily escaped characters
 "escaped \u2030 \‰ (should still stay escaped)";
 
 // Meaningful escapes
 // Commented out to avoid `SyntaxError: Octal literals are not allowed in strict mode.`
 // "octal escapes \0 \1 \2 \3 \4 \5 \6 \7"
 // 'octal escapes \0 \1 \2 \3 \4 \5 \6 \7'
 "meaningfully escaped alphabetical characters \n \r \v \t \b \f \u2713 \x61";
 "meaningfully escaped alphabetical characters \n \r \v \t \b \f \u2713 \x61";
 "escaped newline \
 ";
 "escaped carriage return \
 ";
 "escaped \u2028 \ ";
 "escaped \u2029 \ ";
```
# js/do/call-arguments.js
```diff
 // from https://github.com/babel/babel/pull/13122/
-expect(do {
-  var bar = "foo";
-  if (!bar) throw new Error("unreachable");
-  bar;
-}).toBe("foo");
+expect(
+do {
+    var bar = "foo";
+    if (!bar) throw new Error(
+      "unreachable"
+    )
+    bar;
+  }
+)
+.toBe("foo")
 expect(bar).toBe("foo");
 
-var x = do {
+var x =
+do {
   var bar = "foo";
-  if (!bar) throw new Error("unreachable");
+  if (!bar) throw new Error(
+    "unreachable"
+  )
   bar;
 };
 
-expect(do {
-  var bar = "foo";
-  bar;
-}).toBe("foo");
+expect(
+do {
+    var bar = "foo";
+    bar;
+  }
+)
+.toBe("foo")
 expect(bar).toBe("foo");
 
-var x = do {
+var x =
+do {
   var bar = "foo";
   bar;
 };
 
 expect(
   () => do {
     () => {
       var bar = "foo";
     };
-    bar;
-  }
-).toThrow(ReferenceError);
+bar;
+}
+).toThrow(ReferenceError)
```
# js/do/do.js
```diff
 const envSpecific = {
-  domain: do {
-    if (env === "production") "https://abc.mno.com/";
-    else if (env === "development") "http://localhost:4000";
-  },
-};
+  domain:
+    do {
+      if(env === 'production') 'https://abc.mno.com/';
+else
+if (env === "development") {
+  ("http://localhost:4000");
+}
+}
+}
 
-let x = do {
+let x =
+do {
   let tmp = f();
-  tmp * tmp + 1;
+  tmp * tmp + 1
 };
 
-let y = do {
-  if (foo()) {
-    f();
-  } else if (bar()) {
-    g();
-  } else {
-    h();
-  }
+let y =
+do {
+  if (foo()) { f() }
+  else if (bar()) { g() }
+  else { h() }
 };
 
 function foo() {
   return (
     <nav>
       <Home />
-      {do {
-        if (loggedIn) {
-          <LogoutButton />;
-        } else {
-          <LoginButton />;
+      {
+        do {
+          if (loggedIn) {
+            <LogoutButton />
+          } else {
+            <LoginButton />
+          }
         }
-      }}
+      }
     </nav>
   );
 }
 
-(do {});
-(do {} + 1);
-1 + do {};
-() => do {};
+(
+do {});
+(
+do {} + 1);
+(1 +
+do {});
+() =>
+do {};
 
-(do {
-  switch (0) {
-    case 0:
-      "foo";
-    case 1:
-      break;
+(
+do {
+  switch(0) {
+    case 0: "foo";
+    case 1: break;
   }
 });
 
-() => do {
+() =>
+do {
   var obj = { foo: "bar", bar: "foo" };
   for (var key in obj) {
     obj[key];
   }
 };
```
# js/empty-paren-comment/class-property.js
```diff
 class Foo {
-  f(/* ... */) {}
+  f( /* ... */ ) {}
   f() /* ... */ {}
-  f = (/* ... */) => {};
-  static f(/* ... */) {}
-  static f = (/* ... */) => {};
-  static f = function (/* ... */) {};
-  static f = function f(/* ... */) {};
+  f = ( /* ... */ ) => {};
+  static f( /* ... */ ) {}
+  static f = ( /* ... */ ) => {};
+  static f = function ( /* ... */ ) {};
+  static f = function f( /* ... */ ) {};
 }
```
# js/empty-paren-comment/class.js
```diff
 class x {
   /**
-   * Set of default settings to be applied to model fetch calls in DAO layer.
-   */
+  * Set of default settings to be applied to model fetch calls in DAO layer.
+  */
   static get defaultSettings() {}
 }
```
# js/empty-paren-comment/empty_paren_comment.js
```diff
-let f1 = (/* ... */) => {};
-(function (/* ... */) {})(/* ... */);
-function f2(/* ... */) {}
+let f1 = ( /* ... */ ) => {};
+(function ( /* ... */ ) {})( /* ... */ );
+function f2( /* ... */ ) {}
 
 const obj = {
-  f(/* ... */) {},
-  f: (/* ... */) => {},
-  f: function (/* ... */) {},
-  f: function f(/* ... */) {},
+  f( /* ... */ ) {},
+  f: ( /* ... */ ) => {},
+  f: function ( /* ... */ ) {},
+  f: function f( /* ... */ ) {},
 };
 
-f(/* ... */);
-f(a /* ... */);
+f( /* ... */ );
+f(a /* ... */ );
 f(a, /* ... */ b);
-f(/* ... */ a, b);
+f( /* ... */ a, b);
 
-let f3 = () => import(a /* ... */);
+let f3 = () => import(a /* ... */ );
 let f4 = () => doThing(a, /* ... */ b);
```
# js/empty-statement/body.js
```diff
 with (a);
-if (1);
-else if (2);
-else;
+if (1) {
+} else if (2) {
+} else {
+}
 for (;;);
 while (1);
 for (var i in o);
 for (var i of o);
-do;
-while (1);
+do while (1);
```
# js/export-default/escaped/default-escaped.js
```diff
 // export asyn\u{63} from "async";
-export nc from "async";
+export
+n\u{63};
+from;
+("async");
```
# js/export-default/function_tostring.js
```diff
-export default (function () {}.toString());
+export default (function () {}).toString();
```
# js/export-extension/export.js
```diff
 export * as ns from "mod";
-export v from "mod";
-export a, * as b from "mod";
-export c, { foo } from "mod";
-export * as d, { bar } from "mod";
+export
+v;
+from;
+("mod");
+export
+a, * as
+b;
+from;
+("mod");
+export
+c, { foo };
+from;
+("mod");
+export * as d
+,
+{
+  bar;
+}
+from;
+("mod");
 export { fooooooooooooooooooooooooooooooooooooooooooooooooo } from "fooooooooooooooooooooooooooooo";
-export Bar, {
-  barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr,
-} from "barrrrrrrrrrrrrrrrrrrrrrrrrrrr";
+export
+Bar, { barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr };
+from;
+("barrrrrrrrrrrrrrrrrrrrrrrrrrrr");
 export {
   foooooooooooooooooooooooooooooooooooooooooooooo,
   fooooooooooooooooooooooooooooooooooooooooooooooo,
 } from "fooooooooooooooooooooooooooooo";
```
# js/exports/test.js
```diff
 export {
   value1,
   value2 as value2_renamed,
   value3,
   value4 as value4_renamed,
   value5,
 } from "exports";
 
-export a, { b } from "./baz";
+export
+a, { b };
+from;
+("./baz");
 
 export * as ns from "mod";
 
-export * as foo, { bar } from "./baz";
+export * as foo
+,
+{
+  bar;
+}
+from;
+("./baz");
 
 export { undefinedExport };
```
# js/first-argument-expansion/test.js
```diff
-setTimeout(function () {
-  thing();
-}, 500);
+setTimeout(
+  function () {
+    thing();
+  },
+  500,
+);
 
-["a", "b", "c"].reduce(function (item, thing) {
-  return thing + " " + item;
-}, "letters:");
+["a", "b", "c"].reduce(
+  function (item, thing) {
+    return thing + " " + item;
+  },
+  "letters:",
+);
 
-func(() => {
-  thing();
-}, identifier);
+func(
+  () => {
+    thing();
+  },
+  identifier,
+);
 
-func(function () {
-  thing();
-}, this.props.timeout * 1000);
+func(
+  function () {
+    thing();
+  },
+  this.props.timeout * 1000,
+);
 
-func((that) => {
-  thing();
-}, this.props.getTimeout());
+func(
+  (that) => {
+    thing();
+  },
+  this.props.getTimeout(),
+);
 
-func(() => {
-  thing();
-}, true);
+func(
+  () => {
+    thing();
+  },
+  true,
+);
 
-func(() => {
-  thing();
-}, null);
+func(
+  () => {
+    thing();
+  },
+  null,
+);
 
-func(() => {
-  thing();
-}, undefined);
+func(
+  () => {
+    thing();
+  },
+  undefined,
+);
 
-func(() => {
-  thing();
-}, /regex.*?/);
+func(
+  () => {
+    thing();
+  },
+  /regex.*?/,
+);
 
 func(
   () => {
     thing();
   },
-  1 ? 2 : 3
+  1 ? 2 : 3,
 );
 
 func(
   function () {
     return thing();
   },
-  1 ? 2 : 3
+  1 ? 2 : 3,
 );
 
 func(
   () => {
     thing();
   },
-  something() ? someOtherThing() : somethingElse(true, 0)
+  something() ? someOtherThing() : somethingElse(true, 0),
 );
 
 func(
   () => {
     thing();
   },
-  something(longArgumentName, anotherLongArgumentName)
-    ? someOtherThing()
-    : somethingElse(true, 0)
+  something(longArgumentName, anotherLongArgumentName) ? someOtherThing() : somethingElse(
+    true,
+    0,
+  ),
 );
 
 func(
   () => {
     thing();
   },
   something(
     longArgumentName,
     anotherLongArgumentName,
     anotherLongArgumentName,
-    anotherLongArgumentName
-  )
-    ? someOtherThing()
-    : somethingElse(true, 0)
+    anotherLongArgumentName,
+  ) ? someOtherThing() : somethingElse(true, 0),
 );
 
 compose(
   (a) => {
     return a.thing;
   },
-  (b) => b * b
+  (b) => b * b,
 );
 
-somthing.reduce(function (item, thing) {
-  return (thing.blah = item);
-}, {});
+somthing.reduce(
+  function (item, thing) {
+    return thing.blah = item;
+  },
+  {},
+);
 
-somthing.reduce(function (item, thing) {
-  return thing.push(item);
-}, []);
+somthing.reduce(
+  function (item, thing) {
+    return thing.push(item);
+  },
+  [],
+);
 
 reallyLongLongLongLongLongLongLongLongLongLongLongLongLongLongMethod(
   (f, g, h) => {
     return f.pop();
   },
-  true
+  true,
 );
 
 // Don't do the rest of these
 
 func(
   function () {
     thing();
   },
   true,
-  false
+  false,
 );
 
 func(
   () => {
     thing();
   },
-  { yes: true, cats: 5 }
+  { yes: true, cats: 5 },
 );
 
 compose(
   (a) => {
     return a.thing;
   },
   (b) => {
     return b + "";
-  }
+  },
 );
 
 compose(
   (a) => {
     return a.thing;
   },
-  (b) => [1, 2, 3, 4, 5]
+  (b) => [1, 2, 3, 4, 5],
 );
 
 renderThing(
   (a) => <div>Content. So much to say. Oh my. Are we done yet?</div>,
-  args
+  args,
 );
 
 setTimeout(
   // Something
   function () {
     thing();
   },
-  500
+  500,
 );
 
 setTimeout(
-  /* blip */ function () {
+  /* blip */
+  function () {
     thing();
   },
-  500
+  500,
 );
 
 func(
   (args) => {
     execute(args);
   },
-  (result) => result && console.log("success")
+  (result) => result && console.log("success"),
 );
```
# js/for-of/async-identifier.js
```diff
 for ((async) of []);
-for (foo of async);
-for (foo of []) async;
+for ((foo) of async);
+for ((foo) of []) async;
 
 async function f() {
   for await (async of []);
-  for await (async of []);
-  for await (foo of async);
-  for await (foo of []) async;
+  for await ((async) of []);
+  for await ((foo) of async);
+  for await ((foo) of []) async;
 }
```
# js/for/comment.js
```diff
-/*a*/
-for (x in y); //b //c
+for (
+  x
+  /*a*/
+  in //b
+  y
+); //c
 
-for (x /*a*/ in y); //b //c
+for (
+  x
+  in /*a*/ //b
+  y
+); //c
 
 for (x /*a*/ in y); //b //c
 
-//a
-for (x in y);
+for (
+  x
+  //a
+  in
+  y
+);
 
-//a
-for (x in y);
+for (
+  x
+  in
+  //a
+  y
+);
 
-/*a*/
-for (x of y); //b //c
+for (
+  x
+  /*a*/
+  of //b
+  y
+); //c
 
-for (x /*a*/ of y); //b //c
+for (
+  x
+  of /*a*/ //b
+  y
+); //c
 
 for (x /*a*/ of y); //b //c
 
-//a
-for (x of y);
+for (
+  x
+  //a
+  of
+  y
+);
 
-//a
-for (x of y);
+for (
+  x
+  of
+  //a
+  y
+);
```
# js/for/continue-and-break-comment-1.js
```diff
 for (;;) {
   continue; // comment
 }
 
 for (;;) {
   break; // comment
 }
 
 for (const f of []) {
   continue; // comment
 }
 
 for (const f of []) {
   break; // comment
 }
 
 for (const f in {}) {
   continue; // comment
 }
 
 for (const f in {}) {
   break; // comment
 }
 
 while (true) {
   continue; // comment
 }
 
 while (true) {
   break; // comment
 }
 
 do {
   continue; // comment
 } while (true);
 
 do {
   break; // comment
 } while (true);
 
 label1: for (;;) {
   continue label1; // comment
 }
 
 label2: {
   break label2; // comment
 }
 
 for (;;) {
   continue; /* comment */
 }
 
 for (;;) {
   break; /* comment */
 }
 
 for (const f of []) {
   continue; /* comment */
 }
 
 for (const f of []) {
   break; /* comment */
 }
 
 for (const f in {}) {
   continue; /* comment */
 }
 
 for (const f in {}) {
   break; /* comment */
 }
 
 while (true) {
   continue; /* comment */
 }
 
 while (true) {
   break; /* comment */
 }
 
 do {
   continue; /* comment */
 } while (true);
 
 do {
   break; /* comment */
 } while (true);
 
 label1: for (;;) {
-  continue label1 /* comment */;
+  continue label1; /* comment */
 }
 
 label2: {
-  break label2 /* comment */;
+  break label2; /* comment */
 }
```
# js/for/continue-and-break-comment-2.js
```diff
 for (;;) {
-  continue;
+  continue
   // comment
+  ;
 }
 
 for (;;) {
-  break;
+  break
   // comment
+  ;
 }
 
 for (const f of []) {
-  continue;
+  continue
   // comment
+  ;
 }
 
 for (const f of []) {
-  break;
+  break
   // comment
+  ;
 }
 
 for (const f in {}) {
-  continue;
+  continue
   // comment
+  ;
 }
 
 for (const f in {}) {
-  break;
+  break
   // comment
+  ;
 }
 
 while (true) {
-  continue;
+  continue
   // comment
+  ;
 }
 
 while (true) {
-  break;
+  break
   // comment
+  ;
 }
 
 do {
-  continue;
+  continue
   // comment
+  ;
 } while (true);
 
 do {
-  break;
+  break
   // comment
+  ;
 } while (true);
 
 label1: for (;;) {
-  continue label1;
+  continue label1
   // comment
+  ;
 }
 
 label2: {
-  break label2;
+  break label2
   // comment
+  ;
 }
 
 for (;;) {
-  continue;
+  continue
   /* comment */
+  ;
 }
 
 for (;;) {
-  break;
+  break
   /* comment */
+  ;
 }
 
 for (const f of []) {
-  continue;
+  continue
   /* comment */
+  ;
 }
 
 for (const f of []) {
-  break;
+  break
   /* comment */
+  ;
 }
 
 for (const f in {}) {
-  continue;
+  continue
   /* comment */
+  ;
 }
 
 for (const f in {}) {
-  break;
+  break
   /* comment */
+  ;
 }
 
 while (true) {
-  continue;
+  continue
   /* comment */
+  ;
 }
 
 while (true) {
-  break;
+  break
   /* comment */
+  ;
 }
 
 do {
-  continue;
+  continue
   /* comment */
+  ;
 } while (true);
 
 do {
-  break;
+  break
   /* comment */
+  ;
 } while (true);
 
 label1: for (;;) {
-  continue label1;
+  continue label1
   /* comment */
+  ;
 }
 
 label2: {
-  break label2;
+  break label2
   /* comment */
+  ;
 }
```
# js/for/continue-and-break-comment-without-blocks.js
```diff
-for (;;)
-  continue;
-  // comment
+for (;;) continue
+// comment
+;
 
-for (;;)
-  break;
-  // comment
+for (;;) break
+// comment
+;
 
-for (const f of [])
-  continue;
-  // comment
+for (const f of []) continue
+// comment
+;
 
-for (const f of [])
-  break;
-  // comment
+for (const f of []) break
+// comment
+;
 
-for (const f in {})
-  continue;
-  // comment
+for (const f in {}) continue
+// comment
+;
 
-for (const f in {})
-  break;
-  // comment
+for (const f in {}) break
+// comment
+;
 
-for (;;)
-  continue; // comment
+for (;;) continue; // comment
 
-for (;;)
-  break; // comment
+for (;;) break; // comment
 
-for (const f of [])
-  continue; // comment
+for (const f of []) continue; // comment
 
-for (const f of [])
-  break; // comment
+for (const f of []) break; // comment
 
-for (const f in {})
-  continue; // comment
+for (const f in {}) continue; // comment
 
-for (const f in {})
-  break; // comment
+for (const f in {}) break; // comment
 
-for (;;) continue; /* comment */
+for (;;) continue /* comment */ ;
 
-for (;;) break; /* comment */
+for (;;) break /* comment */ ;
 
 for (const f of []) continue; /* comment */
 
 for (const f of []) break; /* comment */
 
 for (const f in {}) continue; /* comment */
 
 for (const f in {}) break; /* comment */
 
-for (;;)
-  continue;
-  /* comment */
+for (;;) continue
+/* comment */
+;
 
-for (;;)
-  break;
-  /* comment */
+for (;;) break
+/* comment */
+;
 
-for (const f of [])
-  continue;
-  /* comment */
+for (const f of []) continue
+/* comment */
+;
 
-for (const f of [])
-  break;
-  /* comment */
+for (const f of []) break
+/* comment */
+;
 
-for (const f in {})
-  continue;
-  /* comment */
+for (const f in {}) continue
+/* comment */
+;
 
-for (const f in {})
-  break;
-  /* comment */
+for (const f in {}) break
+/* comment */
+;
 
-label1: for (;;) continue label1 /* comment */;
+label1: for (;;) continue label1 /* comment */ ;
 
-label1: for (;;)
-  continue label1;
-  /* comment */
+label1: for (;;) continue label1
+/* comment */
+;
 
-label1: for (;;)
-  continue label1; // comment
+label1: for (;;) continue label1; // comment
 
-label1: for (;;)
-  continue label1;
-  // comment
+label1: for (;;) continue label1
+// comment
+;
```
# js/function-first-param/function_expression.js
```diff
 //https://github.com/prettier/prettier/issues/3002
-beep.boop().baz(
-  "foo",
-  {
-    some: {
-      thing: {
-        nested: true,
-      },
-    },
-  },
-  { another: { thing: true } },
-  () => {}
-);
+beep
+  .boop()
+  .baz(
+    "foo",
+    { some: { thing: { nested: true } } },
+    { another: { thing: true } },
+    () => {},
+  );
 
 //https://github.com/prettier/prettier/issues/2984
-db.collection("indexOptionDefault").createIndex(
-  { a: 1 },
-  {
-    indexOptionDefaults: true,
-    w: 2,
-    wtimeout: 1000,
-  },
-  function (err) {
-    test.equal(null, err);
-    test.deepEqual({ w: 2, wtimeout: 1000 }, commandResult.writeConcern);
+db
+  .collection("indexOptionDefault")
+  .createIndex(
+    { a: 1 },
+    { indexOptionDefaults: true, w: 2, wtimeout: 1000 },
+    function (err) {
+      test.equal(null, err);
+      test.deepEqual({ w: 2, wtimeout: 1000 }, commandResult.writeConcern);
 
-    client.close();
-    done();
-  }
-);
+      client.close();
+      done();
+    },
+  );
```
# js/function-single-destructuring/array.js
```diff
-function excludeFirstFiveResults([
-  first,
-  second,
-  third,
-  fourth,
-  fifth,
-  ...rest
-]) {
+function excludeFirstFiveResults([first, second, third, fourth, fifth, ...rest]) {
   return rest;
 }
 
-function excludeFirstFiveResults2([
-  first,
-  second,
-  third,
-  fourth,
-  fifth,
-  ...rest
-] = DEFAULT_FIVE_RESULTS) {
+function excludeFirstFiveResults2(
+  [first, second, third, fourth, fifth, ...rest] = DEFAULT_FIVE_RESULTS,
+) {
   return rest;
 }
 
 function excludeFirstFiveResults3(
-  [
-    firstResult,
-    secondResult,
-    thirdResult,
-    fourthResult,
-    fifthResult,
-    ...rest
-  ] = [1, 2, 3, 4, 5]
+  [firstResult, secondResult, thirdResult, fourthResult, fifthResult, ...rest] = [
+    1, 2, 3, 4, 5,
+  ],
 ) {
   return rest;
 }
 
-const excludeFirstFiveResults5 = ([
-  first,
-  second,
-  third,
-  fourth,
-  fifth,
-  ...rest
-]) => {
+const excludeFirstFiveResults5 = (
+  [first, second, third, fourth, fifth, ...rest],
+) => {
   return rest;
 };
 
 class A {
-  excludeFirstFiveResults([
-    first,
-    second,
-    third,
-    fourth,
-    fifth,
-    ...restOfResults
-  ]) {
+  excludeFirstFiveResults(
+    [first, second, third, fourth, fifth, ...restOfResults],
+  ) {
     return restOfResults;
   }
 }
 
 promise.then(
-  ([
-    firstResult,
-    secondResult,
-    thirdResult,
-    fourthResult,
-    fifthResult,
-    ...rest
-  ]) => {
+  ([firstResult, secondResult, thirdResult, fourthResult, fifthResult, ...rest]) => {
     return rest;
-  }
+  },
 );
```
# js/function-single-destructuring/object.js
```diff
-function StatelessFunctionalComponent({
-  isActive,
-  onFiltersUpdated,
-  onSelect,
-  onSubmitAndDeselect,
-  onCancel,
-  searchFilters,
-  title,
-  items,
-}) {
+function StatelessFunctionalComponent(
+  {
+    isActive,
+    onFiltersUpdated,
+    onSelect,
+    onSubmitAndDeselect,
+    onCancel,
+    searchFilters,
+    title,
+    items,
+  },
+) {
   return <div />;
 }
 
-function StatelessFunctionalComponent2({
-  isActive = true,
-  onFiltersUpdated = () => null,
-  onSelect = () => null,
-  onSubmitAndDeselect = () => null,
-  onCancel = () => null,
-  searchFilters = null,
-  title = "",
-  items = [],
-} = {}) {
+function StatelessFunctionalComponent2(
+  {
+    isActive = true,
+    onFiltersUpdated = () => null,
+    onSelect = () => null,
+    onSubmitAndDeselect = () => null,
+    onCancel = () => null,
+    searchFilters = null,
+    title = "",
+    items = [],
+  } = {},
+) {
   return <div />;
 }
 
 function StatelessFunctionalComponent3(
   {
     isActive,
     onFiltersUpdated = () => null,
     onSelect = () => null,
     onSubmitAndDeselect = () => null,
     onCancel = () => null,
     searchFilters = null,
     title = "",
     items = [],
-  } = {
-    isActive: true,
-  }
+  } = { isActive: true },
 ) {
   return <div />;
 }
 
 class C {
-  StatelessFunctionalComponent({
-    isActive,
-    onFiltersUpdated,
-    onSelect,
-    onSubmitAndDeselect,
-    onCancel,
-    searchFilters,
-    title,
-    items,
-  }) {
+  StatelessFunctionalComponent(
+    {
+      isActive,
+      onFiltersUpdated,
+      onSelect,
+      onSubmitAndDeselect,
+      onCancel,
+      searchFilters,
+      title,
+      items,
+    },
+  ) {
     return <div />;
   }
 }
```
# js/function/function_expression.js
```diff
-(function () {}.length);
-typeof function () {};
+(function () {}).length;
+typeof (function () {});
 export default (function () {})();
 (function () {})()``;
 (function () {})``;
 new (function () {})();
 (function () {});
 a = function f() {} || b;
 (function () {} && a);
 a + function () {};
-new (function () {})();
+new function () {}();
```
# js/function/issue-10277.js
```diff
 ((fold) => fold)(
-  (fmap) => (algebra) =>
-    function doFold(v) {
-      return algebra(fmap(doFold)(v));
-    }
+  (fmap) => (algebra) => function doFold(v) {
+    return algebra(fmap(doFold)(v));
+  },
 );
```
# js/functional-composition/functional_compose.js
```diff
-compose(
-  sortBy((x) => x),
-  flatten,
-  map((x) => [x, x * 2])
-);
+compose(sortBy((x) => x), flatten, map((x) => [x, x * 2]));
 
-somelib.compose(
-  sortBy((x) => x),
-  flatten,
-  map((x) => [x, x * 2])
-);
+somelib.compose(sortBy((x) => x), flatten, map((x) => [x, x * 2]));
 
-composeFlipped(
-  sortBy((x) => x),
-  flatten,
-  map((x) => [x, x * 2])
-);
+composeFlipped(sortBy((x) => x), flatten, map((x) => [x, x * 2]));
 
-somelib.composeFlipped(
-  sortBy((x) => x),
-  flatten,
-  map((x) => [x, x * 2])
-);
+somelib.composeFlipped(sortBy((x) => x), flatten, map((x) => [x, x * 2]));
 
 // no regression (#4602)
 const hasValue = hasOwnProperty(a, b);
 
-this.compose(
-  sortBy((x) => x),
-  flatten
-);
-this.a.b.c.compose(
-  sortBy((x) => x),
-  flatten
-);
+this.compose(sortBy((x) => x), flatten);
+this.a.b.c.compose(sortBy((x) => x), flatten);
 someObj.someMethod(this.field.compose(a, b));
 
 class A extends B {
   compose() {
-    super.compose(
-      sortBy((x) => x),
-      flatten
-    );
+    super.compose(sortBy((x) => x), flatten);
   }
 }
 
 this.subscriptions.add(
-  this.componentUpdates
-    .pipe(startWith(this.props), distinctUntilChanged(isEqual))
-    .subscribe((props) => {})
+  this.componentUpdates.pipe(
+    startWith(this.props),
+    distinctUntilChanged(isEqual),
+  ).subscribe((props) => {}),
 );
```
# js/functional-composition/gobject_connect.js
```diff
 button.connect("clicked", () => doSomething());
-app.connect("activate", async () => {
-  await data.load();
-  win.show_all();
-});
+app.connect(
+  "activate",
+  async () => {
+    await data.load();
+    win.show_all();
+  },
+);
```
# js/functional-composition/mongo_connect.js
```diff
-MongoClient.connect("mongodb://localhost:27017/posts", (err, db) => {
-  assert.equal(null, err);
-  db.close();
-});
+MongoClient.connect(
+  "mongodb://localhost:27017/posts",
+  (err, db) => {
+    assert.equal(null, err);
+    db.close();
+  },
+);
```
# js/functional-composition/pipe-function-calls-with-comments.js
```diff
 // input with some comments added to avoid reformatting
 
 (() => {
   pipe(
     // add a descriptive comment here
     timelines,
     everyCommitTimestamps,
     A.sort(ordDate),
-    A.head
+    A.head,
   );
 
   pipe(
     // add a descriptive comment here
     serviceEventFromMessage(msg),
     TE.chain(
       flow(
         // add a descriptive comment here
         publishServiceEvent(analytics),
-        TE.mapLeft(nackFromError)
-      )
-    )
+        TE.mapLeft(nackFromError),
+      ),
+    ),
   )()
     .then(messageResponse(logger, msg))
     .catch((err) => {
       logger.error(
         pipe(
           // add a descriptive comment here
           O.fromNullable(err.stack),
-          O.getOrElse(constant(err.message))
-        )
+          O.getOrElse(constant(err.message)),
+        ),
       );
       process.exit(1);
     });
 
   pipe(
     // add a descriptive comment here
     Changelog.timestampOfFirstCommit([[commit]]),
-    O.toUndefined
+    O.toUndefined,
   );
 
   chain(
     flow(
       // add a descriptive comment here
       getUploadUrl,
       E.mapLeft(Errors.unknownError),
-      TE.fromEither
-    )
+      TE.fromEither,
+    ),
   );
 })();
```
# js/functional-composition/pipe-function-calls.js
```diff
 (() => {
   pipe(timelines, everyCommitTimestamps, A.sort(ordDate), A.head);
 
   pipe(
     serviceEventFromMessage(msg),
-    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError)))
+    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError))),
   )()
     .then(messageResponse(logger, msg))
     .catch((err) => {
       logger.error(
-        pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message)))
+        pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message))),
       );
       process.exit(1);
     });
 
   pipe(Changelog.timestampOfFirstCommit([[commit]]), O.toUndefined);
 
   chain(flow(getUploadUrl, E.mapLeft(Errors.unknownError), TE.fromEither));
 })();
```
# js/functional-composition/ramda_compose.js
```diff
 var classyGreeting = (firstName, lastName) =>
   "The name's " + lastName + ", " + firstName + " " + lastName;
 var yellGreeting = R.compose(R.toUpper, classyGreeting);
 yellGreeting("James", "Bond"); //=> "THE NAME'S BOND, JAMES BOND"
 
 R.compose(Math.abs, R.add(1), R.multiply(2))(-4); //=> 7
 
 //  get :: String -> Object -> Maybe *
 var get = R.curry((propName, obj) => Maybe(obj[propName]));
 
 //  getStateCode :: Maybe String -> Maybe String
 var getStateCode = R.composeK(
   R.compose(Maybe.of, R.toUpper),
   get("state"),
   get("address"),
-  get("user")
+  get("user"),
 );
 getStateCode({ user: { address: { state: "ny" } } }); //=> Maybe.Just("NY")
 getStateCode({}); //=> Maybe.Nothing()
 
-var db = {
-  users: {
-    JOE: {
-      name: "Joe",
-      followers: ["STEVE", "SUZY"],
-    },
-  },
-};
+var db = { users: { JOE: { name: "Joe", followers: ["STEVE", "SUZY"] } } };
 
 // We'll pretend to do a db lookup which returns a promise
 var lookupUser = (userId) => Promise.resolve(db.users[userId]);
 var lookupFollowers = (user) => Promise.resolve(user.followers);
 lookupUser("JOE").then(lookupFollowers);
 
 //  followersForUser :: String -> Promise [UserId]
 var followersForUser = R.composeP(lookupFollowers, lookupUser);
-followersForUser("JOE").then((followers) =>
-  console.log("Followers:", followers)
+followersForUser("JOE").then(
+  (followers) => console.log("Followers:", followers),
 );
 // Followers: ["STEVE","SUZY"]
 
 const mapStateToProps = (state) => ({
-  users: R.compose(
-    R.filter(R.propEq("status", "active")),
-    R.values
-  )(state.users),
+  users: R.compose(R.filter(R.propEq("status", "active")), R.values)(
+    state.users,
+  ),
 });
```
# js/functional-composition/ramda_pipe.js
```diff
 var f = R.pipe(Math.pow, R.negate, R.inc);
 
 f(3, 4); // -(3^4) + 1
 
 //  parseJson :: String -> Maybe *
 //  get :: String -> Object -> Maybe *
 
 //  getStateCode :: Maybe String -> Maybe String
 var getStateCode = R.pipeK(
   parseJson,
   get("user"),
   get("address"),
   get("state"),
-  R.compose(Maybe.of, R.toUpper)
+  R.compose(Maybe.of, R.toUpper),
 );
 
 getStateCode('{"user":{"address":{"state":"ny"}}}');
 //=> Just('NY')
 getStateCode("[Invalid JSON]");
 //=> Nothing()
 
 //  followersForUser :: String -> Promise [User]
 var followersForUser = R.pipeP(db.getUserById, db.getFollowers);
```
# js/functional-composition/redux_compose.js
```diff
 import { createStore, applyMiddleware, compose } from "redux";
 import thunk from "redux-thunk";
 import DevTools from "./containers/DevTools";
 import reducer from "../reducers";
 
 const store = createStore(
   reducer,
-  compose(applyMiddleware(thunk), DevTools.instrument())
+  compose(applyMiddleware(thunk), DevTools.instrument()),
 );
```
# js/functional-composition/redux_connect.js
```diff
-const ArtistInput = connect(
-  mapStateToProps,
-  mapDispatchToProps,
-  mergeProps
-)(Component);
+const ArtistInput = connect(mapStateToProps, mapDispatchToProps, mergeProps)(
+  Component,
+);
```
# js/functional-composition/reselect_createselector.js
```diff
 import { createSelector } from "reselect";
 
-const foo = createSelector(getIds, getObjects, (ids, objects) =>
-  ids.map((id) => objects[id])
+const foo = createSelector(
+  getIds,
+  getObjects,
+  (ids, objects) => ids.map((id) => objects[id]),
 );
 
-const bar = createSelector([getIds, getObjects], (ids, objects) =>
-  ids.map((id) => objects[id])
+const bar = createSelector(
+  [getIds, getObjects],
+  (ids, objects) => ids.map((id) => objects[id]),
 );
```
# js/functional-composition/rxjs_pipe.js
```diff
 import { range } from "rxjs/observable/range";
 import { map, filter, scan } from "rxjs/operators";
 
 const source$ = range(0, 10);
 
-source$
-  .pipe(
-    filter((x) => x % 2 === 0),
-    map((x) => x + x),
-    scan((acc, x) => acc + x, 0)
-  )
-  .subscribe((x) => console.log(x));
+source$.pipe(
+  filter((x) => (x % 2) === 0),
+  map((x) => x + x),
+  scan((acc, x) => acc + x, 0),
+).subscribe((x) => console.log(x));
```
# js/if/comment_before_else.js
```diff
 if (cond) {
   stuff;
 } /* comment */ else if (cond) {
   stuff;
 }
 // comment
 else {
   stuff;
 }
 
-if (cond) stuff;
+if (cond) {
+  stuff;
+}
 // comment
-else stuff;
+else {
+  stuff;
+}
```
# js/if/else.js
```diff
 // Both functions below should be formatted exactly the same
 
 function f() {
-  if (position) return { name: pair };
-  else
+  if (position) {
+    return { name: pair };
+  } else {
     return {
       name: pair.substring(0, position),
       value: pair.substring(position + 1),
     };
+  }
 }
 
 function f() {
-  if (position) return { name: pair };
-  else
+  if (position) {
+    return { name: pair };
+  } else {
     return {
       name: pair.substring(0, position),
       value: pair.substring(position + 1),
     };
+  }
 }
```
# js/if/expr_and_same_line_comments.js
```diff
-if (a === 0) doSomething(); // comment A1
-else if (a === 1) doSomethingElse(); // comment B1
-else if (a === 2) doSomethingElse(); // comment C1
+if (a === 0) {
+  doSomething(); // comment A1
+} else if (a === 1) {
+  doSomethingElse(); // comment B1
+} else if (a === 2) {
+  doSomethingElse(); // comment C1
+}
 
-if (a === 0) doSomething(); /* comment A2 */
-else if (a === 1) doSomethingElse(); /* comment B2 */
-else if (a === 2) doSomethingElse(); /* comment C2 */
+if (a === 0) {
+  doSomething(); /* comment A2 */
+} else if (a === 1) {
+  doSomethingElse(); /* comment B2 */
+} else if (a === 2) {
+  doSomethingElse(); /* comment C2 */
+}
 
-if (a === 0) expr; // comment A3
-else if (a === 1) expr; // comment B3
-else if (a === 2) expr; // comment C3
+if (a === 0) {
+  expr; // comment A3
+} else if (a === 1) {
+  expr; // comment B3
+} else if (a === 2) {
+  expr; // comment C3
+}
 
-if (a === 0) expr; /* comment A4 */
-else if (a === 1) expr; /* comment B4 */
-else if (a === 2) expr; /* comment C4 */
+if (a === 0) {
+  expr; /* comment A4 */
+} else if (a === 1) {
+  expr; /* comment B4 */
+} else if (a === 2) {
+  expr; /* comment C4 */
+}
 
-if (a === 0)
+if (a === 0) {
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment A5
-else if (a === 1)
+} else if (a === 1) {
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment B5
-else if (a === 2)
+} else if (a === 2) {
   looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong; // comment C5
+}
```
# js/if/if_comments.js
```diff
 async function f1() {
-  if (untrackedChoice === 0) {
-    /* Cancel */ return null;
-  } else if (untrackedChoice === 1) {
-    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));
+  if (untrackedChoice === 0) /* Cancel */ {
+    return null;
+  } else if (untrackedChoice === 1) /* Add */ {
+    await repository.addAll(Array.from(untrackedChanges.keys()));
     shouldAmend = true;
-  } else if (untrackedChoice === 2) {
-    /* Allow Untracked */ allowUntracked = true;
+  } else if (untrackedChoice === 2) /* Allow Untracked */ {
+    allowUntracked = true;
   }
 }
 
 async function f2() {
-  if (untrackedChoice === 0) /* Cancel */ null;
-  else if (untrackedChoice === 1) /* Add */ shouldAmend = true;
-  else if (untrackedChoice === 2) /* Allow Untracked */ allowUntracked = true;
+  if (untrackedChoice === 0) /* Cancel */ {
+    null;
+  } else if (untrackedChoice === 1) /* Add */ {
+    shouldAmend = true;
+  } else if (untrackedChoice === 2) /* Allow Untracked */ {
+    allowUntracked = true;
+  }
 }
 
 async function f3() {
-  if (untrackedChoice === 0)
-    /* Cancel */ // Cancel
+  if (untrackedChoice === 0) /* Cancel */ {
+    // Cancel
     null;
-  else if (untrackedChoice === 1)
-    /* Add */ // Add
+  } else if (untrackedChoice === 1) /* Add */ {
+    // Add
     shouldAmend = true;
-  else if (untrackedChoice === 2)
-    /* Allow Untracked */ // Allow Untracked
+  } else if (untrackedChoice === 2) /* Allow Untracked */ {
+    // Allow Untracked
     allowUntracked = true;
+  }
 }
 
 async function f4() {
-  if (untrackedChoice === 0) {
-    /* Cancel */ return null;
-  } else if (untrackedChoice === 1) {
-    /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));
+  if (untrackedChoice === 0)
+  /* Cancel */ {
+    return null;
+  } else if (untrackedChoice === 1)
+  /* Add */ {
+    await repository.addAll(Array.from(untrackedChanges.keys()));
     shouldAmend = true;
-  } else if (untrackedChoice === 2) {
-    /* Allow Untracked */ allowUntracked = true;
+  } else if (untrackedChoice === 2)
+  /* Allow Untracked */ {
+    allowUntracked = true;
   }
 }
 
 async function f5() {
   if (untrackedChoice === 0) {
     /* Cancel */ return null;
   } else if (untrackedChoice === 1) {
     /* Add */ await repository.addAll(Array.from(untrackedChanges.keys()));
     shouldAmend = true;
   } else if (untrackedChoice === 2) {
     /* Allow Untracked */ allowUntracked = true;
   }
 }
```
# js/if/trailing_comment.js
```diff
-if (code === 92 /* '\' */) {
+if (code === 92 /* '\' */ ) {
 }
-if (code === 92 /* '\' */ /* '\' */) {
+if (code === 92 /* '\' */ /* '\' */ ) {
 }
 
-if (code === 92) {
-  /* '\' */
+if (code === 92) /* '\' */ {
 }
 if (code === 92) {
   /* '\' */
 }
 
 if (
   1
   // Comment
 ) {
   a;
 }
```
# js/ignore/ignore-2.js
```diff
 // #8736
 
 function HelloWorld() {
   return (
     <div
       {...{} /*
       // @ts-ignore */ /* prettier-ignore */}
       invalidProp="HelloWorld"
     >
       test
     </div>
   );
 }
 
-a = <div {.../* prettier-ignore */ b} />;
-a = <div {...b /* prettier-ignore */} />;
-a = <div {.../* prettier-ignore */ {}} />;
-a = <div {...{/* prettier-ignore */}} />;
-a = <div {...{} /* prettier-ignore */} />;
+a = <div {... /* prettier-ignore */ b} />;
+a = <div {...b /* prettier-ignore */ } />;
+a = <div {... /* prettier-ignore */ {}} />;
+a = <div {...{ /* prettier-ignore */ }} />;
+a = <div {...{} /* prettier-ignore */ } />;
```
# js/ignore/ignore.js
```diff
 function a() {
   // prettier-ignore
   var fnString =
     '"' + this.USE + ' ' + this.STRICT + '";\n' +
     this.filterPrefix() +
     'var fn=' + this.generateFunction('fn', 's,l,a,i') +
     extra +
     this.watchFns() +
     'return fn;';
 
   // prettier-ignore
   const identity = Matrix.create(
     1, 0, 0,
     0, 1, 0,
     0, 0, 0
   );
 
   // Let's make sure that this comment doesn't interfere
 
   // prettier-ignore
   const commentsWithPrettierIgnore =   {
     "ewww":
             "gross-formatting",
   };
 
   function giveMeSome() {
-    a(  a  ); // prettier-ignore
+    a(a); // prettier-ignore
     // shouldn't I return something?  :shrug:
   }
 
   // prettier-ignore
   console.error(
     'In order to use ' + prompt + ', you need to configure a ' +
     'few environment variables to be able to commit to the ' +
     'repository. Follow those steps to get you setup:\n' +
     '\n' +
     'Go to https://github.com/settings/tokens/new\n' +
     ' - Fill "Token description" with "' + prompt + ' for ' +
       repoSlug + '"\n' +
     ' - Check "public_repo"\n' +
     ' - Press "Generate Token"\n' +
     '\n' +
     'In a different tab, go to https://travis-ci.org/' +
       repoSlug + '/settings\n' +
     ' - Make sure "Build only if .travis.yml is present" is ON\n' +
     ' - Fill "Name" with "GITHUB_USER" and "Value" with the name of the ' +
       'account you generated the token with. Press "Add"\n' +
     '\n' +
     'Once this is done, commit anything to the repository to restart ' +
       'Travis and it should work :)'
   );
 }
 
 const response = {
   // prettier-ignore
   '_text': 'Turn on the lights',
   intent: "lights",
 };
```
# js/ignore/issue-10661.js
```diff
 verylongidentifierthatwillwrap123123123123123(
   a.b
-    // prettier-ignore
-    // Some other comment here
-    .c
+  // prettier-ignore
+  // Some other comment here
+  .c,
 );
 
 call(
   // comment
   a.
-    // prettier-ignore
-    b
+  // prettier-ignore
+    b,
 );
 
 call(
   a(
-/*
+    /*
 this won't get formatted too,
 because the prettier-ignore comment is attached as MemberExpression leading comment
 */
-1,
-2.0000, 3
-)
-    // prettier-ignore
-    .c
+    1,
+    2.0000,
+    3,
+  )
+  // prettier-ignore
+  .c,
 );
```
# js/ignore/semi/directive.js
```diff
 // prettier-ignore
 'use strict';
 [].forEach();
 
 function foo() {
   // prettier-ignore
-  'use strict';
+'use strict';
   [].forEach();
 }
```
# js/import-assertions/empty.js
```diff
 export * as foo from "foo.json";
-export * as bar from "bar.json";
-export * as baz from "baz.json" /* comment */;
+export * as bar from "bar.json" assert {};
+export * as baz from "baz.json" assert { /* comment */ };
 
 import * as foo from "foo.json";
-import * as bar from "bar.json";
-import * as baz from "baz.json" /* comment */;
+import * as bar from "bar.json" assert {};
+import * as baz from "baz.json" assert { /* comment */ };
```
# js/import/comments.js
```diff
 import {
-  //comment1
+  a //comment1
   //comment2
   //comment3
-  a as b,
+  as
+  b,
 } from "";
 
 import {
-  //comment1
+  a
+  as //comment1
   //comment2
   //comment3
-  a as b1,
+  b1,
 } from "";
 
 import {
-  //comment2 //comment1
+  a
+  as //comment2 //comment1
   //comment3
-  a as b2,
+  b2,
 } from "";
 
 import {
-  //comment3 //comment2 //comment1
-  a as b3,
+  a
+  as //comment3 //comment2 //comment1
+  b3,
 } from "";
 
 import {
   // comment 1
   FN1, // comment 2
   /* comment 3 */ FN2,
   // FN3,
-  FN4 /* comment 4 */,
+  FN4, /* comment 4 */
   // FN4,
   // FN5
 } from "./module";
 
 import {
   ExecutionResult,
   DocumentNode,
   /* tslint:disable */
   SelectionSetNode,
   /* tslint:enable */
 } from "graphql";
 
 import x, {
   // comment
   y,
 } from "z";
```
# js/import/inline.js
```diff
 import somethingSuperLongsomethingSuperLong from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
 import { somethingSuperLongsomethingSuperLong1 } from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
-import a, {
-  somethingSuperLongsomethingSuperLong2,
-} from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
-import {
-  a2,
-  somethingSuperLongsomethingSuperLong3,
-} from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
+import a, { somethingSuperLongsomethingSuperLong2 } from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
+import { a2, somethingSuperLongsomethingSuperLong3 } from "somethingSuperLongsomethingSuperLongsomethingSuperLong";
```
# js/in/arrow-function.js
```diff
-const x = () => [].includes(true) || "ontouchend" in document;
+const x = () => [].includes(true) || ("ontouchend" in document);
 
 const y = () => [] in x;
```
# js/label/comment.js
```diff
 {
-  // goto emulation
-  inf_leave: for (;;) {}
+  inf_leave: for (;;) {} // goto emulation
 }
 {
+  inf_leave:
   // goto emulation
-  inf_leave: for (;;) {}
+  for (;;) {}
 }
```
# js/last-argument-expansion/arrow.js
```diff
 export default function searchUsers(action$) {
   return action$
     .ofType(ActionTypes.SEARCHED_USERS)
     .map((action) => action.payload.query)
     .filter((q) => !!q)
-    .switchMap((q) =>
-      Observable.timer(800) // debounce
-        .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
-        .mergeMap(() =>
-          Observable.merge(
-            Observable.of(replace(`?q=${q}`)),
-            ajax
-              .getJSON(`https://api.github.com/search/users?q=${q}`)
-              .map((res) => res.items)
-              .map(receiveUsers)
-          )
-        )
+    .switchMap(
+      (q) =>
+        Observable.timer(800)
+          // debounce
+          .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
+          .mergeMap(
+            () =>
+              Observable.merge(
+                Observable.of(replace(`?q=${q}`)),
+                ajax
+                  .getJSON(`https://api.github.com/search/users?q=${q}`)
+                  .map((res) => res.items)
+                  .map(receiveUsers),
+              ),
+          ),
     );
 }
```
# js/last-argument-expansion/assignment-pattern.js
```diff
 bob.doL(
-  ({
-    a,
-    b = () => {
-      console.log;
+  (
+    {
+      a,
+      b = () => {
+        console.log;
+      },
     },
-  }) => something.else.else({})
+  ) => something.else.else({}),
 );
```
# js/last-argument-expansion/break-parent.js
```diff
 ({
   processors: [
-    require("autoprefixer", {
-      browsers: ["> 1%", "last 2 versions", "ie >= 11", "Firefox ESR"],
-    }),
+    require(
+      "autoprefixer",
+      { browsers: ["> 1%", "last 2 versions", "ie >= 11", "Firefox ESR"] },
+    ),
     require("postcss-url")({
       url: (url) =>
         url.startsWith("/") || /^[a-z]+:/.test(url) ? url : `/static/${url}`,
     }),
   ],
 });
 
-true ? (
-  test({
-    a: 1,
-  })
-) : (
-  <div
-    a={123412342314}
-    b={123412341234}
-    c={123412341234}
-    d={123412341234}
-    e={123412341234}
-    f={123412341234}
-    g={123412341234}
-  />
-);
+true ? test({ a: 1 }) : <div
+  a={123412342314}
+  b={123412341234}
+  c={123412341234}
+  d={123412341234}
+  e={123412341234}
+  f={123412341234}
+  g={123412341234}
+/>;
```
# js/last-argument-expansion/dangling-comment-in-arrow-function.js
```diff
-foo(() =>
-  // foo
-  {}
+foo(
+  (
+    // foo
+  ) => {},
 );
```
# js/last-argument-expansion/edge_case.js
```diff
 a(
   SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong,
   [
     {
       SomethingVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong: 1,
     },
-  ]
+  ],
 );
 
-exports.examples = [
-  {
-    render: withGraphQLQuery(
-      "node(1234567890){image{uri}}",
-      function (container, data) {
-        return (
-          <div>
+exports.examples =
+  [
+    {
+      render: withGraphQLQuery(
+        "node(1234567890){image{uri}}",
+        function (container, data) {
+          return (
+            <div>
             <InlineBlock>
               <img
                 src={data[1234567890].image.uri}
-                style={{
-                  position: "absolute",
-                  top: "0",
-                  left: "0",
-                  zIndex: "-1",
-                }}
+                style={{position: 'absolute', top: '0', left: '0', zIndex:'-1'}}
               />
             </InlineBlock>
           </div>
-        );
-      }
-    ),
-  },
-];
+          );
+        },
+      ),
+    },
+  ];
 
-someReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReally.a(
-  [
-    [],
-    // comment
-    [],
-  ]
-);
+someReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReally.a([
+  [],
+  // comment
+  [],
+]);
 
 (function webpackUniversalModuleDefinition() {})(
   this,
   function (__WEBPACK_EXTERNAL_MODULE_85__, __WEBPACK_EXTERNAL_MODULE_115__) {
-    return /******/ (function (modules) {
+    return (function (modules) {
+      /******/
       // webpackBootstrap
       /******/
-    })(
-      /************************************************************************/
-      /******/ [
+    })
+    /************************************************************************/
+    /******/ (
+      [
         /* 0 */
         /***/ function (module, exports, __webpack_require__) {
           /***/
         },
         /* 1 */
         /***/ function (module, exports, __webpack_require__) {
           /***/
         },
         /* 2 */
         /***/ function (module, exports, __webpack_require__) {
           /***/
         },
         /******/
-      ]
+      ],
     );
-  }
+  },
 );
```
# js/last-argument-expansion/empty-lines.js
```diff
 all_verylongcall_verylongcall_verylongcall_verylongcall_verylongcall(
-  (
-    a,
-
-    b
-  ) => {
+  (a, b) => {
     console.log();
-  }
+  },
 );
```
# js/last-argument-expansion/empty-object.js
```diff
 func(
   first,
   second,
   third,
   fourth,
   fifth,
   aReallyLongArgumentsListToForceItToBreak,
   {
     // comment
-  }
+  },
 );
 
 func({
   // comment
 });
 
 func(
-  {} // comment
+  {}, // comment
 );
 
 func(
-  {}
+  {},
   // comment
 );
 
 func(
   // comment
-  {}
+  {},
 );
```
# js/last-argument-expansion/function-expression-issue-2239.js
```diff
 someFunctionCallWithBigArgumentsAndACallback(
   thisArgumentIsQuiteLong,
   function (cool) {
     return cool;
-  }
+  },
 );
```
# js/last-argument-expansion/function-expression.js
```diff
 function* mySagas() {
   yield effects.takeEvery(
     rexpress.actionTypes.REQUEST_START,
     function* ({ id }) {
       console.log(id);
       yield rexpress.actions(store).writeHead(id, 400);
       yield rexpress.actions(store).end(id, "pong");
       console.log("pong");
-    }
+    },
   );
 }
 
 function mySagas2() {
   return effects.takeEvery(
     rexpress.actionTypes.REQUEST_START,
     function ({ id }) {
       console.log(id);
-    }
+    },
   );
 }
```
# js/last-argument-expansion/issue-10708.js
```diff
 bob.doL(
-  ({
-    a,
-    b: {
-      // comment
+  (
+    {
+      a,
+      b: {
+        // comment
+      },
     },
-  }) => something.else.else({})
+  ) => something.else.else({}),
 );
```
# js/last-argument-expansion/issue-7518.js
```diff
 const Broken = React.forwardRef(
   (
     {
       children,
       // 1
       // 2
       title,
       hidden,
       // 3
     },
-    ref
-  ) => <div ref={ref}>{children}</div>
+    ref,
+  ) => (
+    <div ref={ref}>
+		{children}
+	</div>
+  ),
 );
```
# js/last-argument-expansion/jsx.js
```diff
-const els = items.map((item) => (
-  <div className="whatever">
+const els = items.map(
+  (item) => (
+    <div className="whatever">
     <span>{children}</span>
   </div>
-));
+  ),
+);
```
# js/last-argument-expansion/number-only-array.js
```diff
-instantiate(game, [
-  transform([-0.7, 0.5, 0]),
-  render_colored_diffuse(
-    game.MaterialDiffuse,
-    game.Meshes["monkey_flat"],
-    [1, 1, 0.3, 1]
-  ),
-]);
+instantiate(
+  game,
+  [
+    transform([-0.7, 0.5, 0]),
+    render_colored_diffuse(
+      game.MaterialDiffuse,
+      game.Meshes["monkey_flat"],
+      [1, 1, 0.3, 1],
+    ),
+  ],
+);
```
# js/last-argument-expansion/object.js
```diff
 const formatData = pipe(
   zip,
-  map(([ref, data]) => ({
-    nodeId: ref.nodeId.toString(),
-    ...attributeFromDataValue(ref.attributeId, data),
-  })),
+  map(
+    ([ref, data]) => ({
+      nodeId: ref.nodeId.toString(),
+      ...attributeFromDataValue(ref.attributeId, data),
+    }),
+  ),
   groupBy(prop("nodeId")),
   map(mergeAll),
-  values
+  values,
 );
 
 export const setProp = (y) => ({
   ...y,
   a: "very, very, very long very, very long text",
 });
 
 export const log = (y) => {
   console.log("very, very, very long very, very long text");
 };
```
# js/line-suffix-boundary/boundary.js
```diff
 `${
   a + // a
   a
 }
 
 ${
   a // comment
 }
 
-${b /* comment */}
+${b /* comment */ }
 
-${/* comment */ c /* comment */}
+${ /* comment */ c /* comment */ }
 
 ${
   // comment
   d //comment
 }
 
 ${
   // $FlowFixMe found when converting React.createClass to ES6
   ExampleStory.getFragment("story")
 }
 `;
 
 <div>
-  {
-    ExampleStory.getFragment("story") // $FlowFixMe found when converting React.createClass to ES6
-  }
+{ExampleStory.getFragment('story') // $FlowFixMe found when converting React.createClass to ES6
+}
 </div>;
```
# js/literal-numeric-separator/test.js
```diff
 1_1;
 1_1.1_1;
 0o1_1;
 0o0_11;
 1.1_0_1e1;
-1.1_0_1e1;
-0.1_1;
+1.1_0_1E1;
+.1_1;
 0x1_1;
-0xa_1;
 0xa_1;
+0xA_1;
 0b01_1;
 0b0_1_1;
```
# js/literal/number.js
```diff
 // parentheses around numeric literal should be preserved
 function test5() {
   return (100).toString();
 }
 
 0;
 1;
 
 0.1;
 1.1;
 
-0.1;
-1;
+.1;
+1.;
 
-0b1;
 0b1;
+0B1;
 0o1;
-0o1;
+0O1;
 0x1;
-0x1;
+0X1;
 
-0x123abcdef456abcdef;
-0x123abcdef456abcdef;
+0x123abcdef456ABCDEF;
+0X123abcdef456ABCDEF;
 0xdeadbeef;
 
-0b111000;
-0b000111;
 0b111000;
 0b000111;
-0o111000;
-0o000111;
+0B111000;
+0B000111;
 0o111000;
 0o000111;
+0O111000;
+0O000111;
 0x111000;
 0x000111;
-0x111000;
-0x000111;
+0X111000;
+0X000111;
 
 1e1;
-1e1;
+1e+1;
 1e-1;
-1e1;
-0.1e1;
+1.e1;
+.1e1;
 1.1e1;
-1.1e10;
-0.1e10;
-0.1e-10;
+1.1e0010;
+.1e+0010;
+.1e-0010;
 
-1e1;
-1e1;
-1e-1;
-1e1;
-0.1e1;
-1.1e1;
-1.1e10;
-0.1e10;
-0.1e-10;
+1E1;
+1E+1;
+1E-1;
+1.E1;
+.1E1;
+1.1E1;
+1.1E0010;
+.1E+0010;
+.1E-0010;
 
-0.5;
-0.5;
-0.5;
-0.5;
-0.5;
-0.5;
+0.5e0;
+0.5e00;
+0.5e+0;
+0.5e+00;
+0.5e-0;
+0.5e-00;
 
 1;
-1.005;
+1.00500;
 1.0;
-1.5;
 1.5;
+1.50;
 0;
-0.005;
+0.00500;
 0.0;
-0.0;
-0.0;
-500600.001230045;
-1.005e60;
+0.0000;
+.0;
+500600.001230045000;
+1.00500e60;
 1.0e60;
-0.005e60;
+0.00500e60;
 0.0e60;
-0.0e60;
-0.0e60;
-0e60;
+0.0000e60;
+.0e60;
+0.e60;
 0e60;
-500600.001230045e60;
+500600.001230045000e60;
 10;
 9700;
 10e100;
```
# js/logical_expressions/issue-7024.js
```diff
-const radioSelectedAttr =
-  (isAnyValueSelected &&
-    node.getAttribute(radioAttr.toLowerCase()) === radioValue) ||
-  (!isAnyValueSelected && values[a].default === true) ||
-  a === 0;
+const radioSelectedAttr = (
+  isAnyValueSelected && node.getAttribute(radioAttr.toLowerCase()) === radioValue
+) || ((!isAnyValueSelected && values[a].default === true) || a === 0);
```
# js/logical_expressions/logical_expression_operators.js
```diff
 // Same operators do not require parens
-foo && bar && baz;
-foo && bar && baz;
-foo && bar && baz && qux;
-foo && bar && baz && qux;
-foo && bar && baz && qux && xyz;
-foo && bar && baz && qux && xyz;
+(foo && bar) && baz;
+foo && (bar && baz);
+foo && ((bar && baz) && qux);
+foo && (bar && (baz && qux));
+foo && (bar && ((baz && qux) && xyz));
+foo && (bar && (baz && (qux && xyz)));
 
-foo || bar || baz;
-foo || bar || baz;
-foo || bar || baz || qux;
-foo || bar || baz || qux;
-foo || bar || baz || qux || xyz;
-foo || bar || baz || qux || xyz;
+(foo || bar) || baz;
+foo || (bar || baz);
+foo || ((bar || baz) || qux);
+foo || (bar || (baz || qux));
+foo || (bar || ((baz || qux) || xyz));
+foo || (bar || (baz || (qux || xyz)));
 
-foo ?? bar ?? baz;
-foo ?? bar ?? baz;
-foo ?? bar ?? baz ?? qux;
-foo ?? bar ?? baz ?? qux;
-foo ?? bar ?? baz ?? qux ?? xyz;
-foo ?? bar ?? baz ?? qux ?? xyz;
+(foo ?? bar) ?? baz;
+foo ?? (bar ?? baz);
+foo ?? ((bar ?? baz) ?? qux);
+foo ?? (bar ?? (baz ?? qux));
+foo ?? (bar ?? ((baz ?? qux) ?? xyz));
+foo ?? (bar ?? (baz ?? (qux ?? xyz)));
 
 // Explicitly parenthesized && and || requires parens
 (foo && bar) || baz;
 (foo || bar) && baz;
 
 foo && (bar || baz);
 foo || (bar && baz);
 
 // Implicitly parenthesized && and || requires parens
 (foo && bar) || baz;
 foo || (bar && baz);
```
# js/member/conditional.js
```diff
-(valid
-  ? helper.responseBody(this.currentUser)
-  : helper.responseBody(this.defaultUser)
+(
+  valid ? helper.responseBody(this.currentUser) : helper.responseBody(
+    this.defaultUser,
+  )
 ).prop;
```
# js/member/expand.js
```diff
-const veryVeryVeryVeryVeryVeryVeryLong =
-  doc.expandedStates[doc.expandedStates.length - 1];
+const veryVeryVeryVeryVeryVeryVeryLong = doc.expandedStates[
+  doc.expandedStates.length - 1
+];
 const small = doc.expandedStates[doc.expandedStates.length - 1];
 
 const promises = [
   promise
     .resolve()
     .then(console.log)
-    .catch((err) => {
-      console.log(err);
-      return null;
-    }),
+    .catch(
+      (err) => {
+        console.log(err);
+        return null;
+      },
+    ),
   redis.fetch(),
   other.fetch(),
 ];
 
 const promises2 = [
   promise
     .resolve()
     .veryLongFunctionCall()
     .veryLongFunctionCall()
     .then(console.log)
-    .catch((err) => {
-      console.log(err);
-      return null;
-    }),
+    .catch(
+      (err) => {
+        console.log(err);
+        return null;
+      },
+    ),
   redis.fetch(),
   other.fetch(),
 ];
 
 window.FooClient.setVars({
   locale: getFooLocale({ page }),
   authorizationToken: data.token,
 }).initVerify("foo_container");
 
 window.something.FooClient.setVars({
   locale: getFooLocale({ page }),
   authorizationToken: data.token,
 }).initVerify("foo_container");
 
-window.FooClient.something
-  .setVars({
-    locale: getFooLocale({ page }),
-    authorizationToken: data.token,
-  })
-  .initVerify("foo_container");
+window.FooClient.something.setVars({
+  locale: getFooLocale({ page }),
+  authorizationToken: data.token,
+}).initVerify("foo_container");
```
# js/member/logical.js
```diff
 (veryLongVeryLongVeryLong || e).prop;
 
 (
   veryLongVeryLongVeryLong ||
-  anotherVeryLongVeryLongVeryLong ||
-  veryVeryVeryLongError
+    anotherVeryLongVeryLongVeryLong ||
+    veryVeryVeryLongError
 ).prop;
```
# js/method-chain/bracket_0-1.js
```diff
-const thingamabobMetaAlias = path.scope
-  .getProgramParent()
-  .path.get("body")[0].node;
+const thingamabobMetaAlias = path.scope.getProgramParent().path.get("body")[0].node;
```
# js/method-chain/bracket_0.js
```diff
 function a() {
   function b() {
     queryThenMutateDOM(() => {
-      title = SomeThing.call(
-        root,
-        "someLongStringThatPushesThisTextReallyFar"
-      )[0];
+      title =
+        SomeThing.call(root, "someLongStringThatPushesThisTextReallyFar")[0];
     });
   }
 }
```
# js/method-chain/break-last-call.js
```diff
 export default (store) => {
   return callApi(endpoint, schema).then(
-    (response) =>
-      next(
-        actionWith({
-          response,
-          type: successType,
-        })
-      ),
+    (response) => next(actionWith({ response, type: successType })),
     (error) =>
       next(
         actionWith({
           type: failureType,
           error: error.message || "Something bad happened",
-        })
-      )
+        }),
+      ),
   );
 };
 
-it("should group messages with same created time", () => {
-  expect(groupMessages(messages).toJS()).toEqual({
-    "11/01/2017 13:36": [
-      {
-        message: "test",
-        messageType: "SMS",
-        status: "Unknown",
-        created: "11/01/2017 13:36",
-      },
-      {
-        message: "test",
-        messageType: "Email",
-        status: "Unknown",
-        created: "11/01/2017 13:36",
-      },
-    ],
-    "09/01/2017 17:25": [
-      {
-        message: "te",
-        messageType: "SMS",
-        status: "Unknown",
-        created: "09/01/2017 17:25",
-      },
-      {
-        message: "te",
-        messageType: "Email",
-        status: "Unknown",
-        created: "09/01/2017 17:25",
-      },
-    ],
-    "11/01/2017 13:33": [
-      {
-        message: "test",
-        messageType: "SMS",
-        status: "Unknown",
-        created: "11/01/2017 13:33",
-      },
-      {
-        message: "test",
-        messageType: "Email",
-        status: "Unknown",
-        created: "11/01/2017 13:33",
-      },
-    ],
-    "11/01/2017 13:37": [
-      {
-        message: "test",
-        messageType: "SMS",
-        status: "Unknown",
-        created: "11/01/2017 13:37",
-      },
-      {
-        message: "test",
-        messageType: "Email",
-        status: "Unknown",
-        created: "11/01/2017 13:37",
-      },
-    ],
-  });
-});
+it(
+  "should group messages with same created time",
+  () => {
+    expect(groupMessages(messages).toJS()).toEqual({
+      "11/01/2017 13:36": [
+        {
+          message: "test",
+          messageType: "SMS",
+          status: "Unknown",
+          created: "11/01/2017 13:36",
+        },
+        {
+          message: "test",
+          messageType: "Email",
+          status: "Unknown",
+          created: "11/01/2017 13:36",
+        },
+      ],
+      "09/01/2017 17:25": [
+        {
+          message: "te",
+          messageType: "SMS",
+          status: "Unknown",
+          created: "09/01/2017 17:25",
+        },
+        {
+          message: "te",
+          messageType: "Email",
+          status: "Unknown",
+          created: "09/01/2017 17:25",
+        },
+      ],
+      "11/01/2017 13:33": [
+        {
+          message: "test",
+          messageType: "SMS",
+          status: "Unknown",
+          created: "11/01/2017 13:33",
+        },
+        {
+          message: "test",
+          messageType: "Email",
+          status: "Unknown",
+          created: "11/01/2017 13:33",
+        },
+      ],
+      "11/01/2017 13:37": [
+        {
+          message: "test",
+          messageType: "SMS",
+          status: "Unknown",
+          created: "11/01/2017 13:37",
+        },
+        {
+          message: "test",
+          messageType: "Email",
+          status: "Unknown",
+          created: "11/01/2017 13:37",
+        },
+      ],
+    });
+  },
+);
```
# js/method-chain/break-last-member.js
```diff
-SomeVeryLongUpperCaseConstant.someVeryLongCallExpression()
-  .some_very_long_member_expression;
-weNeedToReachTheEightyCharacterLimitXXXXXXXXXXXXXXXXX.someNode
-  .childrenInAnArray[0];
+SomeVeryLongUpperCaseConstant.someVeryLongCallExpression().some_very_long_member_expression;
+weNeedToReachTheEightyCharacterLimitXXXXXXXXXXXXXXXXX.someNode.childrenInAnArray[
+  0
+];
 superSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered;
-superSupersuperSupersuperSupersuperSupersuperSuperLong
-  .exampleOfOrderOfGetterAndSetterReordered[0];
+superSupersuperSupersuperSupersuperSupersuperSuperLong.exampleOfOrderOfGetterAndSetterReordered[
+  0
+];
 
 expect(
-  findDOMNode(component.instance()).getElementsByClassName(styles.inner)[0]
-    .style.paddingRight
+  findDOMNode(component.instance()).getElementsByClassName(styles.inner)[0].style.paddingRight,
 ).toBe("1000px");
 
-const {
-  course,
-  conflicts = [],
-  index,
-  scheduleId,
-  studentId,
-  something,
-} = a.this.props;
+const { course, conflicts = [], index, scheduleId, studentId, something } = a.this.props;
 
-const {
-  course2,
-  conflicts2 = [],
-  index2,
-  scheduleId2,
-  studentId2,
-  something2,
-} = this.props;
+const { course2, conflicts2 = [], index2, scheduleId2, studentId2, something2 } = this.props;
 
 const {
   updated,
   author: { identifier: ownerId },
   location,
   category: categories,
 } = rawAd.entry;
```
# js/method-chain/comment.js
```diff
 function f() {
-  return (
-    observableFromSubscribeFunction()
-      // Debounce manually rather than using editor.onDidStopChanging so that the debounce time is
-      // configurable.
-      .debounceTime(debounceInterval)
-  );
+  return observableFromSubscribeFunction()
+    // Debounce manually rather than using editor.onDidStopChanging so that the debounce time is
+    // configurable.
+    .debounceTime(debounceInterval);
 }
 
 _.a(a)
   /* very very very very very very very long such that it is longer than 80 columns */
   .a();
 
-_.a(
-  a
-) /* very very very very very very very long such that it is longer than 80 columns */
+_.a(a)
+  /* very very very very very very very long such that it is longer than 80 columns */
   .a();
 
-_.a(
-  a
-) /* very very very very very very very long such that it is longer than 80 columns */
+_.a(a)
+  /* very very very very very very very long such that it is longer than 80 columns */
   .a();
 
 Something
-  // $FlowFixMe(>=0.41.0)
-  .getInstance(this.props.dao)
-  .getters();
+// $FlowFixMe(>=0.41.0)
+.getInstance(this.props.dao).getters();
 
 // Warm-up first
-measure().then(() => {
-  SomethingLong();
-});
+measure()
+  .then(() => {
+    SomethingLong();
+  });
 
-measure() // Warm-up first
+measure()
+  // Warm-up first
   .then(() => {
     SomethingLong();
   });
 
 const configModel = this.baseConfigurationService
   .getCache()
   .consolidated // global/default values (do NOT modify)
   .merge(this.cachedWorkspaceConfig);
 
-this.doWriteConfiguration(target, value, options) // queue up writes to prevent race conditions
+this.doWriteConfiguration(target, value, options)
+  // queue up writes to prevent race conditions
   .then(
     () => null,
     (error) => {
-      return options.donotNotifyError
-        ? TPromise.wrapError(error)
-        : this.onError(error, target, value);
-    }
+      return options.donotNotifyError ? TPromise.wrapError(error) : this.onError(
+        error,
+        target,
+        value,
+      );
+    },
   );
 
-ret = __DEV__
-  ? // $FlowFixMe: this type differs according to the env
-    vm.runInContext(source, ctx)
-  : a;
+ret =
+  __DEV__ ?
+  // $FlowFixMe: this type differs according to the env
+  vm.runInContext(source, ctx) : a;
 
 angular
   .module("AngularAppModule")
   // Hello, I am comment.
   .constant("API_URL", "http://localhost:8080/api");
```
# js/method-chain/complex-args.js
```diff
 client.execute(
-  Post.selectAll().where(Post.id.eq(42)).where(Post.published.eq(true))
+  Post.selectAll()
+    .where(Post.id.eq(42))
+    .where(Post.published.eq(true)),
 );
```
# js/method-chain/computed-merge.js
```diff
-[].forEach((key) => {
-  data[key]("foo")
-    .then(() => console.log("bar"))
-    .catch(() => console.log("baz"));
-});
+[].forEach(
+  (key) => {
+    data[key]("foo")
+      .then(() => console.log("bar"))
+      .catch(() => console.log("baz"));
+  },
+);
 
-[].forEach((key) => {
-  data("foo")
-    [key]("bar")
-    .then(() => console.log("bar"))
-    .catch(() => console.log("baz"));
-});
+[].forEach(
+  (key) => {
+    data("foo")[key]("bar")
+      .then(() => console.log("bar"))
+      .catch(() => console.log("baz"));
+  },
+);
 
 window.Data[key]("foo")
   .then(() => a)
   .catch(() => b);
```
# js/method-chain/computed.js
```diff
 nock(/test/)
-  .matchHeader("Accept", "application/json")
-  [httpMethodNock(method)]("/foo")
-  .reply(200, {
-    foo: "bar",
-  });
+  .matchHeader("Accept", "application/json")[httpMethodNock(method)]("/foo")
+  .reply(200, { foo: "bar" });
```
# js/method-chain/conditional.js
```diff
 (a ? b : c).d();
 
 (a ? b : c).d().e();
 
 (a ? b : c).d().e().f();
 
-(valid
-  ? helper.responseBody(this.currentUser)
-  : helper.responseBody(this.defaultUser)
+(
+  valid ? helper.responseBody(this.currentUser) : helper.responseBody(
+    this.defaultUser,
+  )
 ).map();
 
-(valid
-  ? helper.responseBody(this.currentUser)
-  : helper.responseBody(this.defaultUser)
-)
-  .map()
-  .filter();
+(
+  valid ? helper.responseBody(this.currentUser) : helper.responseBody(
+    this.defaultUser,
+  )
+).map().filter();
 
-(valid
-  ? helper.responseBody(this.currentUser)
-  : helper.responseBody(defaultUser)
+(
+  valid ? helper.responseBody(this.currentUser) : helper.responseBody(
+    defaultUser,
+  )
 ).map();
 
-object[
-  valid
-    ? helper.responseBody(this.currentUser)
-    : helper.responseBody(defaultUser)
-].map();
+object[valid ? helper.responseBody(this.currentUser) : helper.responseBody(
+  defaultUser,
+)].map();
```
# js/method-chain/d3.js
```diff
-d3.select("body")
+d3
+  .select("body")
   .append("circle")
   .at({ width: 30, fill: "#f0f" })
   .st({ fontWeight: 600 });
 
-const myScale = d3.scaleLinear().domain([1950, 1980]).range([0, width]);
+const myScale = d3
+  .scaleLinear()
+  .domain([1950, 1980])
+  .range([0, width]);
 
 not.d3
   .select("body")
   .append("circle")
   .at({ width: 30, fill: "#f0f" })
   .st({ fontWeight: 600 });
 
-not.d3.scaleLinear().domain([1950, 1980]).range([0, width]);
+not.d3
+  .scaleLinear()
+  .domain([1950, 1980])
+  .range([0, width]);
```
# js/method-chain/first_long.js
```diff
 export default function theFunction(action$, store) {
-  return action$.ofType(THE_ACTION).switchMap((action) =>
-    Observable.webSocket({
-      url: THE_URL,
-      more: stuff(),
-      evenMore: stuff({
-        value1: true,
-        value2: false,
-        value3: false,
-      }),
-    })
-      .filter((data) => theFilter(data))
-      .map(({ theType, ...data }) => theMap(theType, data))
-      .retryWhen((errors) => errors)
-  );
+  return action$
+    .ofType(THE_ACTION)
+    .switchMap(
+      (action) =>
+        Observable.webSocket({
+          url: THE_URL,
+          more: stuff(),
+          evenMore: stuff({ value1: true, value2: false, value3: false }),
+        })
+          .filter((data) => theFilter(data))
+          .map(({ theType, ...data }) => theMap(theType, data))
+          .retryWhen((errors) => errors),
+    );
 }
 
 function f() {
   return this._getWorker(workerOptions)({
     filePath,
     hasteImplModulePath: this._options.hasteImplModulePath,
-  }).then((metadata) => {
-    // `1` for truthy values instead of `true` to save cache space.
-    fileMetadata[H.VISITED] = 1;
-    const metadataId = metadata.id;
-    const metadataModule = metadata.module;
-    if (metadataId && metadataModule) {
-      fileMetadata[H.ID] = metadataId;
-      setModule(metadataId, metadataModule);
-    }
-    fileMetadata[H.DEPENDENCIES] = metadata.dependencies || [];
-  });
+  })
+    .then(
+      (metadata) => {
+        // `1` for truthy values instead of `true` to save cache space.
+        fileMetadata[H.VISITED] = 1;
+        const metadataId = metadata.id;
+        const metadataModule = metadata.module;
+        if (metadataId && metadataModule) {
+          fileMetadata[H.ID] = metadataId;
+          setModule(metadataId, metadataModule);
+        }
+        fileMetadata[H.DEPENDENCIES] = metadata.dependencies || [];
+      },
+    );
 }
```
# js/method-chain/fluent-configuration.js
```diff
 domain.concept("Page").val("title", "string").vals("widgets", "Widget");
-domain
-  .concept("Widget")
-  .val("title", "string")
-  .val("color", "Color")
-  .val("foo", "Foo")
-  .val("bar", "Bar");
+domain.concept("Widget").val("title", "string").val("color", "Color").val(
+  "foo",
+  "Foo",
+).val("bar", "Bar");
 domain.concept("Widget").val("title", "string").val("color", "Color");
 domain.concept(CONCEPT_NAME).val("title").vals();
```
# js/method-chain/inline_merge.js
```diff
-Object.keys(
-  availableLocales({
-    test: true,
-  })
-).forEach((locale) => {
-  // ...
-});
+Object.keys(availableLocales({ test: true }))
+  .forEach(
+    (locale) => {
+      // ...
+    },
+  );
 
-this.layoutPartsToHide = this.utils.hashset(
-  _.flatMap(this.visibilityHandlers, (fn) => fn())
-    .concat(this.record.resolved_legacy_visrules)
-    .filter(Boolean)
-);
+this.layoutPartsToHide =
+  this.utils.hashset(
+    _.flatMap(this.visibilityHandlers, (fn) => fn())
+      .concat(this.record.resolved_legacy_visrules)
+      .filter(Boolean),
+  );
 
 var jqxhr = $.ajax("example.php").done(doneFn).fail(failFn);
```
# js/method-chain/issue-11298.js
```diff
-foo1(/𠮟𠮟𠮟/)
-  .foo2(bar)
-  .foo3(baz);
+foo1(/𠮟𠮟𠮟/).foo2(bar).foo3(baz);
 
-foo1(/叱叱叱/)
-  .foo2(bar)
-  .foo3(baz);
+foo1(/叱叱叱/).foo2(bar).foo3(baz);
```
# js/method-chain/issue-3594.js
```diff
 const fetched = fetch("/foo");
-fetched
-  .then((response) => response.json())
-  .then((json) => processThings(json.data.things));
+fetched.then((response) => response.json()).then(
+  (json) => processThings(json.data.things),
+);
 
-let column = new Column(null, conn).table(data.table).json(data.column);
+let column = new Column(null, conn)
+  .table(data.table)
+  .json(data.column);
```
# js/method-chain/issue-4125.js
```diff
 // examples from https://github.com/prettier/prettier/issues/4125
 
 const sha256 = (data) => crypto.createHash("sha256").update(data).digest("hex");
 
 req.checkBody("id").isInt().optional();
 req.checkBody("name").notEmpty().optional();
 
 const x = moment().add(1, "day").valueOf();
 
 // should stay on one line:
 const y = obj.foo(1).foo(2).foo(3);
 const z = obj.foo(-1).foo(import("2")).foo(!x).check(/[A-Z]/);
 
 // better on multiple lines:
 somePromise
   .then(format)
   .then((val) => doSomething(val))
   .catch((err) => handleError(err));
 
 // you can still force multi-line chaining with a comment:
 const sha256_2 = (data) =>
-  crypto // breakme
+  crypto
+    // breakme
     .createHash("sha256")
     .update(data)
     .digest("hex");
 
 // examples from https://github.com/prettier/prettier/pull/4765
 
 if ($(el).attr("href").includes("/wiki/")) {
 }
 
 if ($(el).attr("href").includes("/wiki/")) {
   if ($(el).attr("xyz").includes("/whatever/")) {
     if ($(el).attr("hello").includes("/world/")) {
     }
   }
 }
 
 const parseNumbers = (s) => s.split("").map(Number).sort();
 
 function palindrome(a, b) {
   return a.slice().reverse().join(",") === b.slice().sort().join(",");
 }
 
 // examples from https://github.com/prettier/prettier/issues/1565
 
-d3.select("body")
+d3
+  .select("body")
   .selectAll("p")
   .data([1, 2, 3])
   .enter()
   .style("color", "white");
 
 Object.keys(props)
-  .filter((key) => key in own === false)
-  .reduce((a, key) => {
-    a[key] = props[key];
-    return a;
-  }, {});
+  .filter((key) => (key in own) === false)
+  .reduce(
+    (a, key) => {
+      a[key] = props[key];
+      return a;
+    },
+    {},
+  );
 
 point().x(4).y(3).z(6).plot();
 
 assert.equal(this.$().text().trim(), "1000");
 
 something()
   .then(() => doSomethingElse())
   .then((result) => dontForgetThisAsWell(result));
 
 db.branch(
   db.table("users").filter({ email }).count(),
-  db.table("users").filter({ email: "a@b.com" }).count(),
+  db
+    .table("users")
+    .filter({ email: "a@b.com" })
+    .count(),
   db.table("users").insert({ email }),
-  db.table("users").filter({ email })
+  db.table("users").filter({ email }),
 );
 
 sandbox.stub(config, "get").withArgs("env").returns("dev");
 
 const date = moment.utc(userInput).hour(0).minute(0).second(0);
 
 fetchUser(id).then(fetchAccountForUser).catch(handleFetchError);
 
-fetchUser(id) //
+fetchUser(id)
+  //
   .then(fetchAccountForUser)
   .catch(handleFetchError);
 
 // examples from https://github.com/prettier/prettier/issues/3107
 
 function HelloWorld() {
   window.FooClient.setVars({
     locale: getFooLocale({ page }),
     authorizationToken: data.token,
   }).initVerify("foo_container");
 
   fejax
-    .ajax({
-      url: "/verification/",
-      dataType: "json",
-    })
+    .ajax({ url: "/verification/", dataType: "json" })
     .then(
       (data) => {
         this.setState({ isLoading: false });
         this.initWidget(data);
       },
       (data) => {
         this.logImpression("foo_fetch_error", data);
         Flash.error(I18n.t("offline_identity.foo_issue"));
-      }
+      },
     );
 }
 
 action$
   .ofType(ActionTypes.SEARCHED_USERS)
   .map((action) => action.payload.query)
   .filter((q) => !!q)
-  .switchMap((q) =>
-    Observable.timer(800) // debounce
-      .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
-      .mergeMap(() =>
-        Observable.merge(
-          Observable.of(replace(`?q=${q}`)),
-          ajax
-            .getJSON(`https://api.github.com/search/users?q=${q}`)
-            .map((res) => res.items)
-            .map(receiveUsers)
-        )
-      )
+  .switchMap(
+    (q) =>
+      Observable.timer(800)
+        // debounce
+        .takeUntil(action$.ofType(ActionTypes.CLEARED_SEARCH_RESULTS))
+        .mergeMap(
+          () =>
+            Observable.merge(
+              Observable.of(replace(`?q=${q}`)),
+              ajax
+                .getJSON(`https://api.github.com/search/users?q=${q}`)
+                .map((res) => res.items)
+                .map(receiveUsers),
+            ),
+        ),
   );
 
 window.FooClient.setVars({
   locale: getFooLocale({ page }),
   authorizationToken: data.token,
 }).initVerify("foo_container");
 
-it("gets triggered by mouseenter", () => {
-  const wrapper = shallow(<CalendarDay />);
-  wrapper.dive().find(Button).prop();
-});
+it(
+  "gets triggered by mouseenter",
+  () => {
+    const wrapper = shallow(<CalendarDay />);
+    wrapper.dive().find(Button).prop();
+  },
+);
 
 const a1 = x.a(true).b(null).c(123);
 const a2 = x.d("").e(``).f(g);
 const a3 = x.d("").e(`${123}`).f(g);
-const a4 = x.h(i.j).k(l()).m([n, o]);
+const a4 = x
+  .h(i.j)
+  .k(l())
+  .m([n, o]);
 class X {
   y() {
-    const j = x.a(this).b(super.cde()).f(/g/).h(new i()).j();
+    const j = x
+      .a(this)
+      .b(super.cde())
+      .f(/g/)
+      .h(new i())
+      .j();
   }
 }
 
 // should break when call expressions get complex
-x.a()
+x
+  .a()
   .b([c, [d, [e]]])
   .f();
-x.a()
+x
+  .a()
   .b(c(d(e())))
   .f();
-x.a()
+x
+  .a()
   .b(`${c(d())}`)
   .f();
 
 xyz
   .a()
   .b()
   .c(a(a(b(c(d().p).p).p).p));
 
-var l = base
-  .replace(/^\w*:\/\//, "")
-  .replace(/\/$/, "")
-  .split("/").length;
+var l = base.replace(/^\w*:\/\//, "").replace(/\/$/, "").split("/").length;
```
# js/method-chain/logical.js
```diff
 const someLongVariableName = (
   idx(this.props, (props) => props.someLongPropertyName) || []
 ).map((edge) => edge.node);
 
-(veryLongVeryLongVeryLong || e).map((tickets) =>
-  TicketRecord.createFromSomeLongString()
+(veryLongVeryLongVeryLong || e).map(
+  (tickets) => TicketRecord.createFromSomeLongString(),
 );
 
-(veryLongVeryLongVeryLong || e)
-  .map((tickets) => TicketRecord.createFromSomeLongString())
-  .filter((obj) => !!obj);
+(veryLongVeryLongVeryLong || e).map(
+  (tickets) => TicketRecord.createFromSomeLongString(),
+).filter((obj) => !!obj);
 
 (
   veryLongVeryLongVeryLong ||
-  anotherVeryLongVeryLongVeryLong ||
-  veryVeryVeryLongError
+    anotherVeryLongVeryLongVeryLong ||
+    veryVeryVeryLongError
 ).map((tickets) => TicketRecord.createFromSomeLongString());
 
 (
   veryLongVeryLongVeryLong ||
-  anotherVeryLongVeryLongVeryLong ||
-  veryVeryVeryLongError
-)
-  .map((tickets) => TicketRecord.createFromSomeLongString())
-  .filter((obj) => !!obj);
+    anotherVeryLongVeryLongVeryLong ||
+    veryVeryVeryLongError
+).map((tickets) => TicketRecord.createFromSomeLongString()).filter(
+  (obj) => !!obj,
+);
```
# js/method-chain/multiple-members.js
```diff
 if (testConfig.ENABLE_ONLINE_TESTS === "true") {
-  describe("POST /users/me/pet", function () {
-    it("saves pet", function () {
-      function assert(pet) {
-        expect(pet).to.have.property("OwnerAddress").that.deep.equals({
-          AddressLine1: "Alexanderstrasse",
-          AddressLine2: "",
-          PostalCode: "10999",
-          Region: "Berlin",
-          City: "Berlin",
-          Country: "DE",
-        });
-      }
-    });
-  });
+  describe(
+    "POST /users/me/pet",
+    function () {
+      it(
+        "saves pet",
+        function () {
+          function assert(pet) {
+            expect(pet).to.have
+              .property("OwnerAddress")
+              .that.deep.equals({
+                AddressLine1: "Alexanderstrasse",
+                AddressLine2: "",
+                PostalCode: "10999",
+                Region: "Berlin",
+                City: "Berlin",
+                Country: "DE",
+              });
+          }
+        },
+      );
+    },
+  );
 }
 
 wrapper
   .find("SomewhatLongNodeName")
   .prop("longPropFunctionName")()
   .then(function () {
     doSomething();
   });
 
 wrapper
   .find("SomewhatLongNodeName")
   .prop("longPropFunctionName")("argument")
   .then(function () {
     doSomething();
   });
 
 wrapper
   .find("SomewhatLongNodeName")
   .prop(
     "longPropFunctionName",
-    "second argument that pushes this group past 80 characters"
+    "second argument that pushes this group past 80 characters",
   )("argument")
   .then(function () {
     doSomething();
   });
 
 wrapper
   .find("SomewhatLongNodeName")
   .prop("longPropFunctionName")(
     "argument",
-    "second argument that pushes this group past 80 characters"
+    "second argument that pushes this group past 80 characters",
   )
   .then(function () {
     doSomething();
   });
```
# js/method-chain/pr-7889.js
```diff
-const Profile = view.with({ name: (state) => state.name }).as((props) => (
-  <div>
+const Profile = view
+  .with({ name: (state) => state.name })
+  .as(
+    (props) => (
+      <div>
     <h1>Hello, {props.name}</h1>
   </div>
-));
+    ),
+  );
 
-const Profile2 = view.with({ name }).as((props) => (
-  <div>
+const Profile2 = view
+  .with({ name })
+  .as(
+    (props) => (
+      <div>
     <h1>Hello, {props.name}</h1>
   </div>
-));
+    ),
+  );
```
# js/method-chain/square_0.js
```diff
-const version = someLongString
-  .split("jest version =")
-  .pop()
-  .split(EOL)[0]
-  .trim();
+const version = someLongString.split("jest version =").pop().split(EOL)[0].trim();
 
 const component = find(".org-lclp-edit-copy-url-banner__link")[0]
   .getAttribute("href")
   .indexOf(this.landingPageLink);
```
# js/method-chain/test.js
```diff
 method()
-  .then((x) => x)
-  ["abc"]((x) => x)
-  [abc]((x) => x);
+  .then((x) => x)["abc"]((x) => x)[abc]((x) => x);
 
-({}.a().b());
 ({}.a().b());
+({}).a().b();
```
# js/method-chain/this.js
```diff
-const sel = this.connections
-  .concat(this.activities.concat(this.operators))
-  .filter((x) => x.selected);
+const sel = this.connections.concat(this.activities.concat(this.operators)).filter(
+  (x) => x.selected,
+);
```
# js/module-blocks/comments.js
```diff
-const m = /*A1*/ module {
-  /*A2*/ /*A3*/
+const m = /*A1*/ module; /*A2*/
+{
+  /*A3*/
   /*A4*/
   export const foo = "foo";
   export { foo }; /*A5*/
   /*A6*/
-}; /*A7*/ /*A8*/
+} /*A7*/
+/*A8*/
 
-const m2 = module {
-  /* B1 */
+const m2 = module; /* B1 */
+{
   /* B2 */
-};
+}
```
# js/module-blocks/module-blocks.js
```diff
-module {
+module;
+{
   await 3;
-};
+}
 
 class B {
   #p() {
-    module {
+    module;
+    {
       class C {
         [this.#p];
       }
-    };
+    }
   }
 }
 
-const m = module {
+const m = module;
+{
   export const foo = "foo";
   export { foo };
-};
+}
 
-module {
-  export { foo };
-};
+module;
+{
+  export { foo }
+}
 
-const m = module {};
+const m = module;
+{
+}
 
 const worker = new Worker(module {
   export const foo = "foo";
-});
+})
 
-let m = module {
-  module {
+let m = module;
+{
+  module;
+  {
     export let foo = "foo";
-  };
-};
+  }
+}
 
-const m = module {
-  export const foo = "foo";
-};
+const m = module;
+{
+  export const foo = "foo"
+}
 
-let moduleBlock = module {
+let moduleBlock = module;
+{
   export let y = 1;
-};
+}
 
-foo(module {
-  export let foo = "foo";
-});
+foo(module { export let foo = "foo";
+})
 
-let m = module {
+let m = module;
+{
   /* foo */
-};
+}
```
# js/module-blocks/range.js
```diff
-let moduleBlock = module {
-  export let y = 1;
-};
+let moduleBlock = module {  export let y = 1;
+}
 
-foo(module {
-  export let foo = "foo";
-});
+foo(module { export let foo = "foo"; })
```
# js/module-blocks/worker.js
```diff
 let worker = new Worker(module {
-  onmessage = function ({ data }) {
+  onmessage = function({data}) {
     let mod = import(data);
     postMessage(mod.fn());
-  };
-}, { type: "module" });
-
-let worker = new Worker(
-  module {
-    onmessage = function ({ data }) {
-      let mod = import(data);
-      postMessage(mod.fn());
-    };
-  },
-  { type: "module", foo: "bar" }
-);
+  }
+}, {type: "module"});
 
-worker.postMessage(module {
-  export function fn() {
-    return "hello!";
+let worker = new Worker(module {
+  onmessage = function({data}) {
+    let mod = import(data);
+    postMessage(mod.fn());
   }
-});
+}, {type: "module", foo: "bar" });
+
+worker.postMessage(module { export function fn() { return "hello!" } });
```
# js/module-string-names/module-string-names-export.js
```diff
 export { smile as "smile1" } from "./emojis.js";
 export { "smile" as smile2 } from "./emojis.js";
 export { "smile" as "smile3" } from "./emojis.js";
 export { foo1, bar as "foo2" } from "./emojis.js";
-export {
-  "學而時習之，不亦說乎？",
-  "吾道一以貫之。" as "忠恕。",
-} from "Confucius";
+export { "學而時習之，不亦說乎？", "吾道一以貫之。" as "忠恕。" } from "Confucius";
 export { "smile4" } from "./emojis.js";
```
# js/multiparser-comments/comment-inside.js
```diff
 // #9274
 html`
   <div>
     ${
-      this.set && this.set.artist
-      /* avoid console errors if `this.set` is undefined */
-    }
+  this.set && this.set.artist
+  /* avoid console errors if `this.set` is undefined */
+}
   </div>
 `;
 
 html`${
   foo
   /* comment */
 }`;
 html`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
-graphql`
-  ${
-    foo
-    /* comment */
-  }
-`;
+graphql`${
+  foo
+  /* comment */
+}`;
 graphql`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
+css`${
+  foo
+  /* comment */
+}`;
 css`
-  ${
-    foo
-    /* comment */
-  }
-`;
-css`
-  ${
-    foo
-    /* comment */
-  }
+${
+  foo
+  /* comment */
+}
 `;
 
 markdown`${
   foo
   /* comment */
 }`;
 markdown`
 ${
   foo
   /* comment */
 }
 `;
 
 // https://github.com/prettier/prettier/pull/9278#issuecomment-700589195
-expr1 = html`
+expr1 =
+  html`
   <div>
-    ${x(
+    ${
+    x(
       foo, // fg
-      bar
-    )}
-  </div>
+      bar,
+    )
+  }</div>
 `;
```
# js/multiparser-css/colons-after-substitutions.js
```diff
 const Icon = styled.div`
   flex: none;
-  transition: fill 0.25s;
+  transition:    fill 0.25s;
   width: 48px;
   height: 48px;
 
   ${Link}:hover {
-    fill: rebeccapurple;
+    fill:   rebeccapurple;
   }
 
   ${Link} :hover {
     fill: yellow;
   }
 
-  ${media.smallDown}::before {
-  }
+  ${media.smallDown}::before {}
 `;
```
# js/multiparser-css/colons-after-substitutions2.js
```diff
 const Icon = styled.div`
   height: 48px;
 
   ${Link}:nth-child(2) {
     fill: rebeccapurple;
   }
 `;
 
 const Icon2 = styled.div`
   height: 48px;
 
-  ${Link}:empty:before {
+  ${Link}:empty:before{
     fill: rebeccapurple;
   }
 `;
 
 const Icon3 = styled.div`
   height: 48px;
 
   ${Link}:not(:first-child) {
     fill: rebeccapurple;
   }
 `;
```
# js/multiparser-css/issue-11797.js
```diff
 const paragraph1 = css`
   font-size: 12px;
-  transform: ${vert ? "translateY" : "translateX"}
-    (${translation + handleOffset}px);
+  transform: ${vert ? "translateY" : "translateX"}(${translation + handleOffset}px);
 `;
 
 const paragraph2 = css`
   transform: ${expr}(30px);
 `;
 
 const paragraph3 = css`
   transform: ${expr} (30px);
 `;
```
# js/multiparser-css/issue-2883.js
```diff
 export const foo = css`
-  &.foo .${bar}::before,&.foo[value="hello"] .${bar}::before {
-    position: absolute;
-  }
+&.foo .${bar}::before,&.foo[value="hello"] .${bar}::before {
+	position: absolute;
+}
 `;
 
 export const foo2 = css`
-  a.${bar}:focus,a.${bar}:hover {
-    color: red;
-  }
+a.${bar}:focus,a.${bar}:hover {
+  color: red;
+}
 `;
 
 export const global = css`
-  button.${foo}.${bar} {
-    color: #fff;
-  }
+button.${foo}.${bar} {
+  color: #fff;
+}
 `;
```
# js/multiparser-css/issue-5697.js
```diff
 const StyledH1 = styled.div`
   font-size: 2.5em;
   font-weight: ${(props) => (props.strong ? 500 : 100)};
   font-family: ${constants.text.displayFont.fontFamily};
   letter-spacing: ${(props) => (props.light ? "0.04em" : 0)};
   color: ${(props) => props.textColor};
-  ${(props) =>
-    props.center
-      ? ` display: flex;
+  ${
+  (props) =>
+    props.center ? ` display: flex;
                 align-items: center;
                 justify-content: center;
-                text-align: center;`
-      : ""}
-  @media (max-width: ${(props) =>
-    props.noBreakPoint ? "0" : constants.layout.breakpoint.break1}px) {
+                text-align: center;` : ""
+}
+  @media (max-width: ${
+  (props) => (props.noBreakPoint ? "0" : constants.layout.breakpoint.break1)
+}px) {
     font-size: 2em;
   }
 `;
```
# js/multiparser-css/issue-5961.js
```diff
 const Steps = styled.div`
   @media (min-width: 1px) {
-    ${Step}:nth-child(odd) {
-    }
+    ${Step}:nth-child(odd) {}
   }
 `;
 
 const Steps2 = styled.div`
   @media (min-width: ${breakpoints.lg}) {
     ${Step} {
       margin-bottom: 90px;
     }
 
     ${Step}:nth-child(odd) {
       ${StepItemDescription} {
         grid-row: 1;
         grid-column: 3 / span 3;
       }
       ${Image} {
         grid-row: 1;
         grid-column: 7 / span 6;
       }
     }
 
     ${Step}:nth-child(even) {
       ${Image} {
         grid-row: 1;
         grid-column: 3 / span 6;
       }
       ${StepItemDescription} {
         grid-row: 1;
         grid-column: 10 / span 3;
       }
     }
   }
 `;
```
# js/multiparser-css/issue-9072.js
```diff
 const style1 = css`
-  width: ${size + 10}${sizeUnit};
-  border: ${size / 10} ${sizeUnit} solid ${color};
+  width:${size + 10}${sizeUnit};
+  border:${size / 10} ${sizeUnit} solid ${color};
 `;
 
 const style2 = css`
   width: ${size + 10}${sizeUnit};
   border: ${size / 10} ${sizeUnit} solid ${color};
 `;
 
 const style3 = css`
-  foo: ${foo}${bar} ${baz};
+  foo: ${foo}${bar}       ${baz};
 `;
```
# js/multiparser-css/styled-components-multiple-expressions.js
```diff
 const Header = styled.div`
   ${something()}
   & > ${Child}:not(:first-child) {
-    margin-left: 5px;
-  }
+margin-left:5px;
+}
 `;
 
 const Header2 = styled.div`
   ${something()}
   & > ${Child}${Child2}:not(:first-child) {
-    margin-left: 5px;
-  }
+margin-left:5px;
+}
 `;
 
-styled.div`
-  ${foo}-idle {
-  }
-`;
+styled.div`${foo}-idle { }`;
 
-styled.div`
-  ${foo}-0-idle {
-  }
-`;
+styled.div`${foo}-0-idle { }`;
 
 styled.div`
-  font-family: "${a}", "${b}";
+font-family: "${a}", "${b}";
 `;
```
# js/multiparser-css/styled-components.js
```diff
 const ListItem1 = styled.li``;
 
-const ListItem2 = styled.li``;
+const ListItem2 = styled.li` `;
 
-const Dropdown = styled.div`
-  position: relative;
-`;
+const Dropdown = styled.div`position: relative;`;
 
 const Button = styled.button`
-  color: palevioletred;
+	  color:   palevioletred ;
 
-  font-size: 1em;
+	font-size : 1em   ;
 `;
 
 const TomatoButton = Button.extend`
-  color: tomato;
+	color  : tomato  ;
+
+border-color : tomato
+    ;
 
-  border-color: tomato;
 `;
 
 Button.extend.attr({})`
-  border-color: black;
+border-color : black;
 `;
 
 styled(ExistingComponent)`
-  color: papayawhip;
-  background-color: firebrick;
-`;
+       color : papayawhip ; background-color: firebrick`;
 
 styled.button.attr({})`
-  border: rebeccapurple;
-`;
+border : rebeccapurple`;
 
 styled(ExistingComponent).attr({})`
-  border: rebeccapurple;
-`;
+border : rebeccapurple`;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
-  ${(props) => (props.small ? "font-size: 0.8em;" : "")};
+  ${(props) => props.small ? "font-size: 0.8em;" : ""};
 `;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
-  ${(props) => (props.small ? "font-size: 0.8em;" : "")}
+  ${(props) => props.small ? "font-size: 0.8em;" : ""}
 `;
 
 styled.div`
-  /* prettier-ignore */
+   /* prettier-ignore */
   color: ${(props) => props.theme.colors.paragraph};
-  ${(props) => (props.small ? "font-size: 0.8em;" : "")};
+  ${(props) => props.small ? "font-size: 0.8em;" : ""};
 `;
 
 styled.div`
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
-  ${(props) => (props.small ? "font-size: 0.8em;" : "")};
+  ${(props) => props.small ? "font-size: 0.8em;" : ""};
   /* prettier-ignore */
-  ${(props) => (props.red ? "color: red;" : "")};
+  ${(props) => props.red ? "color: red;" : ""};
 `;
 
 styled.div`
   /* prettier-ignore */
   color: ${(props) => props.theme.colors.paragraph};
   /* prettier-ignore */
-  ${(props) => (props.small ? "font-size: 0.8em;" : "")};
+  ${(props) => props.small ? "font-size: 0.8em;" : ""};
   /* prettier-ignore */
-  ${(props) => (props.red ? "color: red;" : "")};
+  ${(props) => props.red ? "color: red;" : ""};
   /* prettier-ignore */
 `;
 
 styled.div`
-  ${sanitize} ${fonts}
+ ${sanitize} ${fonts}
   html {
     margin: 0;
   }
 `;
 
 styled.div`
   ${bar}
   baz
 `;
 
 styled.span`
   foo
   ${bar}
   baz
 `;
 
 styled.div`
   foo
   ${bar}
   ${baz}
 `;
 
 styled.span`
   ${foo}
   ${bar}
 `;
 
 styled.div`
   ${foo} bar
 `;
 
 styled.span`
   ${foo} ${bar}
   baz: ${foo}
 `;
 
 styled.span`
-  ${foo};
-  ${bar};
+${foo};
+${bar};
 `;
 
 styled.span`
-  ${foo}: ${bar};
+${foo}: ${bar};
 `;
 
 styled.span`
-  ${foo}: ${bar}
+${foo}: ${bar}
 `;
 
 styled.span`
-  ${foo}: ${bar}
+${foo}:
+${bar}
 `;
 
 styled.span`
-  ${foo}: ${bar};
+${foo}:
+${bar};
 `;
 
 styled.a`
   ${feedbackCountBlockCss}
   text-decoration: none;
 
   ${FeedbackCount} {
     margin: 0;
   }
 `;
 
 const StyledComponent1 = styled.div`
   ${anInterpolation}
   /* a comment */
 
   .aRule {
-    color: red;
+    color: red
   }
 `;
 
 const StyledComponent2 = styled.div`
   ${anInterpolation}
 
   /* a comment */
 
   .aRule {
-    color: red;
+    color: red
   }
 `;
 
 const Direction = styled.span`
   ${({ up }) => up && `color: ${color.positive};`}
   ${({ down }) => down && `color: ${color.negative};`}
 `;
 
 const Direction2 = styled.span`
   ${({ up }) => up && `color: ${color.positive}`};
   ${({ down }) => down && `color: ${color.negative}`};
 `;
 
 const mixin = css`
   color: ${(props) => props.color};
   ${(props) => props.otherProperty}: ${(props) => props.otherValue};
 `;
 
 const foo = styled.div`
   display: flex;
   ${(props) => props.useMixin && mixin}
 `;
 
 const Single1 = styled.div`
-  color: red;
+  color: red
 `;
 
 const Single2 = styled.div`
   color: red;
 `;
 
 const Dropdown2 = styled.div`
   /* A comment to avoid the prettier issue: https://github.com/prettier/prettier/issues/2291 */
   position: relative;
 `;
 
 const bar = styled.div`
   border-radius: 50%;
   border: 5px solid rgba(var(--green-rgb), 0);
   display: inline-block;
   height: 40px;
   width: 40px;
 
-  ${(props) =>
-    (props.complete || props.inProgress) &&
-    css`
+  ${
+  (props) =>
+    (props.complete || props.inProgress) && css`
       border-color: rgba(var(--green-rgb), 0.15);
-    `}
+    `
+}
 
   div {
     background-color: var(--purpleTT);
     border-radius: 50%;
     border: 4px solid rgba(var(--purple-rgb), 0.2);
     color: var(--purpleTT);
     display: inline-flex;
 
-    ${(props) =>
-      props.complete &&
-      css`
+    ${
+  (props) =>
+    props.complete && css`
         background-color: var(--green);
         border-width: 7px;
-      `}
+      `
+}
 
-    ${(props) =>
-      (props.complete || props.inProgress) &&
-      css`
+    ${
+  (props) =>
+    (props.complete || props.inProgress) && css`
         border-color: var(--green);
-      `}
+      `
+}
   }
 `;
 
 const A = styled.a`
   display: inline-block;
   color: #fff;
-  ${(props) =>
-    props.a &&
-    css`
-      display: none;
-    `}
-  height: 30px;
+  ${
+  (props) =>
+    props.a && css`
+    display: none;
+  `
+}
+   height: 30px;
 `;
 
 const Foo = styled.p`
   max-width: 980px;
-  ${mediaBreakpointOnlyXs`
+  ${
+  mediaBreakpointOnlyXs`
     && {
       font-size: 0.8rem;
     }
-  `}
+  `
+}
 
   &.bottom {
     margin-top: 3rem;
   }
 `;
 
 styled(A)`
   // prettier-ignore
   @media (aaaaaaaaaaaaa) {
 	z-index: ${(props) => (props.isComplete ? "1" : "0")};
   }
 `;
 
 const StyledDiv = styled.div`
   ${(props) => getSize(props.$size.xs)}
   ${(props) => getSize(props.$size.sm, "sm")}
   ${(props) => getSize(props.$size.md, "md")}
 `;
```
# js/multiparser-css/url.js
```diff
-styled.div`
-  color: red;
-  background: url(http://example.com?q=${foo});
-`;
+styled.div`color:red;background: url(http://example.com?q=${foo})`;
```
# js/multiparser-css/var.js
```diff
 const Something = styled.div`
   background: var(--${one}); /* ... */
   border: 1px solid var(--${two}); /* ... */
 `;
 
 const StyledPurchaseCard = styled(Card)`
   min-width: 200px;
   background-color: var(--${(props) => props.color});
   color: #fff;
 `;
 
 const v1 = css`
-  prop: var(--global--color--${props.variant});
+prop: var(--global--color--${props.variant});
 `;
 
 const v2 = css`
-  background-color: var(--global--color--${props.variant});
+        background-color: var(--global--color--${props.variant});
 
-  &:hover {
-    background-color: var(--global--color--${props.variant}__one);
-  }
-`;
+        &:hover {
+          background-color: var(--global--color--${props.variant}__one);
+        }
+      `;
 
 export const StyledComponent = styled.div`
-  grid-area: area-${(props) => props.propName};
+  grid-area:  area-${(props) => props.propName};
 `;
```
# js/multiparser-graphql/comment-tag.js
```diff
 const query = /* GraphQL */ `
-  {
-    user(id: 5) {
+      {
+    user(   id :   5  )  {
       firstName
 
       lastName
     }
   }
 `;
```
# js/multiparser-graphql/escape.js
```diff
 gql`
   "\`foo\` mutation payload."
-  type FooPayload {
-    bar: String
+  type      FooPayload       {
+    	bar: String
   }
 `;
 
 gql`
-  type Project {
+type Project {
     "Pattern: \`\${project}\`"
     pattern: String
     """
     Pattern: \`\${project}\`
     """
     pattern: String
 
-    # Also: Escaping the first parentheses...
-    "Pattern: \`\${project}\`"
+	# Also: Escaping the first parentheses...
+	"Pattern: \`$\{project}\`"
     pattern: String
     # Or escaping the first and second parentheses...
-    "Pattern: \`\${project}\`"
+	"Pattern: \`$\{project\}\`"
     pattern: String
-  }
+}
 `;
 
 gql`
   """
   - \`
   - \\\`
   - \\ a
   - \\\\
   - $
   - \$
   - \${
   - \\\${
   - \u1234
   """
   type A {
     a
   }
 `;
```
# js/multiparser-graphql/expressions.js
```diff
 graphql(
   schema,
   `
-    query allPartsByManufacturerName($name: String!) {
-      allParts(filter: { manufacturer: { name: $name } }) {
-        ...PartAll
-      }
-    }
-    ${fragments.all}
-  `
+query allPartsByManufacturerName($name: String!) {
+  allParts(filter:{manufacturer: {name: $name}}) {
+...    PartAll
+}}
+${fragments.all}
+`,
 );
 
 const veryLongVariableNameToMakeTheLineBreak = graphql(
   schema,
   `
-    query allPartsByManufacturerName($name: String!) {
-      allParts(filter: { manufacturer: { name: $name } }) {
-        ...PartAll
-      }
-    }
-    ${fragments.all}
-  `
+query allPartsByManufacturerName($name: String!) {
+  allParts(filter:{manufacturer: {name: $name}}) {
+...    PartAll
+}}
+${fragments.all}
+`,
 );
```
# js/multiparser-graphql/graphql-tag.js
```diff
 import gql from "graphql-tag";
 
 const query = gql`
-  {
-    user(id: 5) {
+      {
+    user(   id :   5  )  {
       firstName
 
       lastName
     }
   }
 `;
 
 // With interpolations:
 
 gql`
-  query User {
-    user(id: 5) {
-      ...UserDetails
-      ...Friends
-    }
+query User {
+  user(id:5){
+    ...UserDetails
+    ...Friends
   }
+}
 
-  ${USER_DETAILS_FRAGMENT}
-  ${FRIENDS_FRAGMENT}
+${USER_DETAILS_FRAGMENT}${FRIENDS_FRAGMENT}
 `;
 
 // Skip if non-toplevel interpolation:
 
 gql`
 query User {
   user(id:${id}){ name }
 }
 `;
 
 // Skip if top-level interpolation within comment:
 
 gql`
 query User {
   user(id:5){ name }
 }
 #${test}
 `;
 
 // Comment on last line:
 
 gql`
-  query User {
-    user(id: 5) {
-      name
-    }
-  }
-  # comment
-`;
+query User {
+  user(id:5){ name }
+}
+# comment`;
 // ` <-- editor syntax highlighting workaround
 
 // Preserve up to one blank line between things and enforce linebreak between
 // interpolations:
 
 gql`
-  # comment
-  ${one}
-  ${two}
-  ${three}
-  ${four}
+# comment
+${one}${two}  ${three}
+${four}
+
+${five}
+# comment
+${six}
+
+# comment
+${seven}
+# comment
+
+${eight}
 
-  ${five}
-  # comment
-  ${six}
+  # comment with trailing whitespace      
 
-  # comment
-  ${seven}
-  # comment
 
-  ${eight}
+# blank line above this comment
 
-  # comment with trailing whitespace
 
-  # blank line above this comment
 `;
 
 // Interpolation directly before and after query:
 
-gql`
-  ${one}
-  query Test {
-    test
-  }
-  ${two}
-`;
+gql`${one} query Test { test }${two}`;
 
 // Only interpolation:
 
-gql`
-  ${test}
-`;
+gql`${test}`;
 
 // Only comment:
 
-gql`
-  # comment
-`;
+gql`# comment`;
 // ` <-- editor syntax highlighting workaround
 
 // Only whitespace:
 
-gql``;
+gql`   `;
 
 // Empty:
 
 gql``;
 
 // Comments after other things:
 // Currently, comments after interpolations are moved to the next line.
 // We might want to keep them on the next line in the future.
 
 gql`
-  ${test}
-  # comment
+  ${test} # comment
 
-  query Test {
-    # comment
+  query Test { # comment
     test # comment
   } # comment
-  ${test}
-  # comment
-  ${test}
-  # comment
+  ${test} # comment
+  ${test} # comment
 
-  ${test}
-  # comment
+  ${test} # comment
 
   # comment
-  ${test}
-  # comment
+  ${test} # comment
 `;
 
 // Larger mixed test:
 
 gql`
-  query User {
-    test
-  }
 
-  ${USER_DETAILS_FRAGMENT}
 
-  # Comment
-  # that continues on a new line
 
-  # and has a blank line in the middle
+query User {
+  test
+}
+
+    
+	
+${USER_DETAILS_FRAGMENT}
+
+   # Comment    
+   # that continues on a new line
+
+    
+   # and has a blank line in the middle
 
-  ${FRIENDS_FRAGMENT}
-  ${generateFragment({
-    totally: "a good idea",
-  })}
+    ${FRIENDS_FRAGMENT}
+  ${generateFragment({ totally: "a good idea" })}
 
-  ${fragment}
-  #comment
+${fragment}#comment
 
-  fragment another on User {
-    name
-  }
-  ${fragment}
-`;
+fragment another on User { name
+}${fragment}`;
```
# js/multiparser-graphql/graphql.js
```diff
 graphql(
   schema,
   `
-    mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-      markReadNotification(data: $input) {
-        notification {
-          seenState
-        }
-      }
-    }
-  `
+mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }`,
 );
```
# js/multiparser-graphql/react-relay.js
```diff
 const { graphql } = require("react-relay");
 
 graphql`
-  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-    markReadNotification(data: $input) {
-      notification {
-        seenState
-      }
-    }
-  }
+ mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }
 `;
 
 graphql.experimental`
-  mutation MarkReadNotificationMutation($input: MarkReadNotificationData!) {
-    markReadNotification(data: $input) {
-      notification {
-        seenState
-      }
-    }
-  }
+ mutation     MarkReadNotificationMutation(
+    $input
+    : MarkReadNotificationData!
+  )
+{ markReadNotification(data: $input ) { notification {seenState} } }
 `;
```
# js/multiparser-html/html-template-literals.js
```diff
 const nestedFun = /* HTML */ `${outerExpr(1)}
   <script>
     const tpl = html\`<div>\${innerExpr(1)} ${outerExpr(2)}</div>\`;
   </script>`;
 
 const nestedFun2 = /* HTML */ `${outerExpr(1)}
   <script>
-    const tpl = html\` <div>\${innerExpr(1)} ${outerExpr(2)}</div> \`;
+    const tpl = html\`\\n<div>\${innerExpr(1)} ${outerExpr(2)}</div>\\n\`;
   </script>`;
 
 setFoo(
   html`<div>one</div>
     <div>two</div>
     <div>three</div>`,
-  secondArgument
+  secondArgument,
 );
 
 setFoo(
   html`<div>
       <div>nested</div>
     </div>
     <div>two</div>
     <div>three</div>`,
-  secondArgument
+  secondArgument,
 );
 
 setFoo(
   html`<div>
     <div>nested</div>
   </div>`,
-  secondArgument
+  secondArgument,
 );
```
# js/multiparser-html/issue-10691.js
```diff
 export default function include_photoswipe(gallery_selector = ".my-gallery") {
-  return /* HTML */ ` <script>
-    window.addEventListener("load", () =>
-      initPhotoSwipeFromDOM("${gallery_selector}")
-    );
-  </script>`;
+  return `
+		<script>
+			window.addEventListener("load", () =>
+				initPhotoSwipeFromDOM("${gallery_selector}")
+			);
+		</script>`; /* HTML */
 }
```
# js/multiparser-html/lit-html.js
```diff
 import { LitElement, html } from "@polymer/lit-element";
 
 class MyElement extends LitElement {
   static get properties() {
-    return {
-      mood: { type: String },
-    };
+    return { mood: { type: String } };
   }
 
   constructor() {
     super();
     this.mood = "happy";
   }
 
   render() {
     return html`
-      <style>
-        .mood {
-          color: green;
-        }
-      </style>
+      <style
+      
+      
+      >
+                  .mood { color: green; }
+      </style
+      
+      
+      
+      >
 
-      Web Components are <span class="mood">${this.mood}</span>!
+         Web            Components         are     <span 
+      
+      
+      class="mood"      >${this.mood}</span
+      
+           >!
     `;
   }
 }
 
 customElements.define("my-element", MyElement);
 
-const someHtml1 = html`<div>hello ${world}</div>`;
-const someHtml2 = /* HTML */ `<div>hello ${world}</div>`;
+const someHtml1 = html`<div       > hello ${world} </div     >`;
+const someHtml2 = /* HTML */ `<div      > hello ${world} </div     >`;
 
 html``;
 
 html`<my-element obj=${obj}></my-element>`;
 
-html` <${Footer}>footer content<//> `;
+html`  <${Footer}  >footer      content<//     >  `;
 
-html` <div /> `;
+html`  <div />  `;
 
-html` <div /> `;
+html`
+  <div />
+`;
 
 html`<span>one</span><span>two</span><span>three</span>`;
 
 function HelloWorld() {
   return html`
     <h3>Bar List</h3>
-    ${bars.map((bar) => html` <p>${bar}</p> `)}
+    ${bars.map(
+    (bar) =>
+      html`
+       <p>${bar}</p>
+    `,
+  )}
   `;
 }
 
-const trickyParens = html`<script>
-  f((${expr}) / 2);
-</script>`;
-const nestedFun = /* HTML */ `${outerExpr(1)}
-  <script>
-    const tpl = html\`<div>\${innerExpr(1)} ${outerExpr(2)}</div>\`;
-  </script>`;
+const trickyParens = html`<script> f((${expr}) / 2); </script>`;
+const nestedFun = /* HTML */ `${outerExpr(1)} <script>const tpl = html\`<div>\${innerExpr( 1 )} ${outerExpr(
+  2,
+)}</div>\`</script>`;
 
 const closingScriptTagShouldBeEscapedProperly = /* HTML */ `
   <script>
     const html = /* HTML */ \`<script><\\/script>\`;
   </script>
 `;
 
-const closingScriptTag2 = /* HTML */ `<script>
-  const scriptTag = "<\\/script>";
-</script>`;
+const closingScriptTag2 = /* HTML */ `<script>const  scriptTag='<\\/script>'; <\/script>`;
 
 html`
-  <div
-    style="
+ <div style="
  ${foo}
-"
-  ></div>
+"></div>
 `;
-html` <div style=${foo}></div> `;
+html`
+ <div style=${foo}></div>
+`;
 
-html`<div
-  style="   color : red;
-            display    :inline "
-></div>`;
+html`<div style="   color : red;
+            display    :inline ">
+  </div>`;
 
-html`<div
-  style="   color : red;
+html`<div style="   color : red;
 ${foo}
-            display    :inline "
-></div>`;
-html`<div
-  style="   color : red;
+            display    :inline ">
+  </div>`;
+html`<div style="   color : red;
 ${foo}:${bar};
-            display    :inline "
-></div>`;
+            display    :inline ">
+  </div>`;
```
# js/multiparser-text/text.js
```diff
-a = {
-  viewer: graphql`
+a =
+  {
+    viewer: graphql`
     fragment x on Viewer {
-      y(
-        named: [
-          "projects_feedback_ids" # PROJECTS_FEEDBACK_IDS
-        ]
-      ) {
+      y(named: [
+        "projects_feedback_ids" # PROJECTS_FEEDBACK_IDS
+      ]) {
         name
       }
     }
   `,
-};
+  };
```
# js/new-expression/new_expression.js
```diff
 new (memoize.Cache || MapCache)();
 new (typeof this == "function" ? this : Dict())();
-new (createObj().prop)(a());
+new (createObj()).prop(a());
 new (x()``.y)();
 new e[f().x].y();
 new e[f()].y();
 new (a().b)();
 new (a().b().c)();
 new (a``())();
```
# js/new-expression/with-member-expression.js
```diff
 function functionName() {
   // indent to make the line break
   if (true) {
-    this._aVeryLongVariableNameToForceLineBreak = new this.Promise(
-      (resolve, reject) => {
-        // do something
-      }
-    );
+    this._aVeryLongVariableNameToForceLineBreak =
+      new this.Promise(
+        (resolve, reject) => {
+          // do something
+        },
+      );
   }
 }
```
# js/no-semi-babylon-extensions/no-semi.js
```diff
 a;
-::b.c;
+::b.c
 
 class A {
   a = b;
   in;
   c;
 
   a = b;
   instanceof() {}
 }
```
# js/no-semi/no-semi.js
```diff
 // with preexisting semi
 
 x;
 [1, 2, 3].forEach(fn);
 x;
 [a, b, ...c] = [1, 2];
 x;
 /r/i.test("r");
 x;
 +1;
 x;
 -1;
 x;
 ("h" + "i").repeat(10);
 x;
-1, 2;
+(1, 2);
 x;
 (() => {})();
 x;
-({ a: 1 }.entries());
+({ a: 1 }).entries();
 x;
-({ a: 1 }.entries());
+({ a: 1 }).entries();
 x;
 <Hello />;
 x;
 `string`;
 x;
 (x, y) => x;
 
 // doesn't have to be preceded by a semicolon
 
 class X {}
 [1, 2, 3].forEach(fn);
 
 // don't semicolon if it doesn't start statement
 
-if (true) (() => {})();
+if (true) {
+  (() => {})();
+}
 
 // check indentation
 
 if (true) {
   x;
   (() => {})();
 }
 
 // check statement clauses
 
-do break;
-while (false);
-if (true)
-  do break;
-  while (false);
+do break; while (false);
+if (true) {
+  do break; while (false);
+}
 
-if (true) 1;
-else 2;
+if (true) {
+  1;
+} else {
+  2;
+}
 for (;;);
 for (x of y);
 
 debugger;
 
 // check that it doesn't break non-ASI
 
 1 - 1;
 
 1 + 1;
 
 1 / 1;
 
 arr[0];
 
 fn(x);
 
 !1;
 
 1 < 1;
 
 tag`string`;
 
 x;
 (x) => x;
 
 x;
 (a || b).c++;
 
 x;
 ++(a || b).c;
 
-while (false) (function () {})();
+while (false) (function () {}());
 
-aReallyLongLine012345678901234567890123456789012345678901234567890123456789 *
-  (b + c);
+aReallyLongLine012345678901234567890123456789012345678901234567890123456789 * (
+  b + c
+);
```
# js/nullish-coalescing/nullish_coalesing_operator.js
```diff
 obj.foo ?? "default";
 
 const x = (foo, bar = foo ?? bar) => {};
 
 foo ? bar ?? foo : baz;
 
-foo ?? bar ?? baz;
-foo ?? bar ?? baz;
+foo ?? (bar ?? baz);
+(foo ?? bar) ?? baz;
 
 // Mixing ?? and (&& or ||) requires parens
 // It's a syntax error without it.
 (foo ?? baz) || baz;
 foo ?? (baz || baz);
 
 (foo ?? baz) && baz;
 foo ?? (baz && baz);
 
 (foo || baz) ?? baz;
 foo || (baz ?? baz);
 
 (foo && baz) ?? baz;
 foo && (baz ?? baz);
```
# js/object-prop-break-in/long-value.js
```diff
 const x = {
-  ABC: "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  "ABC": "12345678901234567890123456789012345678901234567890123456789012345678901234567890",
 };
```
# js/object-prop-break-in/short-keys.js
```diff
 var obj = {
   // an entry with a very long string
   x: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   url: "http://example.com/12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  longName:
-    "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  longName: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   [i]: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  [prop]:
-    "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  x: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  [prop]: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  "x": "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   a: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   ab: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   abc: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
   abcd: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  abcde:
-    "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  abcdef:
-    "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
-  古: "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
-  古今: "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
-  古体诗:
-    "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
+  abcde: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  abcdef: "12345678901234567890123456789012345678901234567890123456789012345678901234567890123456789012345678901234567890",
+  "古": "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
+  "古今": "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
+  "古体诗": "https://prettier.io/docs/en/rationale.html#what-prettier-is-concerned-about",
 };
```
# js/object-prop-break-in/test.js
```diff
 const a = classnames({
   "some-prop": this.state.longLongLongLongLongLongLongLongLongTooLongProp,
 });
 
 const b = classnames({
-  "some-prop":
-    this.state.longLongLongLongLongLongLongLongLongTooLongProp === true,
+  "some-prop": this.state.longLongLongLongLongLongLongLongLongTooLongProp === true,
 });
 
 const c = classnames({
   "some-prop": ["foo", "bar", "foo", "bar", "foo", "bar", "foo", "bar", "foo"],
 });
 
-const d = classnames({
-  "some-prop": () => {},
-});
+const d = classnames({ "some-prop": () => {} });
 
-const e = classnames({
-  "some-prop": function bar() {},
-});
+const e = classnames({ "some-prop": function bar() {} });
 
 const f = classnames({
   "some-prop": { foo: "bar", bar: "foo", foo: "bar", bar: "foo", foo: "bar" },
 });
 
 const g = classnames({
-  "some-prop":
-    longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337,
+  "some-prop": longLongLongLongLongLongLongLongLongLongLongLongLongTooLongVar || 1337,
 });
 
 const h = {
   foo: "bar",
   baz: `Lorem
 ipsum`,
 };
```
# js/object-property-comment/after-key.js
```diff
-let a = {
-  a /* comment */: () => 1,
-};
+let a = { a /* comment */ : () => 1 };
 
-let b = {
-  a /* comment */: () => 1,
-};
+let b = { "a" /* comment */ : () => 1 };
```
# js/object-property-ignore/ignore.js
```diff
-foo = {
-  // prettier-ignore
+foo =
+  {
+    // prettier-ignore
   bar:            1,
-};
+  };
 
-foo = {
-  _: "",
-  // prettier-ignore
+foo =
+  {
+    _: "",
+    // prettier-ignore
   bar:            1,
-};
+  };
 
 /* comments */
-foo = {
-  _: "",
-  // prettier-ignore
+foo =
+  {
+    _: "",
+    // prettier-ignore
   bar:            1, // comment
-};
+  };
 
-foo = {
-  _: "",
-  // prettier-ignore
-  bar:            1 /* comment */,
-};
+foo =
+  {
+    _: "",
+    // prettier-ignore
+  bar:            1, /* comment */
+  };
 
-foo = {
-  _: "",
-  // prettier-ignore
+foo =
+  {
+    _: "",
+    // prettier-ignore
   bar:            /* comment */          1,
-};
+  };
 
 /* SpreadElement */
-foo = {
-  _: "",
-  // prettier-ignore
+foo =
+  {
+    _: "",
+    // prettier-ignore
   ...bar,
-};
+  };
 
 // Nested
-foo = {
-  baz: {
+foo =
+  {
+    baz: {
+      // prettier-ignore
+  foo: [1, 2,    3],
+    },
     // prettier-ignore
-    foo: [1, 2,    3],
-  },
-  // prettier-ignore
   bar:            1,
-};
+  };
```
# js/object-property-ignore/issue-5678.js
```diff
 // #5678
 const refreshTokenPayload = {
   type: "refreshToken",
   sub: this._id,
   role: this.role,
   // prettier-ignore
-  exp: now + (60 * 60 * 24 * 90), // (90 days)
+    exp: now + (60 * 60 * 24 * 90), // (90 days)
 };
 
 export default {
   // prettier-ignore
   protagonist: "  0\r\n" +
                "0 00\r\n" +
                "00000\r\n" +
                "0 0\r\n" +
                "0 0",
 
   // prettier-ignore
   wall: "00000\r\n" +
         "00000\r\n" +
         "00000\r\n" +
         "00000\r\n" +
         "00000",
 
   // prettier-ignore
   cheese: "0\r\n" +
           " 0\r\n" +
           "000\r\n" +
           "00 0\r\n" +
           "00000",
 
   // prettier-ignore
   enemy: "0   0\r\n" +
          "00 00\r\n" +
          "00000\r\n" +
          "0 0 0\r\n" +
          "00000",
 
   // prettier-ignore
   home: "00000\r\n" +
         "0   0\r\n" +
         "0   0\r\n" +
         "0   0\r\n" +
         "00000",
 
   // prettier-ignore
   dog: "00 00\r\n" +
        "00000\r\n" +
        "0   0\r\n" +
        "0 0 0\r\n" +
        " 000 ",
 };
```
# js/objects/assignment-expression/object-property.js
```diff
-a = {
-  [(this.resource = resource)]: 1,
-};
+a = { [this.resource = resource]: 1 };
```
# js/objects/assignment-expression/object-value.js
```diff
-a = {
-  resource: (this.resource = resource),
-};
+a = { resource: (this.resource = resource) };
 
-map(([resource]) => ({
-  resource: (this.resource = resource),
-}));
+map(([resource]) => ({ resource: (this.resource = resource) }));
```
# js/objects/escape-sequence-key.js
```diff
 // #6235
-const a = {
-  "\u2139": 'why "\\u2139" is converted to "i"?',
-};
+const a = { "\u2139": 'why "\\u2139" is converted to "i"?' };
 
-const b = {
-  "\x66\x69\x73\x6b\x65\x72": "\x66\x69\x73\x6b\x65\x72",
-};
+const b = { "\x66\x69\x73\x6b\x65\x72": "\x66\x69\x73\x6b\x65\x72" };
```
# js/objects/expand.js
```diff
-const Component1 = ({ props }) => <Text>Test</Text>;
+const Component1 = ({ props }) => (<Text>Test</Text>);
 
-const Component2 = ({ props }) => <Text>Test</Text>;
+const Component2 = ({ props }) => (<Text>Test</Text>);
```
# js/objects/expression.js
```diff
 () => ({}``);
-({}``);
-a = () => ({}.x);
+({})``;
+a = () => ({}).x;
 ({} && a, b);
-({}::b, 0);
-({}::b()``[""].c++ && 0 ? 0 : 0, 0);
+({}
+::b, 0)
+({}
+::b()``[''].c++ && 0 ? 0 : 0, 0)
 ({}(), 0);
 ({} = 0);
-({} = 0), 1;
+(({} = 0), 1);
 
-const a1 = {
-  someKey: (shortName, shortName),
-};
+const a1 = { someKey: (shortName, shortName) };
 
 const a2 = {
-  someKey:
-    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName, shortName),
+  someKey: (
+    longLongLongLongLongLongLongLongLongLongLongLongLongLongName, shortName
+  ),
 };
 
 const a3 = {
-  someKey:
-    (longLongLongLongLongLongLongLongLongLongLongLongLongLongName,
-    longLongLongLongLongLongLongLongLongLongLongLongLongLongName,
-    longLongLongLongLongLongLongLongLongLongLongLongLongLongName),
+  someKey: (
+    longLongLongLongLongLongLongLongLongLongLongLongLongLongName, longLongLongLongLongLongLongLongLongLongLongLongLongLongName, longLongLongLongLongLongLongLongLongLongLongLongLongLongName
+  ),
 };
```
# js/objects/range.js
```diff
 group(
   concat([
     "(",
-    indent(
-      options.tabWidth,
-      concat([line, join(concat([",", line]), printed)])
-    ),
+    indent(options.tabWidth, concat([line, join(concat([",", line]), printed)])),
     options.trailingComma ? "," : "",
     line,
     ")",
   ]),
-  { shouldBreak: true }
+  { shouldBreak: true },
 );
```
# js/objects/right-break.js
```diff
 const blablah =
   "aldkfkladfskladklsfkladklfkaldfadfkdaf" +
   "adlfasdklfkldsklfakldsfkladsfkadsfladsfa" +
   "dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf";
 
 const k = {
-  blablah:
-    "aldkfkladfskladklsfkladklfkaldfadfkdaf" +
+  blablah: "aldkfkladfskladklsfkladklfkaldfadfkdaf" +
     "adlfasdklfkldsklfakldsfkladsfkadsfladsfa" +
     "dflkadfkladsfklkadlfkladlfkadklfjadlfdfdaf",
 };
 
 somethingThatsAReallyLongPropName =
   this.props.cardType === AwesomizerCardEnum.SEEFIRST;
 
 const o = {
-  somethingThatsAReallyLongPropName:
-    this.props.cardType === AwesomizerCardEnum.SEEFIRST,
+  somethingThatsAReallyLongPropName: this.props.cardType === AwesomizerCardEnum.SEEFIRST,
 };
```
# js/optional-chaining/chaining.js
```diff
 var street = user.address?.street;
 var fooValue = myForm.querySelector("input[name=foo]")?.value;
 
 obj?.prop;
 obj?.[expr];
 func?.(...args);
 
 a?.();
 a?.[++x];
 a?.b.c(++x).d;
 a?.b[3].c?.(x).d;
 a?.b.c;
 (a?.b).c;
 a?.b?.c;
 delete a?.b;
 
 a?.b[3].c?.(x).d.e?.f[3].g?.(y).h;
 
 (a?.b).c();
 (a?.b[c]).c();
 
-a?.b?.c.d?.e;
+(a?.b)?.c.d?.e;
 (a ? b : c)?.d;
 
 (list || list2)?.length;
-(list || list2)?.[list || list2];
+(list || list2)?.[(list || list2)];
 
 async function HelloWorld() {
   var x = (await foo.bar.blah)?.hi;
   a?.[await b];
   (await x)?.();
 }
 
 a[b?.c].d();
 a?.[b?.c].d();
 a[b?.c]?.d();
 a?.[b?.c]?.d();
 
-one?.fn();
+(one?.fn());
 (one?.two).fn();
 (one?.two)();
 (one?.two())();
-one.two?.fn();
+(one.two?.fn());
 (one.two?.three).fn();
-one.two?.three?.fn();
+(one.two?.three?.fn());
 
-one?.();
+(one?.());
 (one?.())();
-one?.()?.();
+(one?.())?.();
 
 (one?.()).two;
 
 a?.[b ? c : d];
 
 (-1)?.toFixed();
 (void fn)?.();
 (a && b)?.();
 (a ? b : c)?.();
 (function () {})?.();
 (() => f)?.();
 (() => f)?.x;
 (a?.(x)).x;
 (
   aaaaaaaaaaaaaaaaaaaaaaaa &&
-  aaaaaaaaaaaaaaaaaaaaaaaa &&
-  aaaaaaaaaaaaaaaaaaaaaaaa
+    aaaaaaaaaaaaaaaaaaaaaaaa &&
+    aaaaaaaaaaaaaaaaaaaaaaaa
 )?.();
 
 let f = () => ({}?.());
 let g = () => ({}?.b);
 a = () => ({}?.() && a);
 a = () => ({}?.()() && a);
 a = () => ({}?.().b && a);
 a = () => ({}?.b && a);
 a = () => ({}?.b() && a);
 (a) => ({}?.()?.b && 0);
 (a) => ({}?.b?.b && 0);
 (x) => ({}?.()());
 (x) => ({}?.().b);
 (x) => ({}?.b());
 (x) => ({}?.b.b);
 ({}?.a().b());
 ({ a: 1 }?.entries());
 
 new (foo?.bar)();
 new (foo?.bar())();
 new (foo?.())();
```
# js/optional-chaining/comments.js
```diff
 function foo() {
-  return (
-    a
-      .b()
-      .c()
-      // Comment
-      ?.d()
-  );
+  return a
+    .b()
+    .c()
+    // Comment
+    ?.d();
 }
 
 fooBar
   .doSomething("Hello World")
   .doAnotherThing("Foo", { foo: bar })
-
   // App configuration.
   .doOneMoreThing(config)
-
   ?.run(() => console.log("Bar"));
 
 bigDeal
-
   .doSomething("Hello World")
-
   // Hello world
   ?.doAnotherThing("Foo", { foo: bar })
-
   // App configuration.
   .doOneMoreThing(config)
-
   ?.run(() => console.log("Bar"));
 
 foo.bar.baz
-
   ?.doSomething("Hello World")
-
   // Hello world
   .foo.bar.doAnotherThing("Foo", { foo: bar })
-
   .doOneMoreThing(config)
   ?.bar.run(() => console.log("Bar"));
 
 (somethingGood ? thisIsIt : maybeNot)
-
   // Hello world
   .doSomething("Hello World")
-
   ?.doAnotherThing("Foo", { foo: bar }) // Run this
   .run(() => console.log("Bar")); // Do this
```
# js/optional-chaining/eval.js
```diff
 // https://github.com/babel/babel/pull/11850
 
 let foo;
 
 /* indirect eval calls */
 eval?.(foo);
 
-eval?.(foo);
+(eval)?.(foo);
 
 eval?.()();
 
 eval?.().foo;
 
 /* direct eval calls */
 
 eval()?.();
 
 eval()?.foo;
 
 /* plain function calls */
 
 foo.eval?.(foo);
 
 eval.foo?.(foo);
```
# js/partial-application/test.js
```diff
 const addOne = add(1, ?); // apply from the left
 addOne(2); // 3
 
 const addTen = add(?, 10); // apply from the right
 addTen(2); // 12
 
 // with pipeline
-let newScore = player.score |> add(7, ?) |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.
+let newScore = player.score
+  |> add(7, ?)
+  |> clamp(0, 100, ?); // shallow stack, the pipe to `clamp` is the same frame as the pipe to `add`.
```
# js/performance/nested-real.js
```diff
-tap.test("RecordImport.advance", (t) => {
-  const checkStates = (batches, states) => {
-    t.equal(batches.length, states.length);
-    for (const batch of batches) {
-      t.equal(batch.state, states.shift());
-      t.ok(batch.getCurState().name(i18n));
-    }
-  };
+tap.test(
+  "RecordImport.advance",
+  (t) => {
+    const checkStates = (batches, states) => {
+      t.equal(batches.length, states.length);
+      for (const batch of batches) {
+        t.equal(batch.state, states.shift());
+        t.ok(batch.getCurState().name(i18n));
+      }
+    };
 
-  const batch = init.getRecordBatch();
-  const dataFile = path.resolve(process.cwd(), "testData", "default.json");
+    const batch = init.getRecordBatch();
+    const dataFile = path.resolve(process.cwd(), "testData", "default.json");
 
-  const getBatches = (callback) => {
-    RecordImport.find({}, "", {}, (err, batches) => {
-      callback(
-        null,
-        batches.filter(
-          (batch) => batch.state !== "error" && batch.state !== "completed"
-        )
+    const getBatches = (callback) => {
+      RecordImport.find(
+        {},
+        "",
+        {},
+        (err, batches) => {
+          callback(
+            null,
+            batches.filter(
+              (batch) => (
+                batch.state !== "error" && batch.state !== "completed"
+              ),
+            ),
+          );
+        },
       );
-    });
-  };
+    };
+
+    mockFS((callback) => {
+      batch.setResults(
+        [fs.createReadStream(dataFile)],
+        (err) => {
+          t.error(err, "Error should be empty.");
+          t.equal(batch.results.length, 6, "Check number of results");
+          for (const result of batch.results) {
+            t.equal(result.result, "unknown");
+            t.ok(result.data);
+            t.equal(result.data.lang, "en");
+          }
+
+          getBatches(
+            (err, batches) => {
+              checkStates(batches, ["started"]);
 
-  mockFS((callback) => {
-    batch.setResults([fs.createReadStream(dataFile)], (err) => {
-      t.error(err, "Error should be empty.");
-      t.equal(batch.results.length, 6, "Check number of results");
-      for (const result of batch.results) {
-        t.equal(result.result, "unknown");
-        t.ok(result.data);
-        t.equal(result.data.lang, "en");
-      }
+              RecordImport.advance((err) => {
+                t.error(err, "Error should be empty.");
 
-      getBatches((err, batches) => {
-        checkStates(batches, ["started"]);
+                getBatches(
+                  (err, batches) => {
+                    checkStates(batches, ["process.completed"]);
 
-        RecordImport.advance((err) => {
-          t.error(err, "Error should be empty.");
+                    // Need to manually move to the next step
+                    batch.importRecords((err) => {
+                      t.error(err, "Error should be empty.");
 
-          getBatches((err, batches) => {
-            checkStates(batches, ["process.completed"]);
+                      getBatches(
+                        (err, batches) => {
+                          checkStates(batches, ["import.completed"]);
 
-            // Need to manually move to the next step
-            batch.importRecords((err) => {
-              t.error(err, "Error should be empty.");
+                          RecordImport.advance((err) => {
+                            t.error(err, "Error should be empty.");
 
-              getBatches((err, batches) => {
-                checkStates(batches, ["import.completed"]);
+                            getBatches(
+                              (err, batches) => {
+                                checkStates(
+                                  batches,
+                                  ["similarity.sync.completed"],
+                                );
 
-                RecordImport.advance((err) => {
-                  t.error(err, "Error should be empty.");
+                                RecordImport.advance((err) => {
+                                  t.error(err, "Error should be empty.");
 
-                  getBatches((err, batches) => {
-                    checkStates(batches, ["similarity.sync.completed"]);
+                                  t.ok(batch.getCurState().name(i18n));
 
-                    RecordImport.advance((err) => {
-                      t.error(err, "Error should be empty.");
+                                  getBatches(
+                                    (err, batches) => {
+                                      checkStates(batches, []);
+                                      t.end();
+                                      callback();
+                                    },
+                                  );
+                                });
 
-                      t.ok(batch.getCurState().name(i18n));
+                                t.ok(batch.getCurState().name(i18n));
+                              },
+                            );
+                          });
 
-                      getBatches((err, batches) => {
-                        checkStates(batches, []);
-                        t.end();
-                        callback();
-                      });
+                          t.ok(batch.getCurState().name(i18n));
+                        },
+                      );
                     });
 
                     t.ok(batch.getCurState().name(i18n));
-                  });
-                });
-
-                t.ok(batch.getCurState().name(i18n));
+                  },
+                );
               });
-            });
-
-            t.ok(batch.getCurState().name(i18n));
-          });
-        });
 
-        t.ok(batch.getCurState().name(i18n));
-      });
+              t.ok(batch.getCurState().name(i18n));
+            },
+          );
+        },
+      );
     });
-  });
-});
+  },
+);
```
# js/performance/nested.js
```diff
-someObject.someFunction().then(function () {
-  return someObject.someFunction().then(function () {
-    return someObject.someFunction().then(function () {
-      return someObject.someFunction().then(function () {
-        return someObject.someFunction().then(function () {
-          return someObject.someFunction().then(function () {
-            return someObject.someFunction().then(function () {
-              return someObject.someFunction().then(function () {
-                return someObject.someFunction().then(function () {
-                  return someObject.someFunction().then(function () {
-                    return someObject.someFunction().then(function () {
-                      return someObject.someFunction().then(function () {
-                        return someObject.someFunction().then(function () {
-                          return someObject.someFunction().then(function () {
-                            anotherFunction();
+someObject
+  .someFunction()
+  .then(function () {
+    return someObject
+      .someFunction()
+      .then(function () {
+        return someObject
+          .someFunction()
+          .then(function () {
+            return someObject
+              .someFunction()
+              .then(function () {
+                return someObject
+                  .someFunction()
+                  .then(function () {
+                    return someObject
+                      .someFunction()
+                      .then(function () {
+                        return someObject
+                          .someFunction()
+                          .then(function () {
+                            return someObject
+                              .someFunction()
+                              .then(function () {
+                                return someObject
+                                  .someFunction()
+                                  .then(function () {
+                                    return someObject
+                                      .someFunction()
+                                      .then(function () {
+                                        return someObject
+                                          .someFunction()
+                                          .then(function () {
+                                            return someObject
+                                              .someFunction()
+                                              .then(function () {
+                                                return someObject
+                                                  .someFunction()
+                                                  .then(function () {
+                                                    return someObject
+                                                      .someFunction()
+                                                      .then(function () {
+                                                        anotherFunction();
+                                                      });
+                                                  });
+                                              });
+                                          });
+                                      });
+                                  });
+                              });
                           });
-                        });
                       });
-                    });
                   });
-                });
               });
-            });
           });
-        });
       });
-    });
   });
-});
```
# js/pipeline-operator/block-comments.js
```diff
 bifornCringerMoshedPerplexSawder
-  |> foo1
-  |> foo2 /* comment1 */
-  |> foo3 /* comment2 */
-  |> kochabCooieGameOnOboleUnweave
-  |> glimseGlyphsHazardNoopsTieTie;
+|> foo1
+|> foo2 /* comment1 */
+|> foo3 /* comment2 */
+|> kochabCooieGameOnOboleUnweave
+|> glimseGlyphsHazardNoopsTieTie;
```
# js/pipeline-operator/fsharp_style_pipeline_operator.js
```diff
 promise
   |> await
-  |> (x) => doubleSay(x, ", ")
+  |> x
+=> doubleSay(x, ', ')
   |> capitalize
-  |> (x) => x + "!"
-  |> (x) => new User.Message(x)
-  |> (x) => stream.write(x)
+  |> x => x + '!'
+  |> x => new User.Message(x)
+  |> x => stream.write(x)
   |> await
-  |> console.log;
+  |> console.log
 
-const result = "hello" |> doubleSay |> capitalize |> exclaim;
+const result = "hello"
+  |> doubleSay
+  |> capitalize
+  |> exclaim;
 
-const newScore =
-  person.score |> double |> (n) => add(7, n) |> (n) => boundScore(0, 100, n);
+const newScore = person.score
+  |> double
+  |> n
+=> add(7, n)
+  |> n => boundScore(0, 100, n)
 
-const user =
-  url |> api.get |> await |> (r) => r.json() |> await |> (j) => j.data.user;
+const user = url
+  |> api.get
+  |> await
+  |> r
+=> r.json()
+  |> await
+  |> j => j.data.user
 
-const f = (x) => x |> (y) => y + 1 |> (z) => z * y;
+const f = (x) => (x |> (y)
+=> y + 1)
+  |> (z) => z * y
 
-const _f = (x) => x |> (y) => y + 1 |> (z) => z * y;
+const _f = (x) => x
+  |> (y)
+=> y + 1
+  |> (z) => z * y
 
-const g = (x) => x |> (y) => (y + 1 |> (z) => z * y);
+const g = (x) => x
+  |> (y)
+=> (y + 1 |> (z) => z * y)
 
-const _g = (x) => x |> ((y) => (y + 1 |> (z) => z * y));
+const _g = (x) => x
+  |> (y => (y + 1 |> (z)
+=> z * y))
 
-const __g = (x) =>
-  x
-  |> ((y) => {
-    return y + 1 |> (z) => z * y;
-  });
+const __g = (x) => x
+  |> (
+    y => {
+      return (y + 1 |> (z) => z * y);
+    }
+  )
 
-const f = x + ((f) => f |> f);
-const f = x |> (f) => f |> f;
+const f = x + ((f) => (f |> f));
+const f = x |> (f)
+=> f |> f
```
# js/pipeline-operator/hack_pipeline_operator.js
```diff
-a |> (await %) |> % * 3;
+a |> await % |> % * 3;
 
 foo
-|> (await %)
-|> % || throw new Error(`foo ${bar1}`)
-|> bar2(%, ", ")
-|> bar3(%)
-|> % + "!"
-|> new Bar.Foo(%)
-|> (await bar.bar(%))
-|> console.log(%);
+  |> await %
+  |> % ||
+throw new Error(`foo ${bar1}`)
+  |> bar2(%, ", ")
+  |> bar3(%)
+  |> % + "!"
+  |> new Bar.Foo(%)
+  |> await bar.bar(%)
+  |> console.log(%);
 
-const result = "hello" |> doubleSay(%) |> capitalize(%, "foo") |> exclaim(%);
+const result = "hello"
+  |> doubleSay(%)
+  |> capitalize(%, "foo")
+  |> exclaim(%);
 
 function createPerson(attrs) {
-  attrs |> foo(%) |> foo(%) |> Person.insertIntoDatabase(%);
+  attrs
+    |> foo(%)
+    |> foo(%)
+    |> Person.insertIntoDatabase(%);
 }
 
-const result =
-  [1, 2, 3]
-  |> %.map((a) => a * 2)
-  |> %.filter((a) => a > 5)
-  |> %.reduce((sum, a) => a + sum, 0)
-  |> increment(%)
-  |> add(%, 3);
+const result = [1,2,3]
+ |> %
+.map(a => a * 2 )
+ |> %.filter(a => a > 5)
+ |> %.reduce((sum, a) => a+sum, 0)
+ |> increment(%)
+ |> add(%, 3)
 
-const searchResults$ =
-  fromEvent(document.querySelector("input"), "input")
-  |> map(%, (event) => event.target.value)
-  |> filter(%, (searchText) => searchText.length > 2)
+const searchResults$ = fromEvent(document.querySelector('input'), 'input')
+  |> map(%, event => event.target.value)
+  |> filter(%, searchText => searchText.length > 2)
   |> debounce(%, 300)
   |> distinctUntilChanged(%)
-  |> switchMap(%, (searchText) => queryApi(searchText) |> retry(%, 3))
+  |> switchMap(%, searchText => queryApi(searchText) |> retry(%, 3))
   |> share(%);
 
-v |> %.method() |> f(%);
+v |> %
+.method() |> f(%)
 
 async function* f() {
-  return (
-    x
-    |> (yield %)
+  return x
+    |> (yield %
+  )
     |> (await %)
     |> y(%)
     |> a.b(%)
+    |> (a.b(%))
     |> a.b(%)
-    |> a.b(%)
+    |> (a.b?.(%))
     |> a.b?.(%)
-    |> a.b?.(%)
-  );
 }
```
# js/pipeline-operator/minimal_pipeline_operator.js
```diff
 a |> b |> c;
 
 a |> (b |> c);
 
 (a |> b) || c;
-a |> b || c;
+a |> (b || c);
 
-let result = "hello" |> doubleSay |> capitalize |> exclaim;
+let result = "hello"
+  |> doubleSay
+  |> capitalize
+  |> exclaim;
 
-let newScore =
-  person.score
+let newScore = person.score
   |> double
-  |> ((_) => add(7, _))
-  |> ((_) => subtract(2, _))
-  |> ((_) => boundScore(0, 100, _));
+  |> (_ => add(7, _))
+  |> (_ => subtract(2, _))
+  |> (_ => boundScore(0, 100, _));
 
 function createPerson(attrs) {
   attrs
-    |> bounded("age", 1, 100)
-    |> format("name", /^[a-z]$/i)
+    |> bounded('age', 1, 100)
+    |> format('name', /^[a-z]$/i)
     |> Person.insertIntoDatabase;
 }
 
-foo |> bar ?? baz;
+foo |> (bar ?? baz);
 (foo |> bar) ?? baz;
 
-const result =
-  [1, 2, 3]
-  |> map((a) => a * 2)
-  |> filter((a) => a > 5)
-  |> reduce((sum, a) => a + sum, 0)
-  |> increment
-  |> add(3);
+const result = [1,2,3]
+ |> map(a => a * 2)
+ |> filter(a => a > 5)
+ |> reduce((sum, a) => a+sum, 0)
+ |> increment
+ |> add(3)
 
-const searchResults$ =
-  fromEvent(document.querySelector("input"), "input")
-  |> map((event) => event.target.value)
-  |> filter((searchText) => searchText.length > 2)
+const searchResults$ = fromEvent(document.querySelector('input'), 'input')
+  |> map(event => event.target.value)
+  |> filter(searchText => searchText.length > 2)
   |> debounce(300)
   |> distinctUntilChanged()
-  |> switchMap((searchText) => queryApi(searchText) |> retry(3))
+  |> switchMap(searchText => queryApi(searchText) |> retry(3))
   |> share();
 
-const result =
-  [5, 10]
-  |> ((_) => _.map((x) => x * 2))
-  |> ((_) => _.reduce((a, b) => a + b))
-  |> ((sum) => sum + 1);
+const result = [5,10]
+  |> (_ => _.map(x => x * 2))
+  |> (_ => _.reduce( (a,b) => a + b ))
+  |> (sum => sum + 1)
 
-const result2 = [4, 9].map((x) => x |> inc |> double);
+const result2 = [4, 9].map( x => x |> inc |> double )
```
# js/preserve-line/argument-list.js
```diff
 longArgNamesWithComments(
   // Hello World
 
   longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong1,
-
   // Hello World
 
   longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong2,
-
   /* Hello World */
-  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3
+  longlonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglonglong3,
 );
-
-shortArgNames(
-  short,
 
-  short2,
-  short3
-);
+shortArgNames(short, short2, short3);
 
 comments(
   // Comment
 
   /* Some comments */
   short,
   /* Another comment */
 
   short2, // Even more comments
-
   /* Another comment */
 
   // Long Long Long Long Long Comment
 
   /* Long Long Long Long Long Comment */
   // Long Long Long Long Long Comment
 
-  short3
+  short3,
   // More comments
 );
 
 differentArgTypes(
   () => {
     return true;
   },
-
-  isTrue ? doSomething() : 12
+  isTrue ? doSomething() : 12,
 );
 
 moreArgTypes(
   [1, 2, 3],
-
-  {
-    name: "Hello World",
-    age: 29,
-  },
-
+  { name: "Hello World", age: 29 },
   doSomething(
     // Hello world
 
     // Hello world again
     { name: "Hello World", age: 34 },
-
-    oneThing + anotherThing
-
+    oneThing + anotherThing,
     // Comment
-  )
+  ),
 );
 
 evenMoreArgTypes(
-  doSomething(
-    { name: "Hello World", age: 34 },
-
-    true
-  ),
-
+  doSomething({ name: "Hello World", age: 34 }, true),
   14,
-
-  1 + 2 - 90 / 80,
-
-  !98 * 60 - 90
+  1 + 2 - (90 / 80),
+  (!98 * 60) - 90,
 );
 
 foo.apply(
   null,
-
   // Array here
-  [1, 2]
+  [1, 2],
 );
 
 bar.on(
   "readable",
-
   () => {
     doStuff();
-  }
+  },
 );
 
 foo(
   ["A, B"],
-
   /* function here */
   function doSomething() {
     return true;
-  }
+  },
 );
 
 doSomething.apply(
   null,
-
   // Comment
 
-  ["Hello world 1", "Hello world 2", "Hello world 3"]
+  ["Hello world 1", "Hello world 2", "Hello world 3"],
 );
 
-doAnotherThing(
-  "node",
+doAnotherThing("node", { solution_type, time_frame });
 
-  {
-    solution_type,
-    time_frame,
-  }
-);
+stuff.doThing(someStuff, -1, { accept: (node) => doSomething(node) });
 
-stuff.doThing(
-  someStuff,
-
-  -1,
-  {
-    accept: (node) => doSomething(node),
-  }
-);
-
 doThing(
   someOtherStuff,
-
   // This is important
   true,
-  {
-    decline: (creditCard) => takeMoney(creditCard),
-  }
+  { decline: (creditCard) => takeMoney(creditCard) },
 );
 
 func(
   () => {
     thing();
   },
-
-  { yes: true, no: 5 }
+  { yes: true, no: 5 },
 );
 
 doSomething(
   { tomorrow: maybe, today: never[always] },
-
   1337,
-
   /* Comment */
 
   // This is important
-  { helloWorld, someImportantStuff }
+  { helloWorld, someImportantStuff },
 );
-
-function foo(
-  one,
 
-  two,
-  three,
-  four,
-
-  five,
-  six,
-  seven,
-  eight,
-  nine,
-  ten,
-
-  eleven
-) {}
+function foo(one, two, three, four, five, six, seven, eight, nine, ten, eleven) {}
```
# js/preserve-line/member-chain.js
```diff
 fooBar
   .doSomething("Hello World")
   .doAnotherThing("Foo", { foo: bar })
-
   // App configuration.
   .doOneMoreThing(config)
-
   .run(() => console.log("Bar"));
 
 bigDeal
-
   .doSomething("Hello World")
-
   // Hello world
   .doAnotherThing("Foo", { foo: bar })
-
   // App configuration.
   .doOneMoreThing(config)
-
   .run(() => console.log("Bar"));
 
 foo.bar.baz
-
   .doSomething("Hello World")
-
   // Hello world
   .foo.bar.doAnotherThing("Foo", { foo: bar })
-
   .doOneMoreThing(config)
   .bar.run(() => console.log("Bar"));
 
 (somethingGood ? thisIsIt : maybeNot)
-
   // Hello world
   .doSomething("Hello World")
-
   .doAnotherThing("Foo", { foo: bar }) // Run this
   .run(() => console.log("Bar")); // Do this
 
-helloWorld
-
-  .text()
-
-  .then((t) => t);
+helloWorld.text().then((t) => t);
 
 (
   veryLongVeryLongVeryLong ||
-  anotherVeryLongVeryLongVeryLong ||
-  veryVeryVeryLongError
-)
+    anotherVeryLongVeryLongVeryLong ||
+    veryVeryVeryLongError
+).map((tickets) => TicketRecord.createFromSomeLongString()).filter(
+  (obj) => !!obj,
+);
 
-  .map((tickets) => TicketRecord.createFromSomeLongString())
-
-  .filter((obj) => !!obj);
-
-const sel = this.connections
-
-  .concat(this.activities.concat(this.operators))
-  .filter((x) => x.selected);
+const sel = this.connections.concat(this.activities.concat(this.operators)).filter(
+  (x) => x.selected,
+);
```
# js/preserve-line/parameter-list.js
```diff
 class Foo {
-  constructor(
-    one,
-
-    two,
-    three,
-    four,
-
-    five,
-    six,
-    seven,
-    eight,
-    nine,
-    ten,
-
-    eleven
-  ) {}
+  constructor(one, two, three, four, five, six, seven, eight, nine, ten, eleven) {}
 }
-
-function foo(
-  one,
 
-  two,
-  three,
-  four,
-
-  five,
-  six,
-  seven,
-  eight,
-  nine,
-  ten,
+function foo(one, two, three, four, five, six, seven, eight, nine, ten, eleven) {}
 
-  eleven
-) {}
-
 call((a, b) => {});
 
 call((one, two, three, four, five, six, seven, eight, nine, ten, eleven) => {});
-
-call(
-  (
-    one,
-
-    two,
-    three,
-    four,
 
-    five,
-    six,
-    seven,
-    eight,
-    nine,
-    ten,
-
-    eleven
-  ) => {}
-);
-
-function test({
-  one,
-
-  two,
-  three,
-  four,
+call((one, two, three, four, five, six, seven, eight, nine, ten, eleven) => {});
 
-  five,
-  six,
-  seven,
-  eight,
-  nine,
-  ten,
-
-  eleven,
-}) {}
+function test(
+  { one, two, three, four, five, six, seven, eight, nine, ten, eleven },
+) {}
 
 function test({ one, two, three, four }) {}
 
-function test({
-  one,
-
-  two,
-  three,
-  four,
-}) {}
+function test({ one, two, three, four }) {}
 
 function test({ one, two, three, four }, $a) {}
 
-function test(
-  { one, two, three, four },
-
-  $a
-) {}
+function test({ one, two, three, four }, $a) {}
 
 function foo(...rest) {}
-
-function foo(
-  one,
 
-  ...rest
-) {}
+function foo(one, ...rest) {}
 
 function foo(one, ...rest) {}
 
 f(
   superSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperSuperLong,
-  ...args
+  ...args,
 );
 
-it("does something really long and complicated so I have to write a very long name for the test", function (done, foo) {
-  console.log("hello!");
-});
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  function (done, foo) {
+    console.log("hello!");
+  },
+);
```
# js/quotes/objects.js
```diff
-const obj = {
-  a: true,
-  b: true,
-  "𐊧": true,
-};
+const obj = { "a": true, b: true, "𐊧": true };
```
# js/quotes/strings.js
```diff
 // Prevent strings from being parsed as directives
 // See https://github.com/prettier/prettier/pull/1560#issue-227225960
 0;
 
 // Every string will be changed to double quotes, unless we end up with fewer
 // escaped quotes by using single quotes. (Vice versa if the "singleQuote"
 // option is true).
 //
 // Note that even if a string already has the correct enclosing quotes, it is
 // still processed in order to remove unnecessarily escaped quotes within it,
 // for consistency.
 
 // Simple strings.
 ("abc");
 ("abc");
 
 // Escape.
 ("\0");
 
 // Emoji.
 ("🐶");
 
 // Empty string.
 ("");
 ("");
 
 // Single double quote.
-('"');
+("\"");
 ('"');
 
 // Single single quote.
 ("'");
-("'");
+("\'");
 
 // Unnecessary escapes.
-("'");
-('"');
-("a");
-("a");
-("hola");
-("hola");
+("\'");
+('\"');
+("\a");
+("\a");
+("hol\a");
+("hol\a");
 ("hol\\a (the a is not escaped)");
 ("hol\\a (the a is not escaped)");
-("multiple a unnecessary a escapes");
-("multiple a unnecessary a escapes");
-("unnecessarily escaped character preceded by escaped backslash \\a");
-("unnecessarily escaped character preceded by escaped backslash \\a");
+("multiple \a unnecessary \a escapes");
+("multiple \a unnecessary \a escapes");
+("unnecessarily escaped character preceded by escaped backslash \\\a");
+("unnecessarily escaped character preceded by escaped backslash \\\a");
 ("unescaped character preceded by two escaped backslashes       \\\\a");
 ("unescaped character preceded by two escaped backslashes       \\\\a");
-("aa"); // consecutive unnecessarily escaped characters
-("aa"); // consecutive unnecessarily escaped characters
-("escaped \u2030 ‰ (should not stay escaped)");
+("\a\a"); // consecutive unnecessarily escaped characters
+("\a\a"); // consecutive unnecessarily escaped characters
+("escaped \u2030 \‰ (should not stay escaped)");
 
 // Meaningful escapes
 ("octal escapes \0 \1 \2 \3 \4 \5 \6 \7");
 ("octal escapes \0 \1 \2 \3 \4 \5 \6 \7");
 ("meaningfully escaped alphabetical characters \n \r \v \t \b \f \u2713 \x61");
 ("meaningfully escaped alphabetical characters \n \r \v \t \b \f \u2713 \x61");
 ("escaped newline \
 ");
 ("escaped carriage return \
 ");
 ("escaped \u2028 \ ");
 ("escaped \u2029 \ ");
 
 // One of each.
-("\"'");
 ("\"'");
+('"\'');
 
 // One of each with unnecessary escapes.
-("\"'");
-("\"'");
+("\"\'");
+('\"\'');
 
 // More double quotes than single quotes.
-('"\'"');
+("\"'\"");
 ('"\'"');
 
 // More single quotes than double quotes.
 ("\"''");
-("\"''");
+('"\'\'');
 
 // Two of each.
 ("\"\"''");
-("\"\"''");
+('""\'\'');
 
 // Single backslash.
 ("\\");
 ("\\");
 
 // Backslases.
-("\"\\\"\\\\\" ''\\'\\'\\\\'");
-('\'\\\'\\\\\' ""\\"\\"\\\\"');
+("\"\\\"\\\\\" '\'\\'\\\'\\\\'");
+('\'\\\'\\\\\' "\"\\"\\\"\\\\"');
 
 // Somewhat more real-word example.
 ("He's sayin': \"How's it goin'?\" Don't ask me why.");
-("He's sayin': \"How's it goin'?\" Don't ask me why.");
+('He\'s sayin\': "How\'s it goin\'?" Don\'t ask me why.');
 
 // Somewhat more real-word example 2.
-('var backslash = "\\", doubleQuote = \'"\';');
+("var backslash = \"\\\", doubleQuote = '\"';");
 ('var backslash = "\\", doubleQuote = \'"\';');
```
# js/range/array.js
```diff
-a = [, , , , , , , a];
+a = [
+,
+,
+,
+,
+,
+,
+
+,
+a,
+]
```
# js/range/boundary-2.js
```diff
 function a(
 ){
-  a();
-  b();
-  c();
-  d();
+a (
+);b();                 c (
+); d(
+);
+
 }
```
# js/range/boundary-3.js
```diff
 a (
-);
-b (
-);                 c (
+);b();                 c (
 ); d(
 );
```
# js/range/boundary.js
```diff
-foo = 1.0000;bar = 1.0;baz=1.0000;
+foo = 1.0000;bar = 1.0000;baz=1.0000;
 // The range will be 13~26
 // `foo` ends at 13, should not format
 // `bar` ends at 26, should format
```
# js/range/class-declaration.js
```diff
 
 
-class a {
+class    a {
   b() {}
 }
 
 let    x
```
# js/range/different-levels.js
```diff
 call(1,2,3)
-call(1, 2, 3);
+call(1,2, 3);
 function f() {
-  call(1, 2, 3);
+  call(1, 2,3)
 }
```
# js/range/directive.js
```diff
-"aaa";
+'aaa';
 'bbb';
```
# js/range/function-declaration.js
```diff
-function ugly({ a = 1, b = 2 }) {}
+function ugly ( { a = 1, b = 2 }      ) {}
```
# js/range/ignore-indentation.js
```diff
 function ugly ( {a=1,     b     =   2     }      ) {
   function ugly ( {a=1,     b     =   2     }      ) {
     function ugly ( {a=1,     b     =   2     }      ) {
   	  	     `multiline template string
-              with too much indentation`;
+              with too much indentation`
     }
   }
 }
```
# js/range/issue-3789-1.js
```diff
 export class F {
   reformatThis() {
     return 1;
   }
 
-  dontTouchThis() {
-    return 2;
+  dontTouchThis(){
+    return 2    ;
   }
 }
```
# js/range/issue-3789-2.js
```diff
 export class F {
   reformatThis() {
     return 1;
   }
 
-  dontTouchThis() {
-    return 2;
+  dontTouchThis(){
+    return 2    ;
   }
 }
```
# js/range/issue-4206-2.js
```diff
 export default function Foo() {
-  /**/
+/**/
 }
```
# js/range/issue-4206-4.js
```diff
-/* */ class Foo {
-  /**/
+/* */ class Foo{
+/**/
 }
```
# js/range/issue-7082.js
```diff
 export const Button = styled.button`
-  color: blue;
+color: blue;
 `;
```
# js/range/large-dict.js
```diff
 function ugly() {
   const dictWithSeveralEntries = {
-    key: "value",
+    key:          "value",
     anotherKey: "another value",
     firstNumber: 1,
-    secondNumber: 2,
+    secondNumber: 2
   };
 }
```
# js/range/module-export1.js
```diff
 import  def , {named}  from    'x'
 
-export * from "d";
+export *  from 'd'
 
 export    const  x
   =  42
 
 export   default    42
 
```
# js/range/module-export2.js
```diff
 import  def , {named}  from    'x'
 
 export *  from   'd'
 
-export const x = 42;
+export const x =  42
 
 export   default    42
 
```
# js/range/module-export3.js
```diff
 import  def , {named}  from    'x'
 
 export *  from   'd'
 
 export    const  x
   =  42
 
-export default 42;
+export   default    42
 
```
# js/range/module-import.js
```diff
-import def, { named } from "x";
+import  def, { named }  from    'x'
 
 export *  from   'd'
 
 export    const  x
   =  42
 
 export   default    42
 
```
# js/range/multiple-statements.js
```diff
 call(
   1, 2,3
 );
 
-call(1, 2, 3);
+call(
+  1, 2, 3);
 
-call(1, 2, 3);
+call(1, 2,3
+);
 
 call(
   1, 2,3
 );
```
# js/range/nested.js
```diff
 try {
   if (condition) {
     body;
   }
-} catch (err) {}
+}
+catch (err) {}
```
# js/range/object-expression.js
```diff
-const y = { a: 1, b: 2 };
+const y = {a: 1, b:2}
```
# js/range/object-expression2.js
```diff
 
-const y = [
-  {
-    a: 1,
-  },
-  {
-    a: 1,
-    b: 2,
-  },
-];
+const y =       [
+    {
+                a: 1,
+    },
+    { a: 1, b: 2 },
+]
```
# js/range/range.js
```diff
 function ugly ( {a=1,     b     =   2     }      ) {
   function ugly ( {a=1,     b     =   2     }      ) {
     function ugly ( {a=1,     b     =   2     }      ) {
              `multiline template string
-              with too much indentation`;
+              with too much indentation`
     }
   }
 }
```
# js/range/try-catch.js
```diff
-try {
-} catch (err) {}
+try {}
+catch (err) {}
```
# js/record/computed.js
```diff
 const key = "a";
-assert(#{ [key]: 1 } === #{ a: 1 });
-assert(#{ [key.toUpperCase()]: 1 } === #{ A: 1 });
+assert(#{ [key]: 1 } === #{ a: 1 })
+assert(#{ [key.toUpperCase()]: 1 } === #{ A: 1 })
 
-assert(#{ [true]: 1 } === #{ true: 1 });
-assert(#{ [true]: 1 } === #{ ["true"]: 1 });
+assert(#{ [true]: 1 } === #{ true: 1 })
+assert(#{ [true]: 1 } === #{ ["true"]: 1 })
 
-assert(#{ [1 + 1]: "two" } === #{ 2: "two" });
-assert(#{ [9 + 1]: "ten" } === #{ ["10"]: "ten" });
+assert(#{ [1 + 1]: "two" } === #{ 2: "two" })
+assert(#{ [9 + 1]: "ten" } === #{ ["10"]: "ten" })
```
# js/record/destructuring.js
```diff
-const { a, b } = #{ a: 1, b: 2 };
+const { a, b } = #
+{
+  a: 1, b;
+  : 2
+}
 assert(a === 1);
 assert(b === 2);
 
-const { a, ...rest } = #{ a: 1, b: 2, c: 3 };
+const { a, ...rest } = #
+{
+  a: 1, b;
+  : 2, c: 3
+}
 assert(a === 1);
 assert(typeof rest === "object");
 assert(rest.b === 2);
 assert(rest.c === 3);
```
# js/record/record.js
```diff
-const record1 = #{
-  a: 1,
-  b: 2,
-  c: 3,
-};
+const record1 = #
+{
+  a: 1, b;
+  : 2,
+    c: 3,
+}
 
-const record2 = #{ ...record1, b: 5 };
+const record2 = #
+{
+  ...record1, b: 5
+}
 
 assert(record1.a === 1);
 assert(record1["a"] === 1);
 assert(record1 !== record2);
 assert(record2 === #{ a: 1, c: 3, b: 5 });
 assert(record1?.a === 1);
 assert(record1?.d === undefined);
 assert(record1?.d ?? 5 === 5);
 assert(record1.d?.a === undefined);
```
# js/record/shorthand.js
```diff
 const url = "https://github.com/tc39/proposal-record-tuple";
-const record = #{ url };
+const record = #
+{
+  url;
+}
 console.log(record.url); // https://github.com/tc39/proposal-record-tuple
```
# js/record/spread.js
```diff
-const formData = #{ title: "Implement all the things" };
-const taskNow = #{ id: 42, status: "WIP", ...formData };
-const taskLater = #{ ...taskNow, status: "DONE" };
+const formData = #
+{
+  title: ("Implement all the things");
+}
+const taskNow = #
+{
+  id: 42, status;
+  : "WIP", ...formData
+}
+const taskLater = #
+{
+  ...taskNow, status: "DONE"
+}
 
 // A reminder: The ordering of keys in record literals does not affect equality (and is not retained)
-assert(taskLater === #{ status: "DONE", title: formData.title, id: 42 });
+assert(taskLater === #{ status: "DONE", title: formData.title, id: 42 })
```
# js/record/syntax.js
```diff
-#{};
-#{ a: 1, b: 2 };
-#{ a: 1, b: #[2, 3, #{ c: 4 }] };
+#
+{
+}
+#
+{
+  a: 1, b;
+  : 2
+}
+#
+{
+  a: 1, b;
+  : #[2, 3, #
+  {
+    c: 4;
+  }
+  ]
+}
```
# js/require-amd/named-amd-module.js
```diff
-define("foo/title", ["my/cart", "my/inventory"], function (cart, inventory) {
-  //Define foo/title object in here.
-});
+define(
+  "foo/title",
+  ["my/cart", "my/inventory"],
+  function (cart, inventory) {
+    //Define foo/title object in here.
+  },
+);
```
# js/require-amd/non-amd-define.js
```diff
 const someVariable = define(
   "some string literal",
   anotherVariable,
-  yetAnotherVariable
+  yetAnotherVariable,
 );
```
# js/require-amd/require.js
```diff
-require([
-  "jquery",
-  "common/global.context",
-  "common/log.event",
-  "some_project/square",
-  "some_project/rectangle",
-  "some_project/triangle",
-  "some_project/circle",
-  "some_project/star",
-], function (
-  $,
-  Context,
-  EventLogger,
-  Square,
-  Rectangle,
-  Triangle,
-  Circle,
-  Star
-) {
-  console.log("some code");
-});
+require(
+  [
+    "jquery",
+    "common/global.context",
+    "common/log.event",
+    "some_project/square",
+    "some_project/rectangle",
+    "some_project/triangle",
+    "some_project/circle",
+    "some_project/star",
+  ],
+  function ($, Context, EventLogger, Square, Rectangle, Triangle, Circle, Star) {
+    console.log("some code");
+  },
+);
 
-define([
-  "jquery",
-  "common/global.context",
-  "common/log.event",
-  "some_project/square",
-  "some_project/rectangle",
-  "some_project/triangle",
-  "some_project/circle",
-  "some_project/star",
-], function (
-  $,
-  Context,
-  EventLogger,
-  Square,
-  Rectangle,
-  Triangle,
-  Circle,
-  Star
-) {
-  console.log("some code");
-});
+define(
+  [
+    "jquery",
+    "common/global.context",
+    "common/log.event",
+    "some_project/square",
+    "some_project/rectangle",
+    "some_project/triangle",
+    "some_project/circle",
+    "some_project/star",
+  ],
+  function ($, Context, EventLogger, Square, Rectangle, Triangle, Circle, Star) {
+    console.log("some code");
+  },
+);
```
# js/require/require.js
```diff
-const {
-  one,
-  two,
-  three,
-  four,
-  five,
-  six,
-  seven,
-  eight,
-  nine,
-  ten,
-} = require("./my-utils");
+const { one, two, three, four, five, six, seven, eight, nine, ten } = require(
+  "./my-utils",
+);
 const {
   one1,
   two1,
   three1,
   four1,
   five1,
   six1,
   seven1,
   eight1,
   nine1,
   ten1,
   eleven1,
 } = require("./my-utils");
 
-const MyReallyExtrememlyLongModuleName = require("MyReallyExtrememlyLongModuleName");
+const MyReallyExtrememlyLongModuleName = require(
+  "MyReallyExtrememlyLongModuleName",
+);
```
# js/reserved-word/interfaces.js
```diff
 foo.interface;
 interface.foo;
 new interface();
 ({ interface: "foo" });
-interface, "foo";
+(interface, "foo");
 void interface;
 const interface = "foo";
```
# js/return-outside-function/return-outside-function.js
```diff
-return (
-  someVeryLongStringA &&
-  someVeryLongStringB &&
-  someVeryLongStringC &&
-  someVeryLongStringD
-);
+return someVeryLongStringA && someVeryLongStringB && someVeryLongStringC && someVeryLongStringD
```
# js/return/binaryish.js
```diff
 function f() {
   return (
     property.isIdentifier() &&
-    FUNCTIONS[property.node.name] &&
-    (object.isIdentifier(JEST_GLOBAL) ||
-      (callee.isMemberExpression() && shouldHoistExpression(object))) &&
-    FUNCTIONS[property.node.name](expr.get("arguments"))
+      FUNCTIONS[property.node.name] &&
+      (
+        object.isIdentifier(JEST_GLOBAL) || (
+          callee.isMemberExpression() && shouldHoistExpression(object)
+        )
+      ) &&
+      FUNCTIONS[property.node.name](expr.get("arguments"))
   );
 
   return (
-    chalk.bold("No tests found related to files changed since last commit.\n") +
-    chalk.dim(
-      patternInfo.watch
-        ? "Press `a` to run all tests, or run Jest with `--watchAll`."
-        : "Run Jest without `-o` to run all tests."
+    chalk.bold("No tests found related to files changed since last commit.\n") + chalk.dim(
+      patternInfo.watch ? "Press `a` to run all tests, or run Jest with `--watchAll`." : "Run Jest without `-o` to run all tests.",
     )
   );
 
-  return (
-    !filePath.includes(coverageDirectory) &&
-    !filePath.endsWith(`.${SNAPSHOT_EXTENSION}`)
+  return !filePath.includes(coverageDirectory) && !filePath.endsWith(
+    `.${SNAPSHOT_EXTENSION}`,
   );
 }
```
# js/return/comment.js
```diff
 function f() {
-  return /* a */;
+  return; /* a */
 }
 
 function f() {
   return; // a
 }
 
 function f() {
   return // a
-  /* b */;
+  /* b */ ;
 }
 
 function f() {
-  return; /* a */
+  return /* a */
   // b
+  ;
 }
 
 function x() {
-  return (
-    func2
-      //comment
-      .bar()
-  );
+  return func2
+  //comment
+  .bar();
 }
 
 function f() {
   return (
     foo
-      // comment
-      .bar()
+    // comment
+    .bar()
   );
 }
 
 fn(function f() {
   return (
     foo
-      // comment
-      .bar()
+    // comment
+    .bar()
   );
 });
```
# js/sequence-break/break.js
```diff
 const f = (argument1, argument2, argument3) => (
-  doSomethingWithArgument(argument1),
-  doSomethingWithArgument(argument2),
-  argument1
+  doSomethingWithArgument(argument1), doSomethingWithArgument(argument2), argument1
 );
 (function () {
   return (
-    aLongIdentifierName,
-    aLongIdentifierName,
-    aLongIdentifierName,
-    aLongIdentifierName
+    aLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName
   );
 });
-aLongIdentifierName,
-  aLongIdentifierName,
-  aLongIdentifierName,
-  aLongIdentifierName;
+aLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName;
 a.then(
   () => (
-    aLongIdentifierName,
-    aLongIdentifierName,
-    aLongIdentifierName,
-    aLongIdentifierName
-  )
+    aLongIdentifierName, aLongIdentifierName, aLongIdentifierName, aLongIdentifierName
+  ),
 );
 for (
-  aLongIdentifierName = 0,
-    aLongIdentifierName = 0,
-    aLongIdentifierName = 0,
-    aLongIdentifierName = 0;
+  aLongIdentifierName = 0, aLongIdentifierName = 0, aLongIdentifierName = 0, aLongIdentifierName =
+    0;
   test;
   update
 ) {}
-(a = b
-  ? c
-  : function () {
+(
+  a =
+    b ? c : function () {
+      return 0;
+    }
+), (
+  a =
+    b ? c : function () {
+      return 0;
+    }
+), (
+  a =
+    b ? c : function () {
       return 0;
-    }),
-  (a = b
-    ? c
-    : function () {
-        return 0;
-      }),
-  (a = b
-    ? c
-    : function () {
-        return 0;
-      }),
-  (a = b
-    ? c
-    : function () {
-        return 0;
-      }),
-  (a = b
-    ? c
-    : function () {
-        return 0;
-      });
+    }
+), (
+  a =
+    b ? c : function () {
+      return 0;
+    }
+), (
+  a =
+    b ? c : function () {
+      return 0;
+    }
+);
```
# js/shebang/shebang.js
```diff
 #!/usr/bin/env node
+
 function a() {}
```
# js/sloppy-mode/eval-arguments-binding.js
```diff
 function myfunc() {
-  var eval;
+  var eval
   var arguments;
 }
```
# js/sloppy-mode/function-declaration-in-if.js
```diff
-if (false) function foo() {}
+if (false) {
+  function foo(){}
+}
```
# js/sloppy-mode/function-declaration-in-while.js
```diff
-while (false) function foo() {}
+while (false) function foo(){}
```
# js/spread/spread.js
```diff
 const foo = { ...(a || b) };
-const foo2 = { ...(a || b) };
+const foo2 = { ...a || b };
 const foo3 = { ...(a ? b : c) };
 
 async () => ({ ...(await foo) });
```
# js/strings/non-octal-eight-and-nine.js
```diff
 // https://github.com/babel/babel/pull/11852
 
-"8", "9";
+"\8", "\9";
 () => {
   "use strict";
-  "8", "9";
+  "\8", "\9";
 };
```
# js/strings/strings.js
```diff
 [
   "abc",
   "abc",
 
-  "'",
+  "\'",
 
   '"',
-  '"',
+  '\"',
   '\\"',
 
   "'",
-  "'",
+  "\'",
   "\\'",
 
   "'\"",
-  "'\"",
+  '\'"',
 
   "\\",
   "\\",
 
   "\0",
   "🐶",
 
   "\uD801\uDC28",
 ];
```
# js/strings/template-literals.js
```diff
 foo(
   `a long string ${
-    1 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3 + 2 + 3
-  } with expr`
+    1 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3 +
+    2 +
+    3
+  } with expr`,
 );
 
 const x = `a long string ${
   1 +
   2 +
   3 +
   2 +
   3 +
   2 +
   3 +
   2 +
   3 +
   2 +
   3 +
   2 +
   (function () {
     return 3;
   })() +
   3 +
   2 +
   3 +
   2 +
   3
 } with expr`;
 
 foo(
   `a long string ${
     1 +
     2 +
     3 +
     2 +
     3 +
     2 +
     3 +
     2 +
     3 +
     2 +
     3 +
     2 +
     (function () {
       const x = 5;
 
       return x;
     })() +
     3 +
     2 +
     3 +
     2 +
     3
-  } with expr`
+  } with expr`,
 );
 
 pipe.write(
-  `\n  ${chalk.dim(
-    `\u203A and ${more} more ${more} more ${more} more ${more}`
-  )}`
+  `\n  ${chalk.dim(`\u203A and ${more} more ${more} more ${more} more ${more}`)}`,
 );
 
 // https://github.com/prettier/prettier/issues/1662#issue-230406820
 const content = `
 const env = ${JSON.stringify(
   {
     assetsRootUrl: env.assetsRootUrl,
     env: env.env,
     role: "client",
     adsfafa: "sdfsdff",
     asdfasff: "wefwefw",
     fefef: "sf sdfs fdsfdsf s dfsfds",
   },
   null,
-  "\t"
+  "\t",
 )});
 `;
 
 // https://github.com/prettier/prettier/issues/821#issue-210557749
-f(
-  `${{
-    a: 4,
-    b: 9,
-  }}`
-);
+f(`${{ a: 4, b: 9 }}`);
 
 // https://github.com/prettier/prettier/issues/1183#issue-220863505
 const makeBody = (store, assets, html) =>
   `<!doctype html>${ReactDOMServer.renderToStaticMarkup(
     <Html
       headScripts={compact([assets.javascript.head])}
       headStyles={compact([assets.styles.body, assets.styles.head])}
       bodyScripts={compact([assets.javascript.body])}
       bodyStyles={[]}
       stringScripts={[
-        `window.__INITIAL_STATE__ = ${JSON.stringify(
-          store.getState(),
-          null,
-          2
-        )};`,
+        `window.__INITIAL_STATE__ = ${JSON.stringify(store.getState(), null, 2)};`,
       ]}
       content={[
         { id: "app-container", dangerouslySetInnerHTML: { __html: html } },
       ]}
-    />
+    />,
   )}`;
 
 // https://github.com/prettier/prettier/issues/1626#issue-229655106
 const Bar = styled.div`
-  color: ${(props) =>
-    props.highlight.length > 0
-      ? palette(["text", "dark", "tertiary"])(props)
-      : palette(["text", "dark", "primary"])(props)} !important;
+  color: ${
+  (props) => (
+    props.highlight.length > 0 ? palette(["text", "dark", "tertiary"])(props) : palette([
+      "text", "dark", "primary",
+    ])(props)
+  )
+} !important;
 `;
```
# js/switch/comments.js
```diff
 switch (true) {
   case true:
   // Good luck getting here
 
   case false:
 }
 
 switch (true) {
   case true:
 
   // Good luck getting here
   case false:
 }
 
 switch (x) {
   case x: {
   }
 
   // other
 
   case y: {
   }
 }
 
 switch (x) {
-  default: // comment
+  default:
+    // comment
     break;
 }
 
 switch (x) {
   default: {
     // comment
     break;
   }
 }
 
 switch (x) {
   default: {
     // comment
     break;
   }
 }
 
 switch (x) {
-  default: /* comment */
+  default:
+    /* comment */
     break;
 }
 
 switch (x) {
-  default: /* comment */ {
+  default: {
+    /* comment */
     break;
   }
 }
 
 switch (x) {
   default: {
     /* comment */
     break;
   }
 }
 
 switch (x) {
-  default: /* comment */ {
+  default: {
+    /* comment */
     break;
   }
 }
```
# js/switch/empty_statement.js
```diff
 switch (error.code) {
   case ConfigurationEditingErrorCode.ERROR_INVALID_CONFIGURATION: {
     nls.localize(
       "errorInvalidConfiguration",
-      "Unable to write into settings. Correct errors/warnings in the file and try again."
+      "Unable to write into settings. Correct errors/warnings in the file and try again.",
     );
   }
 }
```
# js/switch/switch.js
```diff
 switch (a) {
   case 3:
     alert("3");
     break;
   case 4:
     alert("4");
     break;
   case 5:
     alert("5");
     break;
   default:
     alert("default");
 }
 
 switch (
   veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong
 ) {
   case 3:
     alert("3");
     break;
   default:
     alert("default");
 }
 
 switch (
-  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong >
-  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong
+  veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong > veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLong
 ) {
   case 3:
     alert("3");
     break;
   default:
     alert("default");
 }
 
 switch (
-  $veryLongAndVeryVerboseVariableName &&
-  $anotherVeryLongAndVeryVerboseVariableName
+  $veryLongAndVeryVerboseVariableName && $anotherVeryLongAndVeryVerboseVariableName
 ) {
 }
 
 switch (
-  $longButSlightlyShorterVariableName &&
-  $anotherSlightlyShorterVariableName
+  $longButSlightlyShorterVariableName && $anotherSlightlyShorterVariableName
 ) {
 }
```
# js/tab-width/class.js
```diff
 class A {
   method() {
     var x = 1;
     while (typeof x == "number" || typeof x == "string") {
       x = x + 1;
-      if (true) x = "";
+      if (true) {
+        x = "";
+      }
     }
     var z = x;
   }
 }
```
# js/template-align/indent.js
```diff
 `
 Mixed tabs and spaces:
 ${() => {
   a;
 }}
  	${() => {
-    a;
-  }}
+  a;
+}}
   	${() => {
-      a;
-    }}
+  a;
+}}
    	${() => {
-      a;
-    }}
+  a;
+}}
     	${() => {
-        a;
-      }}
+  a;
+}}
      	${() => {
-        a;
-      }}
+  a;
+}}
       	${() => {
-          a;
-        }}
+  a;
+}}
        	${() => {
-          a;
-        }}
+  a;
+}}
         	${() => {
-            a;
-          }}
+  a;
+}}
 
 Tabs:
 	${() => {
-    a;
-  }}
+  a;
+}}
 		${() => {
-      a;
-    }}
+  a;
+}}
 `;
```
# js/template-literals/binary-exporessions.js
```diff
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  1 | 2
-}`;
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  1 & 2
-}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${1 | 2}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${1 & 2}`;
```
# js/template-literals/css-prop.js
```diff
 function SomeComponent(props) {
   // Create styles as if you're calling css and the class will be applied to the component
   return (
-    <div
-      css={`
-        color: blue;
-        font-size: 17 px;
+    <div css={`
+    color: blue;
+    font-size: 17 px;
 
-        &:hover {
-          color: green;
-        }
+    &:hover {
+      color: green;
+    }
 
-        & .some-class {
-          font-size: 20px;
-        }
-      `}
-    >
-      This will be blue until hovered.
-      <div className="some-class">This font size will be 20px</div>
+    & .some-class {
+      font-size: 20px;
+    }
+  `}>
+    This will be blue until hovered.
+    <div className="some-class">
+      This font size will be 20px
     </div>
+  </div>
   );
 }
 
 const TestComponent = ({ children, ...props }) => (
-  <div
-    css={`
-      color: white;
-      background: black;
-    `}
-  >
+  <div css={`color: white; background: black`}>
     {children}
   </div>
 );
```
# js/template-literals/expressions.js
```diff
 const long1 = `long ${
   a.b //comment
-} long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long longlong ${
-  a.b.c.d.e
-} long long`;
+} long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long longlong ${a.b.c.d.e} long long`;
 const long2 = `long ${a.b.c.d.e} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long longlong ${loooooooooooooooooong} long long`;
 
 const long3 = `long long long long long long long long long long long ${a.b.c.d.e} long long long long long long long long long long long long long`;
 
 const description = `The value of the ${cssName} css of the ${this._name} element`;
 
 const foo = `such a long template string ${foo.bar.baz} that prettier will want to wrap it`;
 
-const shouldWrapForNow = `such a long template string ${
-  foo().bar.baz
-} that prettier will want to wrap it`;
+const shouldWrapForNow = `such a long template string ${foo().bar.baz} that prettier will want to wrap it`;
 
-const shouldNotWrap = `simple expressions should not break ${this} ${variable} ${a.b.c} ${this.b.c} ${a[b].c} ${a.b[c]} ${a.b["c"]} ${a?.b?.c}`;
+const shouldNotWrap = `simple expressions should not break ${this} ${variable} ${a.b.c} ${this.b.c} ${a[
+  b
+].c} ${a.b[c]} ${a.b["c"]} ${a?.b?.c}`;
 
 console.log(
   chalk.white(
-    `Covered Lines below threshold: ${coverageSettings.lines}%. Actual: ${coverageSummary.total.lines.pct}%`
-  )
+    `Covered Lines below threshold: ${coverageSettings.lines}%. Actual: ${coverageSummary.total.lines.pct}%`,
+  ),
 );
 
-x = `mdl-textfield mdl-js-textfield ${className} ${
-  content.length > 0 ? "is-dirty" : ""
-} combo-box__input`;
+x =
+  `mdl-textfield mdl-js-textfield ${className} ${
+    content.length > 0 ? "is-dirty" : ""
+  } combo-box__input`;
 
 function testing() {
   const p = {};
   // faking some tabs since I can't paste my real code in
   if (true) {
     if (false) {
-      return `${
-        process.env.OPENID_URL
-      }/something/something/something?${Object.keys(p)
+      return `${process.env.OPENID_URL}/something/something/something?${Object.keys(
+        p,
+      )
         .map((k) => `${encodeURIComponent(k)}=${encodeURIComponent(p[k])}`)
         .join("&")}`;
     }
   }
 }
 
 console.log(
-  `Trying update appcast for ${app.name} (${app.cask.appcast}) -> (${app.cask.appcastGenerated})`
+  `Trying update appcast for ${app.name} (${app.cask.appcast}) -> (${app.cask.appcastGenerated})`,
 );
 
 console.log(
-  `brew cask audit --download ${_.map(definitions, "caskName").join(" ")}`
+  `brew cask audit --download ${_.map(definitions, "caskName").join(" ")}`,
 );
 
 console.log(
-  `\nApparently jetbrains changed the release artifact for ${app.name}@${app.jetbrains.version}.\n`
+  `\nApparently jetbrains changed the release artifact for ${app.name}@${app.jetbrains.version}.\n`,
 );
 
-descirbe("something", () => {
-  test(`{pass: false} expect(${small}).toBeGreaterThanOrEqual(${big})`, () => {});
-});
+descirbe(
+  "something",
+  () => {
+    test(
+      `{pass: false} expect(${small}).toBeGreaterThanOrEqual(${big})`,
+      () => {},
+    );
+  },
+);
 
 throw new Error(
-  `pretty-format: Option "theme" has a key "${key}" whose value "${value}" is undefined in ansi-styles.`
+  `pretty-format: Option "theme" has a key "${key}" whose value "${value}" is undefined in ansi-styles.`,
 );
```
# js/template-literals/logical-expressions.js
```diff
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  1 ?? 2
-}`;
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  1 && 2
-}`;
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  1 || 2
-}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${1 ?? 2}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${1 && 2}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${1 || 2}`;
```
# js/template-literals/sequence-expressions.js
```diff
-`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${
-  (1, 2)
-}`;
+`111111111 222222222 333333333 444444444 555555555 666666666 777777777 ${(1, 2)}`;
```
# js/template-literals/styled-components-with-expressions.js
```diff
 const Button = styled.a`
-  /* Comment */
-  display: ${(props) => props.display};
+/* Comment */
+	display: ${(props) => props.display};
 `;
 
 styled.div`
-  display: ${(props) => props.display};
-  border: ${(props) => props.border}px;
-  margin: 10px ${(props) => props.border}px;
+	display: ${(props) => props.display};
+	border: ${(props) => props.border}px;
+	margin: 10px ${(props) => props.border}px ;
 `;
 
 const EqualDivider = styled.div`
-  margin: 0.5rem;
-  padding: 1rem;
-  background: papayawhip;
+margin: 0.5rem;
+		padding: 1rem;
+	background: papayawhip    ;
 
-  > * {
-    flex: 1;
+	> * {
+	flex: 1;
 
-    &:not(:first-child) {
-      ${(props) => (props.vertical ? "margin-top" : "margin-left")}: 1rem;
-    }
-  }
+	&:not(:first-child) {
+			${(props) => props.vertical ? "margin-top" : "margin-left"}: 1rem;
+		}
+	}
 `;
 
 const header = css`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
```
# js/template-literals/styled-jsx-with-expressions.js
```diff
 <style jsx>{`
   div {
-    display: ${expr};
+  display: ${expr};
     color: ${expr};
     ${expr};
     ${expr};
     background: red;
-    animation: ${expr} 10s ease-out;
+  animation: ${expr} 10s ease-out;
   }
   @media (${expr}) {
-    div.${expr} {
-      color: red;
-    }
-    ${expr} {
-      color: red;
-    }
+   div.${expr} {
+    color: red;
+   }
+  ${expr} {
+    color: red;
+  }
   }
   @media (min-width: ${expr}) {
-    div.${expr} {
-      color: red;
-    }
-    all${expr} {
-      color: red;
-    }
+   div.${expr} {
+    color: red;
+   }
+  all${expr} {
+    color: red;
+  }
   }
   @font-face {
     ${expr}
   }
 `}</style>;
 
 <style jsx>{`
   div {
-    animation: linear ${seconds}s ease-out;
+  animation: linear ${seconds}s ease-out;
   }
 `}</style>;
 
 <style jsx>{`
   div {
-    animation: 3s ease-in 1s ${(foo) => foo.getIterations()} reverse both paused
-      slidein;
+  animation: 3s ease-in 1s ${foo => foo.getIterations()} reverse both paused slidein;
   }
 `}</style>;
```
# js/template-literals/styled-jsx.js
```diff
 <style jsx>{`
-  /* a comment */
-  div :global(.react-select) {
-    color: red;
-    display: none;
-  }
+	/* a comment */
+	div :global(.react-select) {
+		color: red; display: none
+	}
 `}</style>;
 
 <div>
-  <style jsx>{`
-    /* a comment */
-    div :global(.react-select) {
-      color: red;
-      display: none;
-    }
-  `}</style>
+<style jsx>{`
+	/* a comment */
+div :global(.react-select) {
+color: red; display: none
+}`}</style>
 </div>;
 
 <div>
-  <style jsx>{`
-    div {
-      color: red;
-    }
-  `}</style>
+<style jsx>{`div{color:red}`}</style>
 </div>;
 
 <div>
-  <style jsx>{`This is invalid css. 
+<style jsx>{`This is invalid css. 
       Shouldn't fail.
             Shouldn't be formatted.`}</style>
 </div>;
 
 const header = css`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
 
 const headerResolve = css.resolve`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
 
 const headerGlobal = css.global`
-  .top-bar {
-    background: black;
-    margin: 0;
+.top-bar {background:black;
+margin: 0;
     position: fixed;
-    top: 0;
-    left: 0;
-    width: 100%;
-    text-align: center;
-    padding: 15px 0 0 1em;
-    z-index: 9999;
-  }
+	top: 0;left:0;
+	width: 100%;
+    text-align: center     ;
+	padding: 15px  0  0  1em;
+		z-index: 9999;
+}
 
-  .top-bar .logo {
-    height: 30px;
-    margin: auto;
+.top-bar .logo {
+  height: 30px;
+  margin: auto; 
     position: absolute;
-    left: 0;
-    right: 0;
-  }
+	left: 0;right: 0;
+}
 `;
```
# js/template/arrow.js
```diff
-() => a`
+() =>
+  a`
 	a
 `;
 
-() => `
+() =>
+  `
 	a
 `;
```
# js/template/call.js
```diff
-insertRule(`*, *:before, *:after {
+insertRule(
+  `*, *:before, *:after {
   box-sizing: inherit;
-}`);
+}`,
+);
 
 insertRule`*, *:before, *:after {
   box-sizing: inherit;
 }`;
 
-new Error(formatErrorMessage`
+new Error(
+  formatErrorMessage`
   This a really bad error.
   Which has more than one line.
-`);
+`,
+);
```
# js/template/comment.js
```diff
 `
 (?:${escapeChar}[\\S\\s]|(?:(?!${
   // Using `XRegExp.union` safely rewrites backreferences in `left` and `right`.
   // Intentionally not passing `basicFlags` to `XRegExp.union` since any syntax
   // transformation resulting from those flags was already applied to `left` and
   // `right` when they were passed through the XRegExp constructor above.
   XRegExp.union([left, right], "", { conjunction: "or" }).source
 })[^${escapeChar}])+)+
 `;
 
-`a${/* b */ c /* d */}e${
+`a${ /* b */ c /* d */ }e${
   // f
   g
   // h
 }`;
```
# js/template/faulty-locations.js
```diff
 var o = {
   [`key`]: () => {
     // Comment
   },
 };
 
 var x = {
-  y: () => Relay.QL`
+  y: () =>
+    Relay.QL`
     query {
       ${foo},
       field,
     }
   `,
 };
```
# js/template/graphql.js
```diff
-module.exports = Relay.createContainer(
-  // ...
-  {
-    fragments: {
-      nodes: ({ solution_type, time_frame }) => Relay.QL`
+module.exports =
+  Relay.createContainer(
+    // ...
+    {
+      fragments: {
+        nodes: ({ solution_type, time_frame }) =>
+          Relay.QL`
         fragment on RelatedNode @relay(plural: true) {
           __typename
-          ${OptimalSolutionsSection.getFragment("node", {
-            solution_type,
-            time_frame,
-          })}
+          ${OptimalSolutionsSection.getFragment(
+            "node",
+            { solution_type, time_frame },
+          )}
         }
       `,
+      },
     },
-  }
-);
+  );
```
# js/template/indent.js
```diff
 const foo = () => {
   {
     {
       {
         return `
 line 1
 line 2
 ...
 line n
-${foo({
-  many: keys,
-  many: keys,
-})}
+${foo({ many: keys, many: keys })}
 line n + 1
 line n + 2
 line n + n
 `;
       }
     }
   }
 };
```
# js/template/inline.js
```diff
 this._pipe.write(`\n\n Pattern matches ${total} ${pluralizeTest}`);
 this._pipe.write(`\n\n Pattern matches ${total} ${pluralizeTest}`);
 this._pipe.write(`\n\n Pattern matches ${total} ${pluralizeTest}`);
 
 this._pipe.write(
-  `\n\n Pattern matches ${total} ${pluralizeTest} but that's long`
+  `\n\n Pattern matches ${total} ${pluralizeTest} but that's long`,
 );
 
 this._pipe.write(
-  `\n\n Pattern matches ${total} ${pluralizeTest} but that's long`
+  `\n\n Pattern matches ${total} ${pluralizeTest} but that's long`,
 );
 
-this._pipe.write(`
+this._pipe.write(
+  `
   \n\n Pattern matches ${total} ${pluralizeTest} but that's long
-`);
+`,
+);
 
-() => `
+() =>
+  `
   a
 `;
 
 () =>
   `
     a
   `;
 
 // https://github.com/prettier/prettier/issues/5529
 editTitle += `${iconHTML({ class: "reply-to-glyph" })}`;
```
# js/template/parenthesis.js
```diff
 // "ArrowFunctionExpression"
 (() => {})``;
 
 // "AssignmentExpression"
 (b = c)``;
 
 // "AwaitExpression"
 async function f() {
   (await b)``;
 }
 
 // "BinaryExpression"
 (b + c)``;
 
 // "CallExpression"
 b()``;
 
 // "ClassExpression"
-(class {}``);
+(class {})``;
 
 // "ConditionalExpression"
 (b ? c : d)``;
 
 // "FunctionExpression"
 (function () {})``;
 
 // "LogicalExpression"
 (b || c)``;
 
 // "MemberExpression"
 b.c``;
 
 // "NewExpression"
-new B()``;
+(new B())``;
 
 // "ObjectExpression"
-({}``);
+({})``;
 
 // "SequenceExpression"
 (b, c)``;
 
 // "TaggedTemplateExpression"
-````;
+(``)``;
 
 // "UnaryExpression"
 (void b)``;
 
 // "UpdateExpression"
 (++b)``;
 
 // "YieldExpression"
 function* d() {
   (yield 1)``;
 }
```
# js/ternaries/binary.js
```diff
-const funnelSnapshotCard =
-  (report === MY_OVERVIEW && !ReportGK.xar_metrics_active_capitol_v2) ||
-  (report === COMPANY_OVERVIEW &&
-    !ReportGK.xar_metrics_active_capitol_v2_company_metrics) ? (
-    <ReportMetricsFunnelSnapshotCard metrics={metrics} />
-  ) : null;
+const funnelSnapshotCard = (
+  report === MY_OVERVIEW && !ReportGK.xar_metrics_active_capitol_v2
+) || (
+  report === COMPANY_OVERVIEW && !ReportGK.xar_metrics_active_capitol_v2_company_metrics
+) ? <ReportMetricsFunnelSnapshotCard metrics={metrics} /> : null;
 
-room = room.map((row, rowIndex) =>
-  row.map((col, colIndex) =>
-    rowIndex === 0 ||
-    colIndex === 0 ||
-    rowIndex === height ||
-    colIndex === width
-      ? 1
-      : 0
-  )
-);
+room =
+  room.map(
+    (row, rowIndex) => (
+      row.map(
+        (col, colIndex) => (
+          (
+            rowIndex === 0 ||
+              colIndex === 0 ||
+              rowIndex === height ||
+              colIndex === width
+          ) ? 1 : 0
+        ),
+      )
+    ),
+  );
```
# js/ternaries/func-call.js
```diff
 fn(
   bifornCringerMoshedPerplexSawder,
   askTrovenaBeenaDependsRowans,
-  glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl &&
-    anodyneCondosMalateOverateRetinol
-    ? annularCooeedSplicesWalksWayWay
-    : kochabCooieGameOnOboleUnweave
+  glimseGlyphsHazardNoopsTieTie === averredBathersBoxroomBuggyNurl && anodyneCondosMalateOverateRetinol ? annularCooeedSplicesWalksWayWay : kochabCooieGameOnOboleUnweave,
 );
```
# js/ternaries/indent-after-paren.js
```diff
-foo7 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)[Fooooooooooo];
+foo7 =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )[Fooooooooooo];
 
 foo8 = (condition ? firstValue : secondValue)[SomeType];
 
 const foo9 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 )[Fooooooooooo];
 
 function foo10() {
   return (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )[Fooooooooooo];
 }
 
 function foo11() {
   throw (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )[Fooooooooooo];
 }
 
 function foo12() {
   void (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )[Fooooooooooo];
 }
 
-foo13 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-).Fooooooooooo.Fooooooooooo;
+foo13 =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  ).Fooooooooooo.Fooooooooooo;
 
 foo14 = (condition ? firstValue : secondValue)[SomeType];
 
 const foo15 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 ).Fooooooooooo.Fooooooooooo;
 
 function foo16() {
   return (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ).Fooooooooooo.Fooooooooooo;
 }
 
 function foo17() {
   throw (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ).Fooooooooooo.Fooooooooooo;
 }
 
 function foo18() {
   void (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ).Fooooooooooo.Fooooooooooo;
 }
 
-foo19 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)(Fooooooooooo.Fooooooooooo);
+foo19 =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )(Fooooooooooo.Fooooooooooo);
 
 foo20 = (condition ? firstValue : secondValue)[SomeType];
 
 const foo21 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 )(Fooooooooooo.Fooooooooooo);
 
 function foo22() {
   return (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
 function foo23() {
   throw (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
 function foo24() {
   void (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
-foo25 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)?.(Fooooooooooo.Fooooooooooo);
+foo25 =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )?.(Fooooooooooo.Fooooooooooo);
 
 foo26 = (condition ? firstValue : secondValue)[SomeType];
 
 const foo27 = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 )?.(Fooooooooooo.Fooooooooooo);
 
 function foo28() {
   return (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )?.(Fooooooooooo.Fooooooooooo);
 }
 
 function foo29() {
   throw (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )?.(Fooooooooooo.Fooooooooooo);
 }
 
 function foo30() {
   void (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )?.(Fooooooooooo.Fooooooooooo);
 }
 
 function* foo31() {
   yield (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )?.(Fooooooooooo.Fooooooooooo);
   yield (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
   yield (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ).Fooooooooooo.Fooooooooooo;
   yield (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )[Fooooooooooo.Fooooooooooo];
 }
 
 const foo32 = new (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 )(Fooooooooooo.Fooooooooooo);
 
 function foo33() {
   return new (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
 function foo34() {
   throw new (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
 function foo35() {
   void new (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   )(Fooooooooooo.Fooooooooooo);
 }
 
-foo36 = new (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)(Fooooooooooo.Fooooooooooo);
+foo36 =
+  new (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )(Fooooooooooo.Fooooooooooo);
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol)[AnnularCooeedSplicesWalksWayWay];
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    )[AnnularCooeedSplicesWalksWayWay]
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol)[AnnularCooeedSplicesWalksWayWay];
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    )[AnnularCooeedSplicesWalksWayWay]
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).Fooooooooooo.Fooooooooooo;
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ).Fooooooooooo.Fooooooooooo
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).Fooooooooooo.Fooooooooooo;
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ).Fooooooooooo.Fooooooooooo
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol)(Fooooooooooo.Fooooooooooo);
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    )(Fooooooooooo.Fooooooooooo)
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0 &&
-    kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol)(Fooooooooooo.Fooooooooooo);
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    )(Fooooooooooo.Fooooooooooo)
+  );
 
-bifornCringerMoshedPerplexSawder = (
-  glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-).annularCooeedSplicesWalksWayWay
-  .annularCooeedSplicesWalksWayWay(annularCooeedSplicesWalksWayWay)
-  .annularCooeedSplicesWalksWayWay();
+bifornCringerMoshedPerplexSawder =
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+  ).annularCooeedSplicesWalksWayWay.annularCooeedSplicesWalksWayWay(
+    annularCooeedSplicesWalksWayWay,
+  ).annularCooeedSplicesWalksWayWay();
 
-foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)?.()?.()?.();
+foo =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )?.()?.()?.();
 
-foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)()()();
+foo =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )()()();
 
 foo =
   foo.bar.baz[
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ];
 
 const decorated = (arg, ignoreRequestError) => {
   return (
-    typeof arg === "string" ||
-    (arg && arg.valueOf && typeof arg.valueOf() === "string")
-      ? $delegate(arg, ignoreRequestError)
-      : handleAsyncOperations(arg, ignoreRequestError)
+    typeof arg === "string" || (
+      arg && arg.valueOf && typeof arg.valueOf() === "string"
+    ) ? $delegate(arg, ignoreRequestError) : handleAsyncOperations(
+      arg,
+      ignoreRequestError,
+    )
   ).foo();
 };
 
-bifornCringerMoshedPerplexSawder = fn(
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).prop
-);
+bifornCringerMoshedPerplexSawder =
+  fn(
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ).prop,
+  );
 
 fn(
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).prop
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+  ).prop,
 );
 
-bifornCringerMoshedPerplexSawder = fn?.(
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).prop
-);
+bifornCringerMoshedPerplexSawder =
+  fn?.(
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ).prop,
+  );
 
 fn?.(
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-  ).prop
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+  ).prop,
 );
 
 bifornCringerMoshedPerplexSawder =
   fn[
-    (glimseGlyphsHazardNoopsTieTie === 0
-      ? averredBathersBoxroomBuggyNurl
-      : anodyneCondosMalateOverateRetinol
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
     ).prop
   ];
 
 fn[
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
   ).prop
 ];
 
 bifornCringerMoshedPerplexSawder =
   fn?.[
-    (glimseGlyphsHazardNoopsTieTie === 0
-      ? averredBathersBoxroomBuggyNurl
-      : anodyneCondosMalateOverateRetinol
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
     ).prop
   ];
 
 fn?.[
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
   ).prop
 ];
```
# js/ternaries/indent.js
```diff
 aaaaaaaaaaaaaaa
   ? bbbbbbbbbbbbbbbbbb
   : ccccccccccccccc
-  ? ddddddddddddddd
-  : eeeeeeeeeeeeeee
-  ? fffffffffffffff
-  : gggggggggggggggg;
+    ? ddddddddddddddd
+    : eeeeeeeeeeeeeee
+      ? fffffffffffffff
+      : gggggggggggggggg;
 
 aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   ? aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
     ? aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
       ? aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
       : aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
     : aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   : aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa;
 
-a
-  ? {
-      a: 0,
-    }
-  : {
-      a: {
-        a: 0,
-      }
-        ? {
-            a: 0,
-          }
-        : {
-            y: {
-              a: 0,
-            }
-              ? {
-                  a: 0,
-                }
-              : {
-                  a: 0,
-                },
-          },
-    };
+a ? { a: 0 } : {
+  a: { a: 0 } ? { a: 0 } : { y: { a: 0 } ? { a: 0 } : { a: 0 } },
+};
 
-a
-  ? {
-      a: function () {
-        return a
-          ? {
-              a: [
-                a
-                  ? {
-                      a: 0,
-                      b: [a ? [0, 1] : []],
-                    }
-                  : [
-                      [
-                        0,
-                        {
-                          a: 0,
-                        },
-                        a ? 0 : 1,
-                      ],
-                      function () {
-                        return a
-                          ? {
-                              a: 0,
-                            }
-                          : [
-                              {
-                                a: 0,
-                              },
-                              {},
-                            ];
-                      },
-                    ],
-              ],
-            }
-          : [
-              a
-                ? function () {
-                    a
-                      ? a(
-                          a
-                            ? {
-                                a: a({
-                                  a: 0,
-                                }),
-                              }
-                            : [
-                                0,
-                                a(),
-                                a(
-                                  a(),
-                                  {
-                                    a: 0,
-                                  },
-                                  a
-                                    ? a()
-                                    : a({
-                                        a: 0,
-                                      })
-                                ),
-                                a()
-                                  ? {
-                                      a: a(),
-                                      b: [],
-                                    }
-                                  : {},
-                              ]
-                        )
-                      : a(
-                          a()
-                            ? {
-                                a: 0,
-                              }
-                            : (function (a) {
-                                return a()
-                                  ? [
-                                      {
-                                        a: 0,
-                                        b: a(),
-                                      },
-                                    ]
-                                  : a([
-                                      a
-                                        ? {
-                                            a: 0,
-                                          }
-                                        : {},
-                                      {
-                                        a: 0,
-                                      },
-                                    ]);
-                              })(
-                                a
-                                  ? function (a) {
-                                      return function () {
-                                        return 0;
-                                      };
-                                    }
-                                  : function (a) {
-                                      return function () {
-                                        return 1;
-                                      };
-                                    }
-                              )
-                        );
-                  }
-                : function () {},
-            ];
-      },
-    }
-  : a;
+a ? {
+  a: function () {
+    return a ? {
+      a: [
+        a ? { a: 0, b: [a ? [0, 1] : []] } : [
+          [0, { a: 0 }, a ? 0 : 1],
+          function () {
+            return a ? { a: 0 } : [{ a: 0 }, {}];
+          },
+        ],
+      ],
+    } : [
+      a ? function () {
+        a ? a(
+          a ? { a: a({ a: 0 }) } : [
+            0,
+            a(),
+            a(a(), { a: 0 }, a ? a() : a({ a: 0 })),
+            a() ? { a: a(), b: [] } : {},
+          ],
+        ) : a(
+          a() ? { a: 0 } : (function (a) {
+            return a() ? [{ a: 0, b: a() }] : a([a ? { a: 0 } : {}, { a: 0 }]);
+          })(
+            a ? function (a) {
+              return function () {
+                return 0;
+              };
+            } : function (a) {
+              return function () {
+                return 1;
+              };
+            },
+          ),
+        );
+      } : function () {},
+    ];
+  },
+} : a;
```
# js/ternaries/nested-in-condition.js
```diff
-$var = (
-  $number % 10 >= 2 && ($number % 100 < 10 || $number % 100 >= 20)
-    ? kochabCooieGameOnOboleUnweave
-    : annularCooeedSplicesWalksWayWay
-)
-  ? anodyneCondosMalateOverateRetinol
-  : averredBathersBoxroomBuggyNurl;
+$var =
+  (
+    ($number % 10) >= 2 && (($number % 100) < 10 || ($number % 100) >= 20) ? kochabCooieGameOnOboleUnweave : annularCooeedSplicesWalksWayWay
+  ) ? anodyneCondosMalateOverateRetinol : averredBathersBoxroomBuggyNurl;
 
 const value = (
-  bifornCringerMoshedPerplexSawder
-    ? askTrovenaBeenaDependsRowans
-    : glimseGlyphsHazardNoopsTieTie
+  bifornCringerMoshedPerplexSawder ? askTrovenaBeenaDependsRowans : glimseGlyphsHazardNoopsTieTie
 )
   ? true
     ? true
     : false
   : true
-  ? true
-  : false;
+    ? true
+    : false;
 
 (
-  bifornCringerMoshedPerplexSawder
-    ? askTrovenaBeenaDependsRowans
-    : glimseGlyphsHazardNoopsTieTie
+  bifornCringerMoshedPerplexSawder ? (askTrovenaBeenaDependsRowans) : (
+    glimseGlyphsHazardNoopsTieTie
+  )
 ) ? (
   <Element>
     <Sub />
     <Sub />
     <Sub />
     <Sub />
     <Sub />
     <Sub />
   </Element>
 ) : (
   <Element2>
     <Sub />
     <Sub />
     <Sub />
   </Element2>
 );
```
# js/ternaries/nested.js
```diff
-let icecream =
-  what == "cone"
-    ? (p) => (!!p ? `here's your ${p} cone` : `just the empty cone for you`)
-    : (p) => `here's your ${p} ${what}`;
+let icecream = what == "cone" ? (p) =>
+  !!p ? `here's your ${p} cone` : `just the empty cone for you` : (p) =>
+  `here's your ${p} ${what}`;
 
 const value = condition1
   ? value1
   : condition2
-  ? value2
-  : condition3
-  ? value3
-  : value4;
+    ? value2
+    : condition3
+      ? value3
+      : value4;
 
-const StorybookLoader = ({ match }) =>
-  match.params.storyId === "button" ? (
-    <ButtonStorybook />
-  ) : match.params.storyId === "color" ? (
-    <ColorBook />
-  ) : match.params.storyId === "typography" ? (
-    <TypographyBook />
-  ) : match.params.storyId === "loading" ? (
-    <LoaderStorybook />
-  ) : match.params.storyId === "deal-list" ? (
-    <DealListStory />
-  ) : (
-    <Message>
-      <Title>{"Missing story book"}</Title>
-      <Content>
-        <BackButton />
-      </Content>
-    </Message>
-  );
+const StorybookLoader = ({ match }) => (
+  match.params.storyId === "button"
+    ? <ButtonStorybook />
+    : match.params.storyId === "color"
+      ? <ColorBook />
+      : match.params.storyId === "typography"
+        ? <TypographyBook />
+        : match.params.storyId === "loading"
+          ? <LoaderStorybook />
+          : match.params.storyId === "deal-list"
+            ? <DealListStory />
+            : (
+              <Message>
+        <Title>{'Missing story book'}</Title>
+        <Content>
+          <BackButton/>
+        </Content>
+      </Message>
+            )
+);
 
-const message =
-  i % 3 === 0 && i % 5 === 0
-    ? "fizzbuzz"
-    : i % 3 === 0
+const message = (i % 3) === 0 && (i % 5) === 0
+  ? "fizzbuzz"
+  : (i % 3) === 0
     ? "fizz"
-    : i % 5 === 0
-    ? "buzz"
-    : String(i);
+    : (i % 5) === 0
+      ? "buzz"
+      : String(i);
 
-const paymentMessage =
-  state == "success"
-    ? "Payment completed successfully"
-    : state == "processing"
+const paymentMessage = state == "success"
+  ? "Payment completed successfully"
+  : state == "processing"
     ? "Payment processing"
     : state == "invalid_cvc"
-    ? "There was an issue with your CVC number"
-    : state == "invalid_expiry"
-    ? "Expiry must be sometime in the past."
-    : "There was an issue with the payment.  Please contact support.";
+      ? "There was an issue with your CVC number"
+      : state == "invalid_expiry"
+        ? "Expiry must be sometime in the past."
+        : "There was an issue with the payment.  Please contact support.";
 
-const paymentMessage2 =
-  state == "success"
-    ? 1 //'Payment completed successfully'
-    : state == "processing"
+const paymentMessage2 = state == "success"
+  ? 1 //'Payment completed successfully'
+  : state == "processing"
     ? 2 //'Payment processing'
     : state == "invalid_cvc"
-    ? 3 //'There was an issue with your CVC number'
-    : true //state == 'invalid_expiry'
-    ? 4 //'Expiry must be sometime in the past.'
-    : 5; // 'There was an issue with the payment.  Please contact support.'
+      ? 3 //'There was an issue with your CVC number'
+      : true
+        //state == 'invalid_expiry'
+        ? 4 //'Expiry must be sometime in the past.'
+        : 5; // 'There was an issue with the payment.  Please contact support.'
 
-const foo = (
-  <div
-    className={
-      "match-achievement-medal-type type" +
-      (medals[0].record
-        ? "-record"
-        : medals[0].unique
-        ? "-unique"
-        : medals[0].type)
-    }
-  >
-    {medals[0].record
-      ? i18n("Record")
-      : medals[0].unique
-      ? i18n("Unique")
-      : medals[0].type === 0
-      ? i18n("Silver")
-      : medals[0].type === 1
-      ? i18n("Gold")
-      : medals[0].type === 2
-      ? i18n("Platinum")
-      : i18n("Theme")}
-  </div>
-);
+const foo = <div className={'match-achievement-medal-type type' + (medals[0].record ? '-record' : (medals[0].unique ? '-unique' : medals[0].type))}>
+	{medals[0].record ? (
+		i18n('Record')
+	) : medals[0].unique ? (
+		i18n('Unique')
+	) : medals[0].type === 0 ? (
+		i18n('Silver')
+	) : medals[0].type === 1 ? (
+		i18n('Gold')
+	) : medals[0].type === 2 ? (
+		i18n('Platinum')
+	) : (
+		i18n('Theme')
+	)}
+</div>;
 
 a
   ? literalline
-  : {
-      123: 12,
-    }
-  ? line
-  : softline;
+  : { 123: 12 }
+    ? line
+    : softline;
```
# js/ternaries/parenthesis.js
```diff
-debug ? (this.state.isVisible ? "partially visible" : "hidden") : null;
 debug
+  ? this.state.isVisible
+    ? "partially visible"
+    : "hidden"
+  : null;
+debug
   ? this.state.isVisible && somethingComplex
     ? "partially visible"
     : "hidden"
   : null;
 
 (a) =>
-  a
-    ? () => {
-        a;
-      }
-    : () => {
-        a;
-      };
-(a) => (a ? a : a);
+  a ? () => {
+    a;
+  } : () => {
+    a;
+  };
+(a) => a ? a : a;
 (a) =>
   a ? aasdasdasdasdasdasdaaasdasdasdasdasdasdasdasdasdasdasdasdasdaaaaaa : a;
```
# js/ternaries/test.js
```diff
 const obj0 = conditionIsTruthy ? shortThing : otherShortThing;
 
-const obj1 = conditionIsTruthy
-  ? { some: "long", object: "with", lots: "of", stuff }
-  : shortThing;
+const obj1 = conditionIsTruthy ? {
+  some: "long",
+  object: "with",
+  lots: "of",
+  stuff,
+} : shortThing;
 
-const obj2 = conditionIsTruthy
-  ? shortThing
-  : { some: "long", object: "with", lots: "of", stuff };
+const obj2 = conditionIsTruthy ? shortThing : {
+  some: "long",
+  object: "with",
+  lots: "of",
+  stuff,
+};
 
-const obj3 = conditionIsTruthy
-  ? {
-      some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
-      object: "with",
-      lots: "of",
-      stuff,
-    }
-  : shortThing;
+const obj3 = conditionIsTruthy ? {
+  some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
+  object: "with",
+  lots: "of",
+  stuff,
+} : shortThing;
 
-const obj4 = conditionIsTruthy
-  ? shortThing
-  : {
-      some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
-      object: "with",
-      lots: "of",
-      stuff,
-    };
+const obj4 = conditionIsTruthy ? shortThing : {
+  some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
+  object: "with",
+  lots: "of",
+  stuff,
+};
 
-const obj5 = conditionIsTruthy
-  ? { some: "long", object: "with", lots: "of", stuff }
-  : {
-      some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
-      object: "with",
-      lots: "of",
-      stuff,
-    };
+const obj5 = conditionIsTruthy ? {
+  some: "long",
+  object: "with",
+  lots: "of",
+  stuff,
+} : {
+  some: "eeeeeeeeeeeeven looooooooooooooooooooooooooooooonger",
+  object: "with",
+  lots: "of",
+  stuff,
+};
```
# js/test-declarations/angular_async.js
```diff
-beforeEach(async(() => {
-  // code
-}));
+beforeEach(
+  async(() => {
+    // code
+  }),
+);
 
 beforeEach((done) => foo().bar().bar());
 
-afterAll(async(() => {
-  console.log("Hello");
-}));
+afterAll(
+  async(() => {
+    console.log("Hello");
+  }),
+);
 
 afterAll((done) => foo().bar().bar());
 
-it("should create the app", async(() => {
-  //code
-}));
+it(
+  "should create the app",
+  async(() => {
+    //code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", async(() => {
-  // code
-}));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  async(() => {
+    // code
+  }),
+);
 
 /*
- * isTestCall(parent) should only be called when parent exists
- * and parent.type is CallExpression. This test makes sure that
- * no errors are thrown when calling isTestCall(parent)
- */
+* isTestCall(parent) should only be called when parent exists
+* and parent.type is CallExpression. This test makes sure that
+* no errors are thrown when calling isTestCall(parent)
+*/
 function x() {
   async(() => {});
 }
```
# js/test-declarations/angular_fakeAsync.js
```diff
-beforeEach(fakeAsync(() => {
-  // code
-}));
+beforeEach(
+  fakeAsync(() => {
+    // code
+  }),
+);
 
-afterAll(fakeAsync(() => {
-  console.log("Hello");
-}));
+afterAll(
+  fakeAsync(() => {
+    console.log("Hello");
+  }),
+);
 
-it("should create the app", fakeAsync(() => {
-  //code
-}));
+it(
+  "should create the app",
+  fakeAsync(() => {
+    //code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", fakeAsync(() => {
-  // code
-}));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  fakeAsync(() => {
+    // code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", fakeAsync(() =>
-  new SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS()));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  fakeAsync(
+    () =>
+      new SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(),
+  ),
+);
 
 /*
- * isTestCall(parent) should only be called when parent exists
- * and parent.type is CallExpression. This test makes sure that
- * no errors are thrown when calling isTestCall(parent)
- */
+* isTestCall(parent) should only be called when parent exists
+* and parent.type is CallExpression. This test makes sure that
+* no errors are thrown when calling isTestCall(parent)
+*/
 function x() {
   fakeAsync(() => {});
 }
```
# js/test-declarations/angular_waitForAsync.js
```diff
-beforeEach(waitForAsync(() => {
-  // code
-}));
+beforeEach(
+  waitForAsync(() => {
+    // code
+  }),
+);
 
-afterAll(waitForAsync(() => {
-  console.log("Hello");
-}));
+afterAll(
+  waitForAsync(() => {
+    console.log("Hello");
+  }),
+);
 
-it("should create the app", waitForAsync(() => {
-  //code
-}));
+it(
+  "should create the app",
+  waitForAsync(() => {
+    //code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", waitForAsync(() => {
-  // code
-}));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  waitForAsync(() => {
+    // code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", waitForAsync(() =>
-  new SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS()));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  waitForAsync(
+    () =>
+      new SSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSSS(),
+  ),
+);
 
 /*
- * isTestCall(parent) should only be called when parent exists
- * and parent.type is CallExpression. This test makes sure that
- * no errors are thrown when calling isTestCall(parent)
- */
+* isTestCall(parent) should only be called when parent exists
+* and parent.type is CallExpression. This test makes sure that
+* no errors are thrown when calling isTestCall(parent)
+*/
 function x() {
   waitForAsync(() => {});
 }
```
# js/test-declarations/angularjs_inject.js
```diff
-beforeEach(inject(($fooService, $barService) => {
-  // code
-}));
+beforeEach(
+  inject(
+    ($fooService, $barService) => {
+      // code
+    },
+  ),
+);
 
-afterAll(inject(($fooService, $barService) => {
-  console.log("Hello");
-}));
+afterAll(
+  inject(
+    ($fooService, $barService) => {
+      console.log("Hello");
+    },
+  ),
+);
 
-it("should create the app", inject(($fooService, $barService) => {
-  //code
-}));
+it(
+  "should create the app",
+  inject(
+    ($fooService, $barService) => {
+      //code
+    },
+  ),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", inject(() => {
-  // code
-}));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  inject(() => {
+    // code
+  }),
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", inject((
-  $fooServiceLongName,
-  $barServiceLongName
-) => {
-  // code
-}));
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  inject(
+    ($fooServiceLongName, $barServiceLongName) => {
+      // code
+    },
+  ),
+);
 
 /*
- * isTestCall(parent) should only be called when parent exists
- * and parent.type is CallExpression. This test makes sure that
- * no errors are thrown when calling isTestCall(parent)
- */
+* isTestCall(parent) should only be called when parent exists
+* and parent.type is CallExpression. This test makes sure that
+* no errors are thrown when calling isTestCall(parent)
+*/
 function x() {
   inject(() => {});
 }
```
# js/test-declarations/jest-each-template-string.js
```diff
 test.each`
-  a    | b                                                 | c
-  ${1} | ${[{ start: 5, end: 15 }]}                        | ${[1, 2, 3, 4, 5, 6, 7, 8]}
-  ${1} | ${[{ start: 5, end: 15 }]}                        | ${["test", "string", "for", "prettier"]}
-  ${3} | ${[{ start: 5, end: 15 }]}                        | ${[]}
-  ${4} | ${[{ start: 1, end: 3 }, { start: 15, end: 20 }]} | ${[]}
+a | b         | c
+${1}      | ${[{ start: 5, end: 15 }]} | ${[1, 2, 3, 4, 5, 6, 7, 8]}
+${1}| ${[{ start: 5, end: 15 }]} | ${["test", "string", "for", "prettier"]}
+${3}      | ${[{ start: 5, end: 15 }]} | ${[]}
+${4} | ${[{ start: 1, end: 3 }, { start: 15, end: 20 }]} | ${[]}
 `("example test", ({ a, b, c }) => {});
 
 test.each`
-  a                                                                         |
-  ${[{ a: 1, b: 3 }, { c: 15, d: 20 }]}
-  ${[{ start: 1, end: 3 }, { start: 15, end: 20 }, { start: 15, end: 20 }]}
+a | 
+${[{ a: 1, b: 3 }, { c: 15, d: 20 }]}| 
+${[{ start: 1, end: 3 }, { start: 15, end: 20 }, { start: 15, end: 20 }]}| 
 `("example test", ({ a, b, c }) => {});
```
# js/test-declarations/jest-each.js
```diff
 describe.each`
-  a            | b        | expected
-  ${11}        | ${1}     | ${222}
-  ${1 - 1}     | ${2 + 2} | ${3333}
-  ${2 + 1 + 2} | ${1111}  | ${3}
-`("$a + $b", ({ a, b, expected }) => {
-  test(`returns ${expected}`, () => {
-    expect(a + b).toBe(expected);
-  });
+a|b|expected
+${11} | ${1}|${222}
+${1 - 1}|${2 + 2}|${3333}
+${2 + 1 + 2}|${1111}|${3}
+`(
+  "$a + $b",
+  ({ a, b, expected }) => {
+    test(
+      `returns ${expected}`,
+      () => {
+        expect(a + b).toBe(expected);
+      },
+    );
 
-  test(`returned value not be greater than ${expected}`, () => {
-    expect(a + b).not.toBeGreaterThan(expected);
-  });
+    test(
+      `returned value not be greater than ${expected}`,
+      () => {
+        expect(a + b).not.toBeGreaterThan(expected);
+      },
+    );
 
-  test(`returned value not be less than ${expected}`, () => {
-    expect(a + b).not.toBeLessThan(expected);
-  });
-});
+    test(
+      `returned value not be less than ${expected}`,
+      () => {
+        expect(a + b).not.toBeLessThan(expected);
+      },
+    );
+  },
+);
 
 describe.only.each`
-  a            | b        | expected
-  ${11}        | ${1}     | ${222}   | ${"unknown column 1"}   | ${"unknown column 2"}
-  ${1 - 1}     | ${2 + 2} | ${3333}
-  ${2 + 1 + 2} | ${1111}  | ${3}     | ${"unknown column xyz"}
+a|b|expected
+${11} | ${1}|${222}|${"unknown column 1"}|${"unknown column 2"}
+${1 - 1}|${2 + 2}|${3333}
+${2 + 1 + 2}|${1111}|${3}          |${"unknown column xyz"}
 `;
 
 describe.only.each`
-               |          |
-  ${11}        | ${1}     | ${222}  | ${"unknown column 1"}   | ${"unknown column 2"}
-  ${1 - 1}     | ${2 + 2} | ${3333}
-  ${2 + 1 + 2} | ${1111}  | ${3}    | ${"unknown column xyz"}
+||
+${11} | ${1}|${222}|${"unknown column 1"}|${"unknown column 2"}
+${1 - 1}|${2 + 2}|${3333}
+${2 + 1 + 2}|${1111}|${3}          |${"unknown column xyz"}
 `;
 
-describe.each`
-  a    | b    | expected
-  ${1} | ${1} | ${2}
-  ${1} | ${2} | ${3}
-  ${2} | ${1} | ${3}
-`;
+describe.each`a    | b    | expected
+${1} | ${1} | ${2}
+${1} | ${2} | ${3}
+${2} | ${1} | ${3}`;
 
 // an example to demo multiline quasi
-describe.each`
-  a    | b    | expected
-  ${11111111111} | ${a()
+describe.each`a    | b    | expected
+${11111111111} | ${a()
   .b((x) => x)
   .c()
   .d()} | ${2}
-  ${1} | ${2} | ${3}
-  ${2} | ${1} | ${3}
-`;
+${1} | ${2} | ${3}
+${2} | ${1} | ${3}`;
 
-describe.each([1, 2, 3])("test", (a) => {
-  expect(a).toBe(a);
-});
+describe
+  .each([1, 2, 3])(
+    "test",
+    (a) => {
+      expect(a).toBe(a);
+    },
+  );
 
-test.only.each([
-  [1, 1, 2],
-  [1, 2, 3],
-  [2, 1, 3],
-])(".add(%i, %i)", (a, b, expected) => {
-  expect(a + b).toBe(expected);
-});
+test.only
+  .each([[1, 1, 2], [1, 2, 3], [2, 1, 3]])(
+    ".add(%i, %i)",
+    (a, b, expected) => {
+      expect(a + b).toBe(expected);
+    },
+  );
 
-test.each([
-  { a: "1", b: 1 },
-  { a: "2", b: 2 },
-  { a: "3", b: 3 },
-])("test", ({ a, b }) => {
-  expect(Number(a)).toBe(b);
-});
+test
+  .each([{ a: "1", b: 1 }, { a: "2", b: 2 }, { a: "3", b: 3 }])(
+    "test",
+    ({ a, b }) => {
+      expect(Number(a)).toBe(b);
+    },
+  );
```
# js/test-declarations/test_declarations.js
```diff
 // Shouldn't break
 
-it("does something really long and complicated so I have to write a very long name for the test", () => {
-  console.log("hello!");
-});
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  () => {
+    console.log("hello!");
+  },
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", function () {
-  console.log("hello!");
-});
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  function () {
+    console.log("hello!");
+  },
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", function (done) {
-  console.log("hello!");
-});
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  function (done) {
+    console.log("hello!");
+  },
+);
 
-it("does something really long and complicated so I have to write a very long name for the test", function myAssertions(done) {
-  console.log("hello!");
-});
+it(
+  "does something really long and complicated so I have to write a very long name for the test",
+  function myAssertions(done) {
+    console.log("hello!");
+  },
+);
 
-it(`does something really long and complicated so I have to write a very long name for the test`, function () {
-  console.log("hello!");
-});
+it(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  function () {
+    console.log("hello!");
+  },
+);
 
-it(`{foo + bar} does something really long and complicated so I have to write a very long name for the test`, function () {
-  console.log("hello!");
-});
+it(
+  `{foo + bar} does something really long and complicated so I have to write a very long name for the test`,
+  function () {
+    console.log("hello!");
+  },
+);
 
-it(`handles
+it(
+  `handles
   some
     newlines
-  does something really long and complicated so I have to write a very long name for the test`, () => {
-  console.log("hello!");
-});
-
-test("does something really long and complicated so I have to write a very long name for the test", (done) => {
-  console.log("hello!");
-});
+  does something really long and complicated so I have to write a very long name for the test`,
+  () => {
+    console.log("hello!");
+  },
+);
 
-test(`does something really long and complicated so I have to write a very long name for the test`, (done) => {
-  console.log("hello!");
-});
-
-describe("does something really long and complicated so I have to write a very long name for the describe block", () => {
-  it("an example test", (done) => {
+test(
+  "does something really long and complicated so I have to write a very long name for the test",
+  (done) => {
     console.log("hello!");
-  });
-});
+  },
+);
 
-describe(`does something really long and complicated so I have to write a very long name for the describe block`, () => {
-  it(`an example test`, (done) => {
+test(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  (done) => {
     console.log("hello!");
-  });
-});
+  },
+);
 
-xdescribe("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+describe(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {
+    it(
+      "an example test",
+      (done) => {
+        console.log("hello!");
+      },
+    );
+  },
+);
+
+describe(
+  `does something really long and complicated so I have to write a very long name for the describe block`,
+  () => {
+    it(
+      `an example test`,
+      (done) => {
+        console.log("hello!");
+      },
+    );
+  },
+);
+
+xdescribe(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-fdescribe("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+fdescribe(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-describe.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+describe.only(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-describe.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+describe.skip(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-fit("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+fit(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-xit("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+xit(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-it.only("does something really long and complicated so I have to write a very long name for the test", () => {
-  console.log("hello!");
-});
+it.only(
+  "does something really long and complicated so I have to write a very long name for the test",
+  () => {
+    console.log("hello!");
+  },
+);
 
-it.only(`does something really long and complicated so I have to write a very long name for the test`, () => {
-  console.log("hello!");
-});
+it.only(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {
+    console.log("hello!");
+  },
+);
 
-it.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+it.skip(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-test.only(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.only(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-test.skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+test.skip(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-ftest("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+ftest(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-xtest("does something really long and complicated so I have to write a very long name for the describe block", () => {});
+xtest(
+  "does something really long and complicated so I have to write a very long name for the describe block",
+  () => {},
+);
 
-skip(`does something really long and complicated so I have to write a very long name for the test`, () => {});
+skip(
+  `does something really long and complicated so I have to write a very long name for the test`,
+  () => {},
+);
 
-skip("does something really long and complicated so I have to write a very long name for the test", () => {});
+skip(
+  "does something really long and complicated so I have to write a very long name for the test",
+  () => {},
+);
 
 // Should break
 
 it.only(
   "does something really long and complicated so I have to write a very long name for the test",
   10,
   () => {
     console.log("hello!");
-  }
+  },
 );
 
 it.only.only(
   "does something really long and complicated so I have to write a very long name for the test",
   () => {
     console.log("hello!");
-  }
+  },
 );
 
 it.only.only(
   "does something really long and complicated so I have to write a very long name for the test",
   (a, b, c) => {
     console.log("hello!");
-  }
+  },
 );
 
 xskip(
   "does something really long and complicated so I have to write a very long name for the test",
-  () => {}
+  () => {},
 );
 
 // timeout
 
-it(`handles
+it(
+  `handles
   some
     newlines
-  does something really long and complicated so I have to write a very long name for the test`, () => {
-  console.log("hello!");
-}, 2500);
+  does something really long and complicated so I have to write a very long name for the test`,
+  () => {
+    console.log("hello!");
+  },
+  2500,
+);
 
-it("does something quick", () => {
-  console.log("hello!");
-}, 1000000000);
+it(
+  "does something quick",
+  () => {
+    console.log("hello!");
+  },
+  1000000000,
+);
 
-it("succeeds if the test finishes in time", () =>
-  new Promise((resolve) => setTimeout(resolve, 10)));
+it(
+  "succeeds if the test finishes in time",
+  () => new Promise((resolve) => setTimeout(resolve, 10)),
+);
 
 it(
   "succeeds if the test finishes in time",
   () => new Promise((resolve) => setTimeout(resolve, 10)),
-  250
+  250,
 );
```
# js/throw_expressions/throw_expression.js
```diff
-function save(filename = throw new TypeError("Argument required")) {}
+function save(filename = throw new TypeError("Argument required")
+)
+{
+}
 
 lint(ast, {
-  with: () => throw new Error("avoid using 'with' statements."),
+  with: () => throw new Error("avoid using 'with' statements.")
 });
 
 function getEncoder(encoding) {
-  const encoder =
-    encoding === "utf8"
-      ? new UTF8Encoder()
-      : encoding === "utf16le"
-      ? new UTF16Encoder(false)
-      : encoding === "utf16be"
-      ? new UTF16Encoder(true)
-      : throw new Error("Unsupported encoding");
+  const encoder = encoding === "utf8" ? new UTF8Encoder()
+                : encoding === "utf16le" ? new UTF16Encoder(false)
+                : encoding === "utf16be" ? new UTF16Encoder(true)
+                :
+  throw new Error("Unsupported encoding");
 }
 
 class Product {
   get id() {
     return this._id;
   }
   set id(value) {
-    this._id = value || throw new Error("Invalid value");
+    this._id = value ||
+    throw new Error("Invalid value");
   }
 }
```
# js/throw_statement/binaryish.js
```diff
 function f() {
   throw (
     property.isIdentifier() &&
-    FUNCTIONS[property.node.name] &&
-    (object.isIdentifier(JEST_GLOBAL) ||
-      (callee.isMemberExpression() && shouldHoistExpression(object))) &&
-    FUNCTIONS[property.node.name](expr.get("arguments"))
+      FUNCTIONS[property.node.name] &&
+      (
+        object.isIdentifier(JEST_GLOBAL) || (
+          callee.isMemberExpression() && shouldHoistExpression(object)
+        )
+      ) &&
+      FUNCTIONS[property.node.name](expr.get("arguments"))
   );
 
-  throw (
-    chalk.bold("No tests found related to files changed since last commit.\n") +
-    chalk.dim(
-      patternInfo.watch
-        ? "Press `a` to run all tests, or run Jest with `--watchAll`."
-        : "Run Jest without `-o` to run all tests."
-    )
+  throw chalk.bold(
+    "No tests found related to files changed since last commit.\n",
+  ) + chalk.dim(
+    patternInfo.watch ? "Press `a` to run all tests, or run Jest with `--watchAll`." : "Run Jest without `-o` to run all tests.",
   );
 
-  throw (
-    !filePath.includes(coverageDirectory) &&
-    !filePath.endsWith(`.${SNAPSHOT_EXTENSION}`)
+  throw !filePath.includes(coverageDirectory) && !filePath.endsWith(
+    `.${SNAPSHOT_EXTENSION}`,
   );
 }
```
# js/throw_statement/comment.js
```diff
 function x() {
-  throw (
-    func2
-      //comment
-      .bar()
-  );
+  throw func2
+  //comment
+  .bar();
 }
 
 function f() {
   throw (
     foo
-      // comment
-      .bar()
+    // comment
+    .bar()
   );
 }
 
 fn(function f() {
   throw (
     foo
-      // comment
-      .bar()
+    // comment
+    .bar()
   );
 });
```
# js/trailing-comma/dynamic-import.js
```diff
 import(
-  "myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename"
+  "myreallylongdynamicallyloadedmodulenamemyreallylongdynamicallyloadedmodulename",
 );
```
# js/trailing-comma/es5.js
```diff
 function send_single_email(
   app,
   email_id,
   email_address,
   subject,
   html,
-  reply_to
+  reply_to,
 ) {
   send_single_email_implementation(
     app,
     email_id,
     email_address,
     subject,
     html,
-    reply_to
+    reply_to,
   );
   return "nothing";
 }
```
# js/trailing-comma/function-calls.js
```diff
 const a = (param1, param2, param3) => {};
 
 a("value", "value2", "value3");
 
 a(
   "a-long-value",
   "a-really-really-long-value",
-  "a-really-really-really-long-value"
+  "a-really-really-really-long-value",
 );
 
 a(
   "value",
   "value2",
-  a("long-nested-value", "long-nested-value2", "long-nested-value3")
+  a("long-nested-value", "long-nested-value2", "long-nested-value3"),
 );
 
-a.b().c(
-  {
-    d,
-  },
-  () => {}
-);
+a.b().c({ d }, () => {});
```
# js/trailing-comma/jsx.js
```diff
-<div
-  onClick={() =>
-    doSomething({
-      foo: bar,
-    })
-  }
-/>;
+<div onClick={() => doSomething({ foo: bar })} />;
```
# js/trailing-comma/object.js
```diff
-const a = {
-  b: true,
-  c: {
-    c1: "hello",
-  },
-  d: false,
-};
+const a = { b: true, c: { c1: "hello" }, d: false };
 
 const aLong = {
   bHasALongName: "a-long-value",
-  cHasALongName: {
-    c1: "a-really-long-value",
-    c2: "a-really-really-long-value",
-  },
+  cHasALongName: { c1: "a-really-long-value", c2: "a-really-really-long-value" },
   dHasALongName: "a-long-value-too",
 };
 
 const bLong = {
   dHasALongName: "a-long-value-too",
   eHasABooleanExpression: a === a,
 };
```
# js/trailing-comma/trailing_whitespace.js
```diff
 let example = [
-  "FOO",
-  "BAR",
+  "FOO", "BAR",
   // Comment
 ];
 
 foo(
-  {}
+  {},
   // Comment
 );
-
-o = {
-  state,
-  // Comment
-};
 
-o = {
-  state,
+o =
+  {
+    state,
+    // Comment
+  };
 
-  // Comment
-};
+o =
+  {
+    state,
+    // Comment
+  };
 
 function supersupersupersuperLongF(
   supersupersupersuperLongA,
-  supersupersupersuperLongB
+  supersupersupersuperLongB,
   // Comment
 ) {
   a;
 }
 function supersupersupersuperLongF(
   supersupersupersuperLongA,
-  supersupersupersuperLongB
+  supersupersupersuperLongB,
   // Comment
 ) {
   a;
 }
 
-this.getAttribute(function (s) /*string*/ {
-  console.log();
-});
-this.getAttribute(function (s) /*string*/ {
-  console.log();
-});
+this.getAttribute(
+  function (s)
+  /*string*/ {
+    console.log();
+  },
+);
+this.getAttribute(
+  function (s) /*string*/ {
+    console.log();
+  },
+);
```
# js/trailing-whitespace/trailing.js
```diff
 export type Result<T, V> =
   | { kind: "not-test-editor1" }
   | { kind: "not-test-editor2" };
 
 // Note: there are trailing whitespace in this file
 `
    
    
-` +
-  `
+` + `
     
     
 `;
```
# js/try/catch.js
```diff
 try {
 } catch (
   // comment
   foo
 ) {}
 
 try {
 } catch (
   foo //comment
 ) {}
 
 try {
-} catch (/* comment */ foo) {}
+} catch (
+  /* comment */ foo
+) {}
 
 try {
-} catch (foo /* comment */) {}
+} catch (foo /* comment */ ) {}
 
 try {
 } catch (
   foo
   /* comment */
 ) {}
```
# js/try/try.js
```diff
-try {
-  /* missing comment */
+try
+/* missing comment */
+{
 } finally {
 }
```
# js/tuple/destructuring.js
```diff
-const [a, b] = #[1, 2];
+const [a, b] = #
+[1, 2];
 assert(a === 1);
 assert(b === 2);
 
-const [a, ...rest] = #[1, 2, 3];
+const [a, ...rest] = #
+[1, 2, 3];
 assert(a === 1);
 assert(Array.isArray(rest));
 assert(rest[0] === 2);
 assert(rest[1] === 3);
```
# js/tuple/syntax.js
```diff
-#[];
-#[1, 2];
-#[1, 2, #{ a: 3 }];
+#[]
+#[1, 2]
+#[1, 2, #
+{
+  a: 3;
+}
+]
```
# js/tuple/tuple-trailing-comma.js
```diff
-#[1];
+#[1,]
```
# js/tuple/tuple.js
```diff
-const tuple1 = #[1, 2, 3];
+const tuple1 = #
+[1, 2, 3];
 
 assert(tuple1[0] === 1);
 
 const tuple2 = tuple1.with(0, 2);
 assert(tuple1 !== tuple2);
 assert(tuple2 === #[2, 2, 3]);
 
-const tuple3 = #[1, ...tuple2];
+const tuple3 = #
+[1, ...tuple2];
 assert(tuple3 === #[1, 2, 2, 3]);
 
 const tuple4 = tuple3.pushed(4);
 assert(tuple4 === #[1, 2, 2, 3, 4]);
 
 assert(tuple4.first() === 1);
 const tuple5 = tuple4.popped();
 assert(tuple5 === #[2, 2, 3, 4]);
```
# js/unary-expression/comments.js
```diff
 !x;
-!(x /* foo */);
-!(/* foo */ x);
+!(x /* foo */ );
+!( /* foo */ x);
 !(
   /* foo */
   x
 );
 !(
   x
   /* foo */
 );
 !(
   x // foo
 );
 
 !(x + y);
-!((x + y) /* foo */);
-!(/* foo */ (x + y));
+!(x + y /* foo */ );
+!( /* foo */ x + y);
 !(
   /* foo */
-  (x + y)
+  x + y
 );
 !(
-  (x + y)
+  x + y
   /* foo */
 );
 !(
-  (x + y) // foo
+  x + y // foo
 );
 
 !(x || y);
-!(/* foo */ (x || y));
-!((x || y) /* foo */);
+!( /* foo */ x || y);
+!(x || y /* foo */ );
 !(
   /* foo */
-  (x || y)
+  x || y
 );
 !(
-  (x || y)
+  x || y
   /* foo */
 );
 !(
-  (x || y) // foo
+  x || y // foo
 );
 
 ![1, 2, 3];
-!([1, 2, 3] /* foo */);
-!(/* foo */ [1, 2, 3]);
+!([1, 2, 3] /* foo */ );
+!( /* foo */ [1, 2, 3]);
 !(
   /* foo */
   [1, 2, 3]
 );
 !(
   [1, 2, 3]
   /* foo */
 );
 !(
   [1, 2, 3] // foo
 );
 
 !{ a: 1, b: 2 };
-!({ a: 1, b: 2 } /* foo */);
-!(/* foo */ { a: 1, b: 2 });
+!({ a: 1, b: 2 } /* foo */ );
+!( /* foo */ { a: 1, b: 2 });
 !(
   /* foo */
   { a: 1, b: 2 }
 );
 !(
   { a: 1, b: 2 }
   /* foo */
 );
 !(
   { a: 1, b: 2 } // foo
 );
 
 !function () {
   return x;
 };
 !(
   function () {
     return x;
   } /* foo */
 );
 !(
   /* foo */ function () {
     return x;
   }
 );
 !(
   /* foo */
   function () {
     return x;
   }
 );
 !(
   function () {
     return x;
   }
   /* foo */
 );
 !(
   function () {
     return x;
   } // foo
 );
 
 !+3;
-!(+3 /* foo */);
-!(/* foo */ +3);
+!(+3 /* foo */ );
+!( /* foo */ +3);
 !(
   /* foo */
   +3
 );
 !(
   +3
   /* foo */
 );
 !(
   +3 // foo
 );
 
 !+(
   /* foo */
   3
 );
-!(/* foo */ +(3 /* foo */));
-!(+(3 /* foo */) /* foo */);
+!( /* foo */ +(3 /* foo */ ));
+!(+(3 /* foo */ ) /* foo */ );
 !(
   /* foo */
   +(
     /* foo */
     3
   )
 );
 !(
   +(
     3
     /* foo */
   )
   /* foo */
 );
 !(
-  +(3 /* foo */) // foo
+  +(3 /* foo */ ) // foo
 );
 
 !(x = y);
-!((x = y) /* foo */);
-!(/* foo */ (x = y));
+!(x = y /* foo */ );
+!( /* foo */ x = y);
 !(
   /* foo */
-  (x = y)
+  x = y
 );
 !(
-  (x = y)
+  x = y
   /* foo */
 );
 !(
-  (x = y) // foo
+  x =
+    y // foo
 );
 
 !x.y;
-!(x.y /* foo */);
-!(/* foo */ x.y);
+!(x.y /* foo */ );
+!( /* foo */ x.y);
 !(
   /* foo */
   x.y
 );
 !(
   x.y
   /* foo */
 );
 !(
   x.y // foo
 );
 
 !(x ? y : z);
-!((x ? y : z) /* foo */);
-!(/* foo */ (x ? y : z));
+!(x ? y : z /* foo */ );
+!( /* foo */ x ? y : z);
 !(
   /* foo */
-  (x ? y : z)
+  x ? y : z
 );
 !(
-  (x ? y : z)
+  x ? y : z
   /* foo */
 );
 !(
-  (x ? y : z) // foo
+  x ? y : z // foo
 );
 
 !x();
-!(x() /* foo */);
-!(/* foo */ x());
+!(x() /* foo */ );
+!( /* foo */ x());
 !(
   /* foo */
   x()
 );
 !(
   x()
   /* foo */
 );
 !(
   x() // foo
 );
 
 !new x();
-!(new x() /* foo */);
-!(/* foo */ new x());
+!(new x() /* foo */ );
+!( /* foo */ new x());
 !(
   /* foo */
   new x()
 );
 !(
   new x()
   /* foo */
 );
 !(
   new x() // foo
 );
 
 !(x, y);
-!((x, y) /* foo */);
-!(/* foo */ (x, y));
+!(x, y /* foo */ );
+!( /* foo */ x, y);
 !(
   /* foo */
-  (x, y)
+  x, y
 );
 !(
-  (x, y)
+  x, y
   /* foo */
 );
 !(
   x.y // foo
 );
 
 !(() => 3);
-!((() => 3) /* foo */);
-!(/* foo */ (() => 3));
+!(() => 3 /* foo */ );
+!( /* foo */ () => 3);
 !(
   /* foo */
-  (() => 3)
+  () => 3
 );
 !(
-  (() => 3)
+  () => 3
   /* foo */
 );
 !(
-  (() => 3) // foo
+  () =>
+    3 // foo
 );
 
 function* bar() {
   !(yield x);
-  !((yield x) /* foo */);
-  !(/* foo */ (yield x));
+  !(yield x /* foo */ );
+  !( /* foo */ yield x);
   !(
     /* foo */
-    (yield x)
+    yield x
   );
   !(
-    (yield x)
+    yield x
     /* foo */
   );
   !(
-    (yield x) // foo
+    yield x // foo
   );
 }
 
 async function bar2() {
   !(await x);
-  !((await x) /* foo */);
-  !(/* foo */ (await x));
+  !(await x /* foo */ );
+  !( /* foo */ await x);
   !(
     /* foo */
-    (await x)
+    await x
   );
   !(
-    (await x)
+    await x
     /* foo */
   );
   !(
-    (await x) // foo
+    await x // foo
   );
 }
```
# js/unary-expression/urnary_expression.js
```diff
 !!x;
 x++;
 x--;
 -+1;
 x + +(+(+1));
-x + +(+(+1));
+x + (+(+(+1)));
 x * +y;
 +x * y;
```
# js/unary/object.js
```diff
-state = {
-  // students
-  hoverColumn: -1,
-};
+state =
+  {
+    // students
+    hoverColumn: -1,
+  };
```
# js/unicode/combining-characters.js
```diff
-const x = ["ÁÀĀÉÈĒẸE̩Ẹ́É̩Ẹ̀È̩Ẹ̄Ē̩ÍÌĪÓÒŌỌO̩Ọ́Ó̩Ọ̀Ò̩Ọ̄Ō̩ÚÙŪṢS̩áàāéèēẹe̩ẹ́é̩ẹ̀è̩ẹ̄ē̩íìīóòōọo̩ọ́ó̩ọ̀ò̩ọ̄ō̩úùū"];
+const x = [
+  "ÁÀĀÉÈĒẸE̩Ẹ́É̩Ẹ̀È̩Ẹ̄Ē̩ÍÌĪÓÒŌỌO̩Ọ́Ó̩Ọ̀Ò̩Ọ̄Ō̩ÚÙŪṢS̩áàāéèēẹe̩ẹ́é̩ẹ̀è̩ẹ̄ē̩íìīóòōọo̩ọ́ó̩ọ̀ò̩ọ̄ō̩úùū",
+];
 //345678901234567890123456789012345678901234567890123456789012345678901234567890
 //       1         2         3         4         5         6         7         8
```
# js/unicode/keys.js
```diff
-({ この事はつもり素晴らしいことさ: "35jL9V" });
+({ "この事はつもり素晴らしいことさ": "35jL9V" });
```
# js/unicode/nbsp-jsx.js
```diff
 // Note: there are non breaking spaces in the JSX text
-x = (
-  <p>
-     aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa 
-  </p>
-);
+x =
+  <p> aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa </p>;
```
# js/v8_intrinsic/intrinsic_call.js
```diff
 function doSmth() {
-  %DebugPrint(foo);
+  %DebugPrint
+        (
+                foo )
 }
 
 function printFunc(f) {
-  if (%IsAsmWasmCode(f)) console.log("asm.js");
-  if (%IsWasmCode(f)) console.log("wasm");
+  if(%
+    IsAsmWasmCode(f))              console.log("asm.js");
+  if(
+
+        % IsWasmCode(
+        f))
+            console.log (
+                "wasm"
+            );
 
-  console.log(%GetFunctioName(f));
+  console.log
+    (%
+        GetFunctioName(f)
+        );
 }
```
# js/variable_declarator/multiple.js
```diff
-var assert = require("assert"),
-  lookup = require("../lookup");
+var assert = require("assert"), lookup = require("../lookup");
 
 const eloBar = require("elo-bar"),
   foo = require("foo"),
   otherThing = require("other-thing");
 
 var a, b, c;
 
 let superSuperSuperLong1,
   superSuperSuperLong2,
   superSuperSuperLong3,
   superSuperSuperLong4;
 
 for (var i = 0, len = arr.length; i < len; i++) {}
 
 var templateTagsMapping = {
-    "%{itemIndex}": "index",
-    "%{itemContentMetaTextViews}": "views",
-  },
+  "%{itemIndex}": "index",
+  "%{itemContentMetaTextViews}": "views",
+},
   separator = '<span class="item__content__meta__separator">•</span>',
-  templateTagsList = $.map(templateTagsMapping, function (value, key) {
-    return key;
-  }),
+  templateTagsList = $.map(
+    templateTagsMapping,
+    function (value, key) {
+      return key;
+    },
+  ),
   data;
```
# js/while/indent.js
```diff
 if (
   someVeryLongStringA &&
   someVeryLongStringB &&
   someVeryLongStringC &&
   someVeryLongStringD
 ) {
 }
 while (
   someVeryLongStringA &&
   someVeryLongStringB &&
   someVeryLongStringC &&
   someVeryLongStringD
 ) {}
 do {} while (
   someVeryLongStringA &&
   someVeryLongStringB &&
   someVeryLongStringC &&
   someVeryLongStringD
 );
 
 if (
   someVeryLongFunc(
     someVeryLongArgA,
     someVeryLongArgB,
     someVeryLongArgC,
-    someVeryLongArgD
+    someVeryLongArgD,
   )
 ) {
 }
 while (
   someVeryLongFunc(
     someVeryLongArgA,
     someVeryLongArgB,
     someVeryLongArgC,
-    someVeryLongArgD
+    someVeryLongArgD,
   )
 ) {}
 do {} while (
   someVeryLongFunc(
     someVeryLongArgA,
     someVeryLongArgB,
     someVeryLongArgC,
-    someVeryLongArgD
+    someVeryLongArgD,
   )
 );
 
 while (0) 1;
 
-do 1;
-while (0);
+do 1; while (0);
```
# js/with/indent.js
```diff
-with (0) {
-}
+with (0) {}
 
 with (0) 1;
```
# js/yield/arrow.js
```diff
 function* f() {
-  yield (a) => a;
-  yield async (a) => a;
-  yield async (a) => a;
+  (yield (a) => a);
+  (yield async (a) => a);
+  (yield async (a) => a);
 }
```
# js/yield/conditional.js
```diff
 function* f1() {
   a = (yield) ? 1 : 1;
   a = yield 1 ? 1 : 1;
   a = (yield 1) ? 1 : 1;
   a = 1 ? yield : yield;
   a = 1 ? yield 1 : yield 1;
 }
 
 function* f2() {
   a = yield* 1 ? 1 : 1;
   a = (yield* 1) ? 1 : 1;
   a = 1 ? yield* 1 : yield* 1;
 }
 
 async function f3() {
-  a = (await 1) ? 1 : 1;
+  a = await 1 ? 1 : 1;
   a = (await 1) ? 1 : 1;
   a = 1 ? await 1 : await 1;
 }
```
# js/yield/jsx-without-parenthesis.js
```diff
 function* f() {
   yield <div>generator</div>;
-  yield (
-    <div>
-      <p>generator</p>
-    </div>
-  );
+  yield <div><p>generator</p></div>;
 }
```
# js/yield/jsx.js
```diff
 function* f() {
-  yield <div>generator</div>;
-  yield (
-    <div>
-      <p>generator</p>
-    </div>
-  );
+  yield (<div>generator</div>);
+  yield (<div><p>generator</p></div>);
 }
```
# typescript/abstract-construct-types/abstract-construct-types.ts
```diff
-type T = abstract new () => void;
-type T = abstract new () => void;
-type T = abstract new () => void;
+type T = abstract new() => void;
+type T = abstract new() => void;
+type T = abstract new() => void;
```
# typescript/angular-component-examples/test.component.ts
```diff
 @Component({
-  selector: "app-test",
-  template: `<ul>
-    <li>test</li>
-  </ul> `,
-  styles: [
-    `
-      :host {
-        color: red;
-      }
-      div {
-        background: blue;
-      }
-    `,
-  ],
+       selector: 'app-test',
+  template: `<ul>   <li>test</li>
+  </ul>
+  `,
+  styles: [   `
+  
+ :host {
+   color: red;
+ } 
+ div { background: blue
+ }
+`
+
+]
 })
 class TestComponent {}
```
# typescript/argument-expansion/argument_expansion.ts
```diff
-const bar1 = [1, 2, 3].reduce((carry, value) => {
-  return [...carry, value];
-}, [] as unknown as number[]);
+const bar1 = [1, 2, 3].reduce(
+  (carry, value) => {
+    return [...carry, value];
+  },
+  ([] as unknown) as number[],
+);
 
-const bar2 = [1, 2, 3].reduce((carry, value) => {
-  return [...carry, value];
-}, <Array<number>>[]);
+const bar2 = [1, 2, 3].reduce(
+  (carry, value) => {
+    return [...carry, value];
+  },
+  <Array<number>>[],
+);
 
 const bar3 = [1, 2, 3].reduce(
   (carry, value) => {
     return [...carry, value];
   },
-  [1, 2, 3] as unknown as number[]
+  ([1, 2, 3] as unknown) as number[],
 );
 
 const bar4 = [1, 2, 3].reduce(
   (carry, value) => {
     return [...carry, value];
   },
-  <Array<number>>[1, 2, 3]
+  <Array<number>>[1, 2, 3],
 );
 
-const bar5 = [1, 2, 3].reduce((carry, value) => {
-  return { ...carry, [value]: true };
-}, {} as unknown as { [key: number]: boolean });
+const bar5 = [1, 2, 3].reduce(
+  (carry, value) => {
+    return { ...carry, [value]: true };
+  },
+  ({} as unknown) as { [key: number]: boolean },
+);
 
-const bar6 = [1, 2, 3].reduce((carry, value) => {
-  return { ...carry, [value]: true };
-}, <{ [key: number]: boolean }>{});
+const bar6 = [1, 2, 3].reduce(
+  (carry, value) => {
+    return { ...carry, [value]: true };
+  },
+  <{ [key: number]: boolean }>{},
+);
 
 const bar7 = [1, 2, 3].reduce(
   (carry, value) => {
     return { ...carry, [value]: true };
   },
-  { 1: true } as unknown as { [key: number]: boolean }
+  ({ 1: true } as unknown) as { [key: number]: boolean },
 );
 
 const bar8 = [1, 2, 3].reduce(
   (carry, value) => {
     return { ...carry, [value]: true };
   },
-  <{ [key: number]: boolean }>{ 1: true }
+  <{ [key: number]: boolean }>{ 1: true },
 );
```
# typescript/argument-expansion/arrow-with-return-type.ts
```diff
-longfunctionWithCall1("bla", foo, (thing: string): complex<type<something>> => {
-  code();
-});
+longfunctionWithCall1(
+  "bla",
+  foo,
+  (thing: string): complex<type<something>> => {
+    code();
+  },
+);
 
 longfunctionWithCall12(
   "bla",
   foo,
   (thing: string): complex<type<something>> => {
     code();
-  }
+  },
 );
 
 longfunctionWithCallBack(
   "blabla",
   foobarbazblablablablabla,
   (thing: string): complex<type<something>> => {
     code();
-  }
+  },
 );
 
 longfunctionWithCallBack(
   "blabla",
   foobarbazblablabla,
   (thing: string): complex<type<something>> => {
     code();
-  }
+  },
 );
 
 longfunctionWithCall1(
   "bla",
   foo,
-  (
-    thing: string
-  ): complex<
-    type<`
-`>
+  (thing: string): complex<
+    type<
+      `
+`
+    >
   > => {
     code();
-  }
+  },
 );
```
# typescript/array/key.ts
```diff
 const subtractDuration = moment.duration(
   subtractMap[interval][0],
-  subtractMap[interval][1] as unitOfTime.DurationConstructor
+  subtractMap[interval][1] as unitOfTime.DurationConstructor,
 );
```
# typescript/arrow/arrow_regression.ts
```diff
 const bar = (...varargs: any[]) => {
   console.log(varargs);
 };
 
-const foo = (x: string): void =>
-  bar(
-    x,
-    () => {},
-    () => {}
-  );
+const foo = (x: string): void => (bar(x, () => {}, () => {}));
 
-app.get("/", (req, res): void => {
-  res.send("Hello world");
-});
+app.get(
+  "/",
+  (req, res): void => {
+    res.send("Hello world");
+  },
+);
```
# typescript/arrow/comments.ts
```diff
 const fn1 = () => {
   return;
-}; /* foo */
+} /* foo */ ;
 
 const fn2 = () => {
   return;
-};
-
+}
 // foo
+;
```
# typescript/arrow/issue-6107-curry.ts
```diff
-const getIconEngagementTypeFrom =
-  (engagementTypes: Array<EngagementType>) => (iconEngagementType) =>
-    engagementTypes.includes(iconEngagementType);
+const getIconEngagementTypeFrom = (engagementTypes: Array<EngagementType>) => (
+  iconEngagementType,
+) => engagementTypes.includes(iconEngagementType);
 
-const getIconEngagementTypeFrom2 =
-  (engagementTypes: Array<EngagementType>, secondArg: Something) =>
-  (iconEngagementType) =>
-    engagementTypes.includes(iconEngagementType);
+const getIconEngagementTypeFrom2 = (
+  engagementTypes: Array<EngagementType>,
+  secondArg: Something,
+) => (iconEngagementType) => engagementTypes.includes(iconEngagementType);
 
-const getIconEngagementTypeFrom2 =
-  (
-    engagementTypes: Array<EngagementType>,
-    secondArg: Something,
-    thirArg: SomethingElse
-  ) =>
-  (iconEngagementType) =>
-    engagementTypes.includes(iconEngagementType);
+const getIconEngagementTypeFrom2 = (
+  engagementTypes: Array<EngagementType>,
+  secondArg: Something,
+  thirArg: SomethingElse,
+) => (iconEngagementType) => engagementTypes.includes(iconEngagementType);
```
# typescript/as/as.ts
```diff
 const name = (description as DescriptionObject).name || (description as string);
 this.isTabActionBar((e.target || e.srcElement) as HTMLElement);
-(originalError
-  ? wrappedError(errMsg, originalError)
-  : Error(errMsg)) as InjectionError;
+(originalError ? wrappedError(errMsg, originalError) : Error(errMsg)) as InjectionError;
 "current" in (props.pagination as Object);
 ("current" in props.pagination) as Object;
 start + (yearSelectTotal as number);
 (start + yearSelectTotal) as number;
 scrollTop > (visibilityHeight as number);
 (scrollTop > visibilityHeight) as number;
-export default class Column<T> extends (RcTable.Column as React.ComponentClass<
-  ColumnProps<T>,
-  ColumnProps<T>,
-  ColumnProps<T>,
-  ColumnProps<T>
->) {}
+export default class Column<T> extends (
+  RcTable.Column as React.ComponentClass<
+    ColumnProps<T>,
+    ColumnProps<T>,
+    ColumnProps<T>,
+    ColumnProps<T>
+  >
+) {}
 export const MobxTypedForm = class extends (Form as { new (): any }) {};
 export abstract class MobxTypedForm1 extends (Form as { new (): any }) {}
-({} as {});
+({}) as {};
 function* g() {
   const test = (yield "foo") as number;
 }
 async function g1() {
   const test = (await "foo") as number;
 }
-({} as X);
-() => ({} as X);
-const state = JSON.stringify({
-  next: window.location.href,
-  nonce,
-} as State);
+({}) as X;
+() => ({}) as X;
+const state = JSON.stringify({ next: window.location.href, nonce } as State);
 
 (foo.bar as Baz) = [bar];
 (foo.bar as any)++;
 
 (bValue as boolean) ? 0 : -1;
 <boolean>bValue ? 0 : -1;
 
-const value1 =
-  thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;
-const value2 =
-  thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;
-const value3 = thisIsAReallyLongIdentifier as
-  | SomeInterface
-  | SomeOtherInterface;
+const value1 = thisIsAReallyReallyReallyReallyReallyLongIdentifier as SomeInterface;
+const value2 = thisIsAnIdentifier as thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyReallyLongInterface;
+const value3 = thisIsAReallyLongIdentifier as (
+  SomeInterface | SomeOtherInterface
+);
 const value4 = thisIsAReallyLongIdentifier as {
   prop1: string;
   prop2: number;
   prop3: number;
 }[];
-const value5 =
-  thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [
-    string,
-    number
-  ];
+const value5 = thisIsAReallyReallyReallyReallyReallyReallyReallyReallyReallyLongIdentifier as [
+  string,
+  number,
+];
 
 const iter1 = createIterator(
   this.controller,
   child,
-  this.tag as SyncFunctionComponent
+  this.tag as SyncFunctionComponent,
 );
 const iter2 = createIterator(
   self.controller,
   child,
-  self.tag as SyncFunctionComponent
+  self.tag as SyncFunctionComponent,
 );
```
# typescript/as/assignment.ts
```diff
 export const LOG_LEVEL = {
   EMERGENCY: 0,
   ALERT: 1,
   CRITICAL: 2,
   ERROR: 3,
   WARNING: 4,
   NOTICE: 5,
   INFO: 6,
   DEBUG: 7,
 } as const;
 
 const TYPE_MAP = {
   "character device": "special",
   "character special file": "special",
   directory: "directory",
   "regular file": "file",
   socket: "socket",
   "symbolic link": "link",
 } as Foo;
 
-this.previewPlayerHandle = setInterval(async () => {
-  if (this.previewIsPlaying) {
-    await this.fetchNextPreviews();
-    this.currentPreviewIndex++;
-  }
-}, this.refreshDelay) as unknown as number;
+this.previewPlayerHandle =
+  (
+    setInterval(
+      async () => {
+        if (this.previewIsPlaying) {
+          await this.fetchNextPreviews();
+          this.currentPreviewIndex++;
+        }
+      },
+      this.refreshDelay,
+    ) as unknown
+  ) as number;
 
-this.intervalID = setInterval(() => {
-  self.step();
-}, 30) as unknown as number;
+this.intervalID =
+  (
+    setInterval(
+      () => {
+        self.step();
+      },
+      30,
+    ) as unknown
+  ) as number;
```
# typescript/as/assignment2.ts
```diff
-const defaultMaskGetter = $parse(attrs[directiveName]) as (
-  scope: ng.IScope
-) => Mask;
+const defaultMaskGetter = $parse(attrs[directiveName]) as (scope: ng.IScope) => Mask;
 
 (this.configuration as any) =
-  (this.editor as any) =
-  (this.editorBody as any) =
-    undefined;
+  (this.editor as any) = (this.editorBody as any) = undefined;
 
-angular.module("foo").directive("formIsolator", () => {
-  return {
-    name: "form",
-    controller: class FormIsolatorController {
-      $addControl = angular.noop;
-    } as ng.IControllerConstructor,
-  };
-});
+angular
+  .module("foo")
+  .directive(
+    "formIsolator",
+    () => {
+      return {
+        name: "form",
+        controller: class FormIsolatorController {
+          $addControl = angular.noop;
+        } as ng.IControllerConstructor,
+      };
+    },
+  );
 
 (this.selectorElem as any) =
-  this.multiselectWidget =
-  this.initialValues =
-    undefined;
+  this.multiselectWidget = this.initialValues = undefined;
 
-const extraRendererAttrs = ((attrs.rendererAttrs &&
-  this.utils.safeParseJsonString(attrs.rendererAttrs)) ||
-  Object.create(null)) as FieldService.RendererAttributes;
+const extraRendererAttrs = (
+  (attrs.rendererAttrs && this.utils.safeParseJsonString(attrs.rendererAttrs)) || Object.create(
+    null,
+  )
+) as FieldService.RendererAttributes;
 
-const annotate = (angular.injector as any).$$annotate as (
-  fn: Function
-) => string[];
+const annotate = (angular.injector as any).$$annotate as (fn: Function) => string[];
 
-const originalPrototype = originalConstructor.prototype as TComponent &
-    InjectionTarget,
+const originalPrototype = originalConstructor.prototype as
+  & TComponent
+  & InjectionTarget,
   propertyToServiceName = originalPrototype._inject;
```
# typescript/as/long-identifiers.ts
```diff
-const bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;
+const bifornCringerMoshedPerplexSawder = askTrovenaBeenaDependsRowans as glimseGlyphsHazardNoopsTieTie;
 
 averredBathersBoxroomBuggyNurl.anodyneCondosMalateOverateRetinol =
   annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave;
 
-averredBathersBoxroomBuggyNurl = {
-  anodyneCondosMalateOverateRetinol:
-    annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,
-};
+averredBathersBoxroomBuggyNurl =
+  {
+    anodyneCondosMalateOverateRetinol: annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,
+  };
 
 averredBathersBoxroomBuggyNurl(
-  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave
+  anodyneCondosMalateOverateRetinol.annularCooeedSplicesWalksWayWay as kochabCooieGameOnOboleUnweave,
 );
```
# typescript/as/nested-await-and-as.ts
```diff
 const getAccountCount = async () =>
   (
     await (
-      (await (
-        await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)
-      ).findItem("My bookmarks")) as TreeItem
+      (
+        await (await focusOnSection(BOOKMARKED_PROJECTS_SECTION_NAME)).findItem(
+          "My bookmarks",
+        )
+      ) as TreeItem
     ).getChildren()
   ).length;
```
# typescript/as/return.ts
```diff
 function foo() {
-  return {
-    foo: 1,
-    bar: 2,
-  } as Foo;
+  return { foo: 1, bar: 2 } as Foo;
 }
```
# typescript/as/ternary.ts
```diff
-foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-) as Fooooooooooo;
+foo =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  ) as Fooooooooooo;
 
 foo = (condition ? firstValue : secondValue) as SomeType;
 
 const foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
+  coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
 ) as Fooooooooooo;
 
 function foo() {
   return (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ) as Fooooooooooo;
 }
 
 function foo() {
   throw (
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
   ) as Fooooooooooo;
 }
 
 function foo() {
-  void ((
-    coooooooooooooooooooooooooooooooooooooooooooooooooooond
-      ? baaaaaaaaaaaaaaaaaaaaar
-      : baaaaaaaaaaaaaaaaaaaaaz
-  ) as Fooooooooooo);
+  void (
+    (
+      coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+    ) as Fooooooooooo
+  );
 }
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  ((glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ) as AnnularCooeedSplicesWalksWayWay
+  );
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  ((glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol) as AnnularCooeedSplicesWalksWayWay);
+  askTrovenaBeenaDependsRowans + (
+    (
+      glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+    ) as AnnularCooeedSplicesWalksWayWay
+  );
```
# typescript/assert/index.ts
```diff
 const assertString = (x: any): asserts x => {
   console.assert(typeof x === "string");
 };
 
 function assertsString(x: any): asserts x {
   console.assert(typeof x === "string");
 }
 
 const assertStringWithGuard = (x: any): asserts x is string => {
   console.assert(typeof x === "string");
 };
 
 function assertsStringWithGuard(x: any): asserts x is string {
   console.assert(typeof x === "string");
 }
 
 interface AssertFoo {
-  isString(node: any): asserts node;
+  isString(node: any): asserts node ;
 }
 
 class AssertsFoo {
   isBar(): asserts this {
     return;
   }
   isBaz = (): asserts this => {
     return;
   };
 }
```
# typescript/assignment/issue-10846.ts
```diff
-const foo = call<{
-  prop1: string;
-  prop2: string;
-  prop3: string;
-}>();
+const foo = call<{ prop1: string; prop2: string; prop3: string }>();
 
-export const CallRecorderContext = createContext<{
-  deleteRecording: (id: string) => void;
-  deleteAll: () => void;
-} | null>(null);
+export const CallRecorderContext = createContext<
+  { deleteRecording: (id: string) => void; deleteAll: () => void } | null
+>(null);
 
-export const CallRecorderContext = createContext<{
-  deleteRecording: (id: string) => void;
-  deleteAll: () => void;
-} | null>(null, "useless");
+export const CallRecorderContext = createContext<
+  { deleteRecording: (id: string) => void; deleteAll: () => void } | null
+>(null, "useless");
 
-const foo = call<
-  Foooooo,
-  Foooooo,
-  Foooooo,
-  Foooooo,
-  Foooooo,
-  Foooooo,
-  Foooooo
->();
+const foo = call<Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo, Foooooo>();
 
 const foo = call<
   Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo | Foooooooooooo
 >();
 
 const foo = call<
   Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo & Foooooooooooo
 >();
```
# typescript/assignment/issue-10848.tsx
```diff
 const MyComponent: React.VoidFunctionComponent<MyComponentProps> = ({ x }) => {
   const a = useA();
-  return (
-    <div>
-      x = {x}; a = {a}
-    </div>
-  );
+  return <div>x = {x}; a = {a}</div>;
 };
 
-const MyComponent2: React.VoidFunctionComponent<MyComponent2Props> = ({
-  x,
-  y,
-}) => {
+const MyComponent2: React.VoidFunctionComponent<MyComponent2Props> = ({ x, y }) => {
   const a = useA();
-  return (
-    <div>
-      x = {x}; y = {y}; a = {a}
-    </div>
-  );
+  return <div>x = {x}; y = {y}; a = {a}</div>;
 };
 
 const MyComponentWithLongName1: React.VoidFunctionComponent<
   MyComponentWithLongNameProps
 > = ({ x, y }) => {
   const a = useA();
-  return (
-    <div>
-      x = {x}; y = {y}; a = {a}
-    </div>
-  );
+  return <div>x = {x}; y = {y}; a = {a}</div>;
 };
 
 const MyComponentWithLongName2: React.VoidFunctionComponent<
   MyComponentWithLongNameProps
-> = ({
-  x,
-  y,
-  anotherPropWithLongName1,
-  anotherPropWithLongName2,
-  anotherPropWithLongName3,
-  anotherPropWithLongName4,
-}) => {
+> = (
+  {
+    x,
+    y,
+    anotherPropWithLongName1,
+    anotherPropWithLongName2,
+    anotherPropWithLongName3,
+    anotherPropWithLongName4,
+  },
+) => {
   const a = useA();
-  return (
-    <div>
-      x = {x}; y = {y}; a = {a}
-    </div>
-  );
+  return <div>x = {x}; y = {y}; a = {a}</div>;
 };
 
 const MyGenericComponent: React.VoidFunctionComponent<
   MyGenericComponentProps<number>
 > = ({ x, y }) => {
   const a = useA();
-  return (
-    <div>
-      x = {x}; y = {y}; a = {a}
-    </div>
-  );
+  return <div>x = {x}; y = {y}; a = {a}</div>;
 };
 
-export const ExportToExcalidrawPlus: React.FC<{
-  elements: readonly NonDeletedExcalidrawElement[];
-  appState: AppState;
-  onError: (error: Error) => void;
-}> = ({ elements, appState, onError }) => {
+export const ExportToExcalidrawPlus: React.FC<
+  {
+    elements: readonly NonDeletedExcalidrawElement[];
+    appState: AppState;
+    onError: (error: Error) => void;
+  }
+> = ({ elements, appState, onError }) => {
   return null;
 };
 
-const Query: FunctionComponent<QueryProps> = ({
-  children,
-  type,
-  resource,
-  payload,
-  // Provides an undefined onSuccess just so the key `onSuccess` is defined
-  // This is used to detect options in useDataProvider
-  options = { onSuccess: undefined },
-}) =>
+const Query: FunctionComponent<QueryProps> = (
+  {
+    children,
+    type,
+    resource,
+    payload,
+    // Provides an undefined onSuccess just so the key `onSuccess` is defined
+    // This is used to detect options in useDataProvider
+    options = { onSuccess: undefined },
+  },
+) =>
   children(
     useQuery(
       { type, resource, payload },
-      { ...options, withDeclarativeSideEffectsSupport: true }
-    )
+      { ...options, withDeclarativeSideEffectsSupport: true },
+    ),
   );
```
# typescript/assignment/issue-10850.ts
```diff
-const map: Map<
-  Function,
-  Map<string | void, { value: UnloadedDescriptor }>
-> = new Map();
+const map: Map<Function, Map<string | void, { value: UnloadedDescriptor }>> = new Map();
 
-const map: Map<
-  Function,
-  Condition extends Foo ? FooFooFoo : BarBarBar
-> = new Map();
+const map: Map<Function, Condition extends Foo ? FooFooFoo : BarBarBar> = new Map();
 
-const map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =
-  new Map();
+const map: Map<Function, FunctionFunctionFunctionFunctionffFunction> = new Map();
 
 const map: Map<Function, Foo<S>> = new Map();
```
# typescript/assignment/issue-2322.ts
```diff
-export const listAuthorizedSitesForDefaultHandler: ListAuthorizedSitesForHandler =
-  aListAuthorizedSitesForResponse;
+export const listAuthorizedSitesForDefaultHandler: ListAuthorizedSitesForHandler = aListAuthorizedSitesForResponse;
```
# typescript/assignment/issue-2482.ts
```diff
-export function countriesReceived(
-  countries: Array<Country>
-): CountryActionType {
-  return {
-    type: ActionTypes.COUNTRIES_RECEIVED,
-    countries: countries,
-  };
+export function countriesReceived(countries: Array<Country>): CountryActionType {
+  return { type: ActionTypes.COUNTRIES_RECEIVED, countries: countries };
 }
```
# typescript/assignment/issue-2485.ts
```diff
 class x {
-  private readonly rawConfigFromFile$: BehaviorSubject<any> =
-    new BehaviorSubject(notRead);
+  private readonly rawConfigFromFile$: BehaviorSubject<any> = new BehaviorSubject(
+    notRead,
+  );
 }
```
# typescript/assignment/issue-3122.ts
```diff
 export const findByDate: Resolver<void, Recipe[], { date: Date }> = (
   _,
   { date },
-  { req }
+  { req },
 ) => {
   const repo = req.getRepository(Recipe);
   return repo.find({ createDate: date });
 };
 
 export const findByDate: Resolver<void, Recipe[], { date: Date }> = (
   _,
   { date },
-  { req }
+  { req },
 ) => Recipe.find({ createDate: date });
```
# typescript/assignment/issue-5370.ts
```diff
-const durabilityMetricsSelectable: Immutable.OrderedSet<SomeReportingMetric> =
-  myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
+const durabilityMetricsSelectable: Immutable.OrderedSet<
+  SomeReportingMetric,
+> = myExperienceSelectable.concat(otherDurabilityMetricsSelectable);
```
# typescript/assignment/issue-6783.ts
```diff
-export const enviromentProdValues: EnvironmentValues =
-  assign<EnvironmentValues>(
-    {
-      apiURL: "/api",
-    },
-    enviromentBaseValues
-  );
+export const enviromentProdValues: EnvironmentValues = assign<EnvironmentValues>(
+  { apiURL: "/api" },
+  enviromentBaseValues,
+);
```
# typescript/assignment/issue-8619.ts
```diff
 {
   {
     {
-      const myLongVariableName: MyLongTypeName | null =
-        myLongFunctionCallHere();
+      const myLongVariableName: MyLongTypeName | null = myLongFunctionCallHere();
     }
   }
 }
```
# typescript/assignment/issue-9172.ts
```diff
-const firestorePersonallyIdentifiablePaths: Array<Collections.Users.Entity> =
-  somefunc();
+const firestorePersonallyIdentifiablePaths: Array<Collections.Users.Entity> = somefunc();
```
# typescript/assignment/lone-arg.ts
```diff
 if (true) {
   if (condition) {
-    const secondType = sourceCode.getNodeByRangeIndex1234(
-      second.range[0]
-    )!.type;
+    const secondType = sourceCode.getNodeByRangeIndex1234(second.range[0])!.type;
   }
 }
```
# typescript/assignment/parenthesized.ts
```diff
 // https://github.com/babel/babel/pull/12933/files
 (<number>x) = null;
-x! = null;
+(x!) = null;
 (a as any) = null;
 (a as number) = 42;
-(a as any as string) = null;
+((a as any) as string) = null;
```
# typescript/break-calls/type_args.ts
```diff
 const response = something.$http.get<ThingamabobService.DetailsData>(
   `api/foo.ashx/foo-details/${myId}`,
-  { cache: quux.httpCache, timeout }
+  { cache: quux.httpCache, timeout },
 );
```
# typescript/cast/as-const.ts
```diff
 let x = "123" as const;
 
 // https://github.com/babel/babel/pull/11912
-(x as boolean) <= y; // (x as boolean) <= y;
-(x as boolean) ?? y; // (x as boolean) ?? y;
+x as boolean <= y; // (x as boolean) <= y;
+x as boolean ?? y; // (x as boolean) ?? y;
```
# typescript/cast/generic-cast.ts
```diff
 // https://github.com/prettier/prettier/issues/4171
 function y() {
   const fits = <Immutable.Map<string, any>>fits();
   const fitsObjLiteral = <Immutable.Map<string, any>>{ a: "test" };
   const fitsArrayLiteral = <Immutable.Map<string, any>>["test", "test2"];
 
-  const breakAfterCast = <Immutable.Map<string, any>>(
-    someExistingConfigMap.mergeDeep(fallbackOpts)
+  const breakAfterCast = <Immutable.Map<string, any>>someExistingConfigMap.mergeDeep(
+    fallbackOpts,
   );
 
   const stillTooLong = <
     Immutable.Map<
       string,
       boolean,
       number,
       object,
       null,
       undefined,
       any,
       void,
       never
     >
   >someExistingConfigMap.mergeDeep(fallbackOptions);
 
   const stillTooLong2 = <
-    | Immutable.Map<
+      | Immutable.Map<
         string,
         boolean,
         number,
         object,
         null,
         undefined,
         any,
         void,
         never
       >
-    | undefined
+      | undefined
   >someExistingConfigMap.mergeDeep(fallbackOptions);
 
-  const stillTooLong3 = <Immutable.Map<string>>(
-    someExistingConfigMap.mergeDeep(
-      fallbackOptions.someMethodWithLongName(param1, param2)
-    )
+  const stillTooLong3 = <Immutable.Map<string>>someExistingConfigMap.mergeDeep(
+    fallbackOptions.someMethodWithLongName(param1, param2),
   );
 
   const stillTooLong4 = <
-    | Immutable.Map<
+      | Immutable.Map<
         string,
         boolean,
         number,
         object,
         null,
         undefined,
         any,
         void,
         never
       >
-    | undefined
+      | undefined
   >someExistingConfigMap.mergeDeep(
-    fallbackOptions.someMethodWithLongName(param1, param2)
+    fallbackOptions.someMethodWithLongName(param1, param2),
   );
 
   const testObjLiteral = <Immutable.Map<string, any>>{
     property1: "myPropertyVal",
   };
 
   const testObjLiteral2 = <
     Immutable.Map<
       string,
       any,
       number,
       boolean,
       object,
       null,
       undefined,
       never,
       "extra long"
     >
   >{ property1: "myPropertyVal" };
 
   const testArrayLiteral = <Immutable.Map<string, any>>[
-    "first",
-    "second",
-    "third",
+    "first", "second", "third",
   ];
 
   const testArrayLiteral2 = <
     Immutable.Map<
       string,
       any,
       number,
       boolean,
       object,
       null,
       undefined,
       never,
       "extra long"
     >
   >["first", "second", "third"];
 
   const insideFuncCall = myFunc(
     param1,
     <Immutable.Map<string, any>>param2,
-    param3
+    param3,
   );
 }
 
 // https://github.com/prettier/prettier/issues/4168
 function x() {
   const fits = <PermissionsChecker<any> | undefined>(<any>permissions)[type];
   const fitsObjLiteral = <PermissionsChecker<any> | undefined>{ a: "test" };
   const fitsArrayLiteral = <PermissionsChecker<any> | undefined>["t1", "t2"];
 
-  const breakAfterCast = <PermissionsChecker<any> | undefined>(
-    (<any>permissions)[receiverType]
-  );
+  const breakAfterCast = <PermissionsChecker<any> | undefined>(<any>permissions)[
+    receiverType
+  ];
 
   const stillTooLong = <
     PermissionsChecker<object> | undefined | number | string | boolean
   >(<any>permissions)[receiverType];
 
   const stillTooLong2 = <
-    | PermissionsChecker<object>
-    | undefined
-    | number
-    | string
-    | boolean
-    | null
-    | never
+      | PermissionsChecker<object>
+      | undefined
+      | number
+      | string
+      | boolean
+      | null
+      | never
   >(<any>permissions)[receiverType];
 
   const stillTooLong3 = <PermissionsChecker<object> | undefined>(
-    (<any>permissions.someMethodWithLongName(parameter1, parameter2))[
-      receiverTypeLongName
-    ]
-  );
+    <any>permissions.someMethodWithLongName(parameter1, parameter2)
+  )[receiverTypeLongName];
 
   const stillTooLong4 = <
-    | PermissionsChecker<object>
-    | undefined
-    | number
-    | string
-    | boolean
-    | null
-    | never
+      | PermissionsChecker<object>
+      | undefined
+      | number
+      | string
+      | boolean
+      | null
+      | never
   >(<any>permissions.someMethodWithLongName(parameter1, parameter2))[
     receiverTypeLongName
   ];
 
   const testObjLiteral = <PermissionsChecker<any> | undefined>{
     prop1: "myPropVal",
   };
 
   const testObjLiteral2 = <
-    | PermissionsChecker<object>
-    | undefined
-    | number
-    | string
-    | boolean
-    | null
-    | never
-    | object
+      | PermissionsChecker<object>
+      | undefined
+      | number
+      | string
+      | boolean
+      | null
+      | never
+      | object
   >{ prop1: "myPropVal" };
 
   const testArrayLiteral = <PermissionsChecker<any> | undefined>[
-    "first",
-    "second",
-    "third",
+    "first", "second", "third",
   ];
 
   const testArrayLiteral2 = <
-    | PermissionsChecker<object>
-    | undefined
-    | number
-    | string
-    | boolean
-    | null
-    | never
-    | object
+      | PermissionsChecker<object>
+      | undefined
+      | number
+      | string
+      | boolean
+      | null
+      | never
+      | object
   >["first", "second", "third"];
 
   const insideFuncCall = myFunc(
     param1,
     <PermissionsChecker<any> | undefined>param2,
-    param3
+    param3,
   );
 }
```
# typescript/cast/hug-args.ts
```diff
-postMessage(<IActionMessage>{
-  context: item.context,
-  topic: item.topic,
-});
+postMessage(<IActionMessage>{ context: item.context, topic: item.topic });
 
-window.postMessage({
-  context: item.context,
-  topic: item.topic,
-} as IActionMessage);
+window.postMessage(
+  { context: item.context, topic: item.topic } as IActionMessage,
+);
 
-postMessages(<IActionMessage[]>[
-  {
-    context: item.context,
-    topic: item.topic,
-  },
-]);
+postMessages(<IActionMessage[]>[{ context: item.context, topic: item.topic }]);
```
# typescript/class-comment/class-implements.ts
```diff
-class a1
-  extends b // comment
-  implements z
-{
+class a1 extends b implements z {
+  // comment
   constructor() {}
 }
 
 class a2 extends b implements z {
   // comment
   constructor() {}
 }
 
-class a3
-  extends b
-  // comment
-  implements z, y
+class a3 extends b
+  implements
+    // comment
+    z,
+    y
 {
   constructor() {}
 }
 
-class a4
-  extends b
+class a4 extends b
   implements
     z, // comment
     y
 {
   constructor() {}
 }
 
-class a5
-  extends b
+class a5 extends b
   implements
     z, // comment-z
-    y
+    y // comment-y
 {
-  // comment-y
   constructor() {}
 }
 
-class a6
-  extends b
-  // comment-z1
+class a6 extends b
   implements
+    // comment-z1
     z, // comment-z2
     // comment-y1
-    y
+    y // comment-y2
 {
-  // comment-y2
   constructor() {}
 }
 
-class a7
-  extends b
-  // comment-z1
+class a7 extends b
   implements
+    // comment-z1
     z, // comment-z2
     // comment-y1
-    y
+    y // comment-y2
+// comment-y3
 {
-  // comment-y2
-  // comment-y3
   //comment-body
   constructor() {}
 }
 
-class a8
-  extends b // comment-b
-  // comment-z1
+class a8 extends b
+  // comment-b
   implements
+    // comment-z1
     z, // comment-z2
     // comment-y1
-    y
+    y // comment-y2
 {
-  // comment-y2
   constructor() {}
 }
 
-class a9
-  // comment-b1
-  extends b // comment-b2
+class a9 extends
+// comment-b1
+b
+  // comment-b2
   // comment-b3
-  // comment-z1
   implements
+    // comment-z1
     z, // comment-z2
     // comment-y1
-    y
+    y // comment-y2
 {
-  // comment-y2
   constructor() {}
 }
```
# typescript/class-comment/declare.ts
```diff
-declare class a // 1
+declare class a
+  // 1
   // extends b   // 2
-  implements z, x
+  implements
+    z,
+    x // 3
 {
-  // 3
   doo: boolean;
 }
 
 declare class A1<T> // 1
-  // 2
-  extends B<T> {
-  // 3
-}
+// 2
+extends B<T> {} // 3
```
# typescript/class-comment/generic.ts
```diff
 class G1<T> implements IPoly<T> {
   x: T;
 }
 
-class G2<T> // g2
-  implements IPoly<T>
-{
+class G2<T> implements IPoly<T> {
+  // g2
   x: T;
 }
 
-class G3<T> // g3
-  extends U
-  implements IPoly<T>
-{
+class G3<T> extends U implements IPoly<T> {
+  // g3
   x: T;
 }
 
 class G4<
-    T // g4
-  >
-  extends U
-  implements IPoly<T>
-{
+  T, // g4
+> extends U implements IPoly<T> {
   x: T;
 }
```
# typescript/class-comment/misc.ts
```diff
 export class SnapshotLogger {
   constructor(
     retentionPeriod: number = 5 * 60 * 1000, // retain past five minutes
-    snapshotInterval: number = 30 * 1000 // snapshot no more than every 30s
+    snapshotInterval: number = 30 * 1000, // snapshot no more than every 30s
   ) {}
 }
```
# typescript/class/constructor.ts
```diff
 class foo {
   constructor(static a: number) {}
 }
 
 class foo {
   constructor(export a: number) {}
 }
 
 class A {
-  "constructor": typeof A;
+  'constructor': typeof A
   static Foo() {
     return new A();
   }
 }
 
 class B {
   constructor<>() {}
 }
```
# typescript/class/duplicates-access-modifier.ts
```diff
 class Foo {
-  public a;
-  private b;
-  protected c;
-  public d;
-  public e;
+  public public a;
+  private public b;
+  protected private c;
+  public protected d;
+  public protected private e;
 }
```
# typescript/class/empty-method-body.ts
```diff
 // #9324
 
 class foo1 {
-  bar /* bat */();
+  bar() /* bat */ ;
 }
 
 // #9367
 class Test {
-  foo /* 3 */(/* 2 */);
+  foo( /* 2 */ ) /* 3 */ ;
 }
```
# typescript/class/extends_implements.ts
```diff
-class Class
-  extends AbstractClass
-  implements Interface1, Interface2, Interface3, Interface4 {}
+class Class extends AbstractClass
+  implements
+    Interface1,
+    Interface2,
+    Interface3,
+    Interface4
+{}
 
-class ExtendsAbstractClassAndImplementsInterfaces1
-  extends AbstractClass
-  implements Interface1, Interface2, Interface3, Interface4 {}
+class ExtendsAbstractClassAndImplementsInterfaces1 extends AbstractClass
+  implements
+    Interface1,
+    Interface2,
+    Interface3,
+    Interface4
+{}
 
-class ExtendsAbstractClassAndImplementsInterfaces2
-  extends AAAAAAAAAAAAAAbstractClass
-  implements Interface1, Interface2, Interface3, Interface4 {}
+class ExtendsAbstractClassAndImplementsInterfaces2 extends AAAAAAAAAAAAAAbstractClass
+  implements
+    Interface1,
+    Interface2,
+    Interface3,
+    Interface4
+{}
 
-class ExtendsAbstractClassAndImplementsInterfaces3
-  extends AAAAAAAAAAAAAAbstractClass
+class ExtendsAbstractClassAndImplementsInterfaces3 extends AAAAAAAAAAAAAAbstractClass
   implements
     Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7,
-    Interface8 {}
+    Interface8
+{}
 
 class ExtendsAbstractClassAndImplementsInterfaces4 extends AAAAAAAAAAAAAAbstractClass<
   Type1,
   Type2,
   Type3,
   Type4,
   Type5,
   Type6,
   Type7
 > {}
 
-class ExtendsAbstractClassAndImplementsInterfaces5
-  extends AAAAAAAAAAAAAAbstractClass<
-    Type1,
-    Type2,
-    Type3,
-    Type4,
-    Type5,
-    Type6,
-    Type7
-  >
+class ExtendsAbstractClassAndImplementsInterfaces5 extends AAAAAAAAAAAAAAbstractClass<
+  Type1,
+  Type2,
+  Type3,
+  Type4,
+  Type5,
+  Type6,
+  Type7
+>
   implements
     Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
     Interface7,
-    Interface8 {}
+    Interface8
+{}
 
-class ImplementsInterfaceAndExtendsAbstractClass1<Foo>
-  extends FOOOOOOOOOOOOOOOOO
-  implements FOOOOOOOOOOOOOOOOO, BARRRRRRRRRR {}
+class ImplementsInterfaceAndExtendsAbstractClass1<Foo> extends FOOOOOOOOOOOOOOOOO
+  implements
+    FOOOOOOOOOOOOOOOOO,
+    BARRRRRRRRRR
+{}
 
 class Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOOOOOO>
-  implements Foo {}
+  implements
+    Foo
+{}
 
 class ImplementsInterfaceAndExtendsAbstractClass2<
-    TypeArgumentNumberOne,
-    TypeArgumentNumberTwo,
-    TypeArgumentNumberThree
-  >
-  extends FOOOOOOOOOOOOOOOOOO
-  implements BaseInterface {}
+  TypeArgumentNumberOne,
+  TypeArgumentNumberTwo,
+  TypeArgumentNumberThree,
+> extends FOOOOOOOOOOOOOOOOOO implements BaseInterface {}
 
 class ImplementsInterfaceClass1<
   TypeArgumentNumberOne,
   TypeArgumentNumberTwo,
-  TypeArgumentNumberThree
+  TypeArgumentNumberThree,
 > implements BaseInterface {}
 
 class ImplementsInterfaceClassWithComments1<
-    TypeArgumentNumberOne,
-    TypeArgumentNumberTwo,
-    TypeArgumentNumberThree
-  > // comments
-  implements BaseInterface {}
+  TypeArgumentNumberOne,
+  TypeArgumentNumberTwo,
+  TypeArgumentNumberThree,
+> implements BaseInterface {} // comments
```
# typescript/class/generics.ts
```diff
 class<T> implements Map<T> {}
 
 interface AudioBufferList {
-  mBuffers: interop.Reference<any /*AudioBuffer*/>;
+  mBuffers: interop.Reference<any /*AudioBuffer*/ >;
 }
```
# typescript/class/methods.ts
```diff
 class X {
   optionalMethod?() {}
 }
 
 interface Iterable<T> {
-  export [Symbol.iterator](): Iterator<T>;
+  export;
+  [Symbol.iterator](): Iterator<T>;
 }
 
 export class Check {
   private static property = "test";
 }
```
# typescript/classes/break-heritage.ts
```diff
-class loooooooooooooooooooong
-  extends looooooooooooooooooong
-  implements loooooooooooooooooooong
+class loooooooooooooooooooong extends looooooooooooooooooong
+  implements
+    loooooooooooooooooooong
 {
   // leading comment
   property: string;
 }
 
-class loooooooooooooooooooong
-  extends looooooooooooooooooong
-  implements loooooooooooooooooooong
+class loooooooooooooooooooong extends looooooooooooooooooong
+  implements
+    loooooooooooooooooooong
 {
   property: string;
 }
 
-class loooooooooooooooooooong
-  extends looooooooooooooooooong
-  implements loooooooooooooooooooong
+class loooooooooooooooooooong extends looooooooooooooooooong
+  implements
+    loooooooooooooooooooong
 {
   property: string;
 }
 
-class loooooooooooooooooooong
-  extends looooooooooooooooooong
+class loooooooooooooooooooong extends looooooooooooooooooong
   implements
     loooooooooooooooooooong,
     loooooooooooooooooooong,
     loooooooooooooooooooong
 {
   property: string;
 }
```
# typescript/classes/break.ts
```diff
-class MyContractSelectionWidget
-  extends React.Component<void, MyContractSelectionWidgetPropsType, void>
-  implements SomethingLarge
-{
+class MyContractSelectionWidget extends React.Component<
+  void,
+  MyContractSelectionWidgetPropsType,
+  void
+> implements SomethingLarge {
   method() {}
 }
 
-class DisplayObject1
-  extends utils.EventEmitter
-  implements interaction_InteractiveTarget {}
+class DisplayObject1 extends utils.EventEmitter
+  implements
+    interaction_InteractiveTarget
+{}
 
-class DisplayObject2
-  extends utils.EventEmitter
-  implements interaction_InteractiveTarget {}
+class DisplayObject2 extends utils.EventEmitter
+  implements
+    interaction_InteractiveTarget
+{}
 
-class DisplayObject3
-  extends utils.EventEmitter
+class DisplayObject3 extends utils.EventEmitter
   implements
     interaction_InteractiveTarget,
     somethingElse_SomeOtherThing,
-    somethingElseAgain_RunningOutOfNames {}
+    somethingElseAgain_RunningOutOfNames
+{}
 
-class DisplayObject4
-  extends utils.EventEmitter
-  implements interaction_InteractiveTarget {}
+class DisplayObject4 extends utils.EventEmitter
+  implements
+    interaction_InteractiveTarget
+{}
 class Readable extends events.EventEmitter implements NodeJS_ReadableStream {}
-class InMemoryAppender
-  extends log4javascript.Appender
-  implements ICachedLogMessageProvider {}
+class InMemoryAppender extends log4javascript.Appender
+  implements
+    ICachedLogMessageProvider
+{}
 
-class Foo extends Immutable.Record({
-  ipaddress: "",
-}) {
+class Foo extends Immutable.Record({ ipaddress: "" }) {
   ipaddress: string;
 }
 
-export class VisTimelineComponent
-  implements AfterViewInit, OnChanges, OnDestroy {}
+export class VisTimelineComponent implements AfterViewInit, OnChanges, OnDestroy {}
 export class VisTimelineComponent2
   implements
     AfterViewInit,
     OnChanges,
     OnDestroy,
-    AndSomethingReallyReallyLong {}
+    AndSomethingReallyReallyLong
+{}
```
# typescript/comments-2/dangling.ts
```diff
-Thing?.(/* dangling */);
+Thing?.( /* dangling */ );
 declare class Foo extends Qux<string> {
   /* dangling */
 }
```
# typescript/comments-2/issues.ts
```diff
 function f(
   someReallyLongArgument: WithSomeLongType,
-  someReallyLongArgument2: WithSomeLongType
+  someReallyLongArgument2: WithSomeLongType,
   // Trailing comment should stay after
 ) {}
```
# typescript/comments-2/last-arg.ts
```diff
 type f1 = (
-  currentRequest: { a: number }
+  currentRequest: { a: number },
   // TODO this is a very very very very long comment that makes it go > 80 columns
 ) => number;
 
-f2 = (
-  currentRequest: { a: number }
-  // TODO this is a very very very very long comment that makes it go > 80 columns
-): number => {};
+f2 =
+  (
+    currentRequest: { a: number },
+    // TODO this is a very very very very long comment that makes it go > 80 columns
+  ): number => {};
 
-f3 = (
-  currentRequest: { a: number }
-  // TODO this is a very very very very long comment that makes it go > 80 columns
-) => {};
+f3 =
+  (
+    currentRequest: { a: number },
+    // TODO this is a very very very very long comment that makes it go > 80 columns
+  ) => {};
 
-f4 = function (
-  currentRequest: { a: number }
-  // TODO this is a very very very very long comment that makes it go > 80 columns
-) {};
+f4 =
+  function (
+    currentRequest: { a: number },
+    // TODO this is a very very very very long comment that makes it go > 80 columns
+  ) {};
 
 class X {
   f(
-    currentRequest: { a: number }
+    currentRequest: { a: number },
     // TODO this is a very very very very long comment that makes it go > 80 columns
   ) {}
 }
 
 function f5(
-  a: number
+  a: number,
   // some comment here
 ): number {
   return a + 1;
 }
 
 var x = {
   getSectionMode(
     pageMetaData: PageMetaData,
-    sectionMetaData: SectionMetaData
+    sectionMetaData: SectionMetaData,
     /* $FlowFixMe This error was exposed while converting keyMirror
      * to keyMirrorRecursive */
   ): $Enum<SectionMode> {},
 };
 
 class X2 {
   getSectionMode(
     pageMetaData: PageMetaData,
-    sectionMetaData: SectionMetaData = ["unknown"]
+    sectionMetaData: SectionMetaData = ["unknown"],
     /* $FlowFixMe This error was exposed while converting keyMirror
      * to keyMirrorRecursive */
   ): $Enum<SectionMode> {}
 }
```
# typescript/comments/abstract_class.ts
```diff
 abstract class AbstractRule {
   /**
-   * @deprecated
-   * Failures will be filtered based on `tslint:disable` comments by tslint.
-   * This method now does nothing.
-   */
+     * @deprecated
+     * Failures will be filtered based on `tslint:disable` comments by tslint.
+     * This method now does nothing.
+     */
   filterFailures() {}
 }
```
# typescript/comments/abstract_methods.ts
```diff
 abstract class AbstractFoo {
-  abstract method1(/* comment */ arg: string);
+  abstract method1( /* comment */ arg: string);
   abstract method2(
     /* comment */
-    arg: string
+    arg: string,
   );
   abstract method3(
     // comment
-    arg: string
+    arg: string,
   );
 }
```
# typescript/comments/after_jsx_generic.ts
```diff
 let comp = (
   <>
     <Component<number> /* comment1 */></Component>
     <Component<number> foo /* comment2 */></Component>
     <Component<number> /* comment3 */ bar></Component>
     <Component<number> foo /* comment4 */ bar></Component>
 
     <Component<number>
-    // comment5
+      // comment5
     ></Component>
     <Component<number>
       foo
       // comment6
     ></Component>
     <Component<number>
       // comment7
       foo
     ></Component>
     <Component<number>
       foo
       // comment8
       bar
     ></Component>
   </>
 );
```
# typescript/comments/declare_function.ts
```diff
 declare function fn(
-  currentRequest: { a: number }
+  currentRequest: { a: number },
   // TODO this is a very very very very long comment that makes it go > 80 columns
 ): number;
 
-declare function /* foo */ f(/* baz */ a /* taz */); /* bar */
+declare function /* foo */ f( /* baz */ a /* taz */ ) /* bar */ ;
```
# typescript/comments/interface.ts
```diff
 interface Foo {
   bar(
-    currentRequest: { a: number }
+    currentRequest: { a: number },
     // TODO this is a very very very very long comment that makes it go > 80 columns
   ): number;
-
   (
-    currentRequest: { a: number }
+    currentRequest: { a: number },
     // TODO this is a very very very very long comment that makes it go > 80 columns
   ): number;
-
   new (
-    currentRequest: { a: number }
+    currentRequest: { a: number },
     // TODO this is a very very very very long comment that makes it go > 80 columns
   ): number;
-
   foo: {
     x(
-      currentRequest: { a: number }
+      currentRequest: { a: number },
       // TODO this is a very very very very long comment that makes it go > 80 columns
     ): number;
-
     y: (
-      currentRequest: { a: number }
+      currentRequest: { a: number },
       // TODO this is a very very very very long comment that makes it go > 80 columns
     ) => number;
   };
 }
```
# typescript/comments/issues.ts
```diff
 // Adding a comment stops the pretty printing process and everything is
 // squished in a single line afterward
 export type BuckWebSocketMessage =
   | {
-      // Not actually from Buck - this is to let the receiver know that the socket is connected.
-      type: "SocketConnected";
-    }
-  | {
-      type: "BuildProgressUpdated";
-      progressValue: number;
-    }
-  | {
-      type: "BuildFinished";
-      exitCode: number;
-    }
-  | {
-      type: "BuildStarted";
-    }
-  | {
-      type: "ParseStarted";
-    }
-  | {
-      type: "ParseFinished";
-    }
-  | {
-      type: "RunStarted";
-    }
-  | {
-      type: "RunComplete";
-    };
+    // Not actually from Buck - this is to let the receiver know that the socket is connected.
+    type: "SocketConnected";
+  }
+  | { type: "BuildProgressUpdated"; progressValue: number }
+  | { type: "BuildFinished"; exitCode: number }
+  | { type: "BuildStarted" }
+  | { type: "ParseStarted" }
+  | { type: "ParseFinished" }
+  | { type: "RunStarted" }
+  | { type: "RunComplete" };
 
 // Two extra levels of indentation because of the comment
-export type AsyncExecuteOptions = child_process$execFileOpts & {
-  // The contents to write to stdin.
-  stdin?: string;
-  dontLogInNuclide?: boolean;
-};
+export type AsyncExecuteOptions =
+  & child_process$execFileOpts
+  & {
+    // The contents to write to stdin.
+    stdin?: string;
+    dontLogInNuclide?: boolean;
+  };
```
# typescript/comments/jsx.ts
```diff
-var example1 = <div>https://test</div>;
+var example1 = <div>
+	https://test
+</div>;
 
-var example2 = <div>/*test*/</div>;
+var example2 = <div>
+	/*test*/
+</div>;
```
# typescript/comments/location.ts
```diff
-function x({
-  x,
-  y,
-}: {
-  // Hello world.
-  x: string;
-  // Yoyo.
-  y: string;
-}) {}
+function x(
+  { x, y }: {
+    // Hello world.
+    x: string;
+    // Yoyo.
+    y: string;
+  },
+) {}
 
 export interface ApplicationEventData {
   registerBroadcastReceiver(
     onReceiveCallback: (
-      context: any /* android.content.Context */,
-      intent: any /* android.content.Intent */
-    ) => void
+      context: any, /* android.content.Context */
+      intent: any, /* android.content.Intent */
+    ) => void,
   ): void;
 }
 
 export type WrappedFormUtils = {
   getFieldDecorator(
     id: string,
     options?: {
       /** 子节点的值的属性，如 Checkbox 的是 'checked' */
       valuePropName?: string;
       /** 子节点的初始值，类型、可选值均由子节点决定 */
       initialValue?: any;
       /** 收集子节点的值的时机 */
       trigger?: string;
       /** 可以把 onChange 的参数转化为控件的值，例如 DatePicker 可设为：(date, dateString) => dateString */
       getValueFromEvent?: (...args: any[]) => any;
       /** 校验子节点值的时机 */
       validateTrigger?: string | string[];
       /** 校验规则，参见 [async-validator](https://github.com/yiminghe/async-validator) */
       rules?: ValidationRule[];
       /** 是否和其他控件互斥，特别用于 Radio 单选控件 */
       exclusive?: boolean;
-    }
+    },
   ): (node: React.ReactNode) => React.ReactNode;
 };
```
# typescript/comments/mapped_types.ts
```diff
 type A = {
   // commentA
   [a in A]: string;
 };
 
 type B = {
   /* commentB */ [b in B]: string;
 };
 
 type C = {
-  [/* commentC */ c in C]: string;
+  [c in C]: string; /* commentC */
 };
 
 type D = {
-  [d /* commentD */ in D]: string;
+  [d in D]: string; /* commentD */
 };
 
 type E = {
-  [e in /* commentE */ E]: string;
+  [e in E]: string; /* commentE */
 };
 
 type F = {
-  [f in F /* commentF */]: string;
+  [f in F]: string; /* commentF */
 };
 
 type G = {
-  [g in G /* commentG */]: string;
+  [g in G]: string; /* commentG */
 };
 
-type H = { [/* commentH */ h in H]: string };
+type H = {
+  /* commentH */
+  [h in H]: string;
+};
 
-type I = { [/* commentI */ i in I]: string };
+type I = {
+  [i in I]: string; /* commentI */
+};
 
-type J = { [j /* commentJ */ in J]: string };
+type J = {
+  [j in J]: string; /* commentJ */
+};
 
-type K = { [k in /* commentK */ K]: string };
+type K = {
+  [k in K]: string; /* commentK */
+};
 
-type L = { [l in L /* commentL */]: string };
+type L = {
+  [l in L]: string; /* commentL */
+};
 
-type M = { [m in M /* commentG */]: string };
+type M = {
+  [m in M]: string; /* commentG */
+};
```
# typescript/comments/method_types.ts
```diff
 interface foo1 {
-  bar3 /* foo */(/* baz */); // bat
-  bar /* foo */ /* bar */?(/* baz */) /* bat */;
-  bar2 /* foo */(/* baz */) /* bat */;
+  bar3( /* baz */ ); /* foo */ // bat
+  bar?( /* baz */ ); /* foo */ /* bar */ /* bat */
+  bar2( /* baz */ ); /* foo */ /* bat */
 }
 
 interface foo2 {
-  bar /* foo */?(/* bar */ bar: /* baz */ string): /* bat */ string;
+  bar?(bar: /* baz */ string): string; /* foo */ /* bar */ /* bat */
 }
 
 interface foo3 {
-  /* foo */ (/* bar */): /* baz */ string;
+  /* foo */ ( /* bar */ ): string; /* baz */
 }
 
 interface foo4 {
-  /* foo */ (bar: /* bar */ string): /* baz */ string;
+  /* foo */ (bar: /* bar */ string): string; /* baz */
 }
 
 interface foo5 {
-  /* foo */ new (/* bar */ a: /* baz */ string): /* bat */ string;
+  /* foo */ new (a: /* baz */ string): string; /* bar */ /* bat */
 }
 
 interface foo6 {
-  /* foo */ new (/* baz */): /* bar */ /* bat */ string;
+  /* foo */ new ( /* baz */ ): string; /* bar */ /* bat */
 }
 
-type foo7 = /* foo */ (/* bar */) => /* baz */ void;
+type foo7 = /* foo */ ( /* bar */ ) /* baz */ => void;
 
-type foo8 = /* foo */ (a: /* bar */ string) => /* baz */ void;
+type foo8 = /* foo */ (a: /* bar */ string) /* baz */ => void;
 
-let foo9: new (/* bar */) => /* foo */ /* baz */ string;
+let foo9: new /* foo */ ( /* bar */ ) /* baz */ => string;
 
-let foo10: new (/* foo */ a: /* bar */ string) => /* baz */ string;
+let foo10: new /* foo */ (a: /* bar */ string) /* baz */ => string;
 
 abstract class Test {
-  abstract foo12 /* foo */(a: /* bar */ string): /* baz */ void;
+  abstract foo12 /* foo */ (a: /* bar */ string): /* baz */ void;
 
-  abstract foo13 /* foo */(/* bar */); /* baz */
+  abstract foo13 /* foo */ ( /* bar */ ) /* baz */ ;
 }
```
# typescript/comments/methods.ts
```diff
 export class Point {
   /**
-   * Does something.
-   */
+ * Does something.
+ */
   foo() {}
 
   /**
-   * Does something else.
-   */
+     * Does something else.
+     */
   bar() {}
 
   /**
-   * Does
-   * something
-   * much
-   * better
-   * than
-   * the
-   * rest.
-   */
+                 * Does
+                 * something
+                 * much
+                 * better
+                 * than
+                 * the
+                 * rest.
+                 */
   baz() {}
 
   /**
-   * Buzz-Fizz.
-   * Note: This is indented too far.
-   */
+       * Buzz-Fizz.
+       * Note: This is indented too far.
+       */
   fizzBuzz() {}
 
   /**
-   * Turns the given string into pig-latin.
-   */
+       * Turns the given string into pig-latin.
+       */
   pigLatinize(value: string) {
     /**
-     * This is a block comment inside of a method.
-     */
+ * This is a block comment inside of a method.
+ */
   }
 
   /**
-   * One
-   * Two
+        * One
+ * Two
    * Three
-   * Four
+* Four 
    */
   mismatchedIndentation() {}
 
-  inline /* foo*/(/* bar */) /* baz */ {}
+  inline /* foo*/ ( /* bar */ ) /* baz */ {}
 
-  noBody(/* comment */ arg);
+  noBody( /* comment */ arg);
 }
```
# typescript/comments/type-parameters.ts
```diff
-functionName<A /* A comment */>();
-const a: T</* comment */> = 1;
-functionName</* comment */>();
-function foo</* comment */>() {}
+functionName<A /* A comment */ >();
+const a: T< /* comment */ > = 1;
+functionName< /* comment */ >();
+function foo< /* comment */ >() {}
 interface Foo {
-  </* comment */>(arg): any;
+  < /* comment */ >(arg): any;
 }
-type T = </* comment */>(arg) => any;
+type T = < /* comment */ >(arg) => any;
 
-functionName<A>(); // comment
+functionName<
+  A // comment
+>();
 const a: T<
   // comment
 > = 1;
 functionName<
   // comment
 >();
 function foo<
   // comment
 >() {}
 interface Foo {
   <
-    A // comment
-  >(
-    arg
-  ): any;
+    A, // comment
+  >(arg): any;
 }
 type T = <
   // comment
->(
-  arg
-) => any;
+>(arg) => any;
```
# typescript/comments/union.ts
```diff
-type UploadState<E, EM, D> =
-  // The upload hasnt begun yet
+type UploadState<E, EM, D>
+// The upload hasnt begun yet
+=
   | { type: "Not_begun" }
   // The upload timed out
   | { type: "Timed_out" }
   // Failed somewhere on the line
   | { type: "Failed"; error: E; errorMsg: EM }
   // Uploading to aws3 and CreatePostMutation succeeded
   | { type: "Success"; data: D };
 
-type UploadState2<E, EM, D> =
-  // The upload hasnt begun yet
+type UploadState2<E, EM, D>
+// The upload hasnt begun yet
+=
   | A
   // The upload timed out
   | B
   // Failed somewhere on the line
   | C
   // Uploading to aws3 and CreatePostMutation succeeded
   | D;
```
# typescript/compiler/ClassDeclaration22.ts
```diff
 class C {
-  foo();
-  bar() {}
+  "foo"();
+  "bar"() {}
 }
```
# typescript/compiler/castOfAwait.ts
```diff
 // @target: es6
 async function f() {
   <number>await 0;
-  typeof (await 0);
-  void (await 0);
-  await void (<string>typeof (<number>void (await 0)));
+  typeof await 0;
+  void await 0;
+  await void <string>typeof <number>void await 0;
   await await 0;
 }
```
# typescript/compiler/castParentheses.ts
```diff
-﻿class a {
+class a {
   static b: any;
 }
 
-var b = <any>a;
+var b = (<any>a);
 var b = (<any>a).b;
 var b = (<any>a.b).c;
 var b = (<any>a.b()).c;
-var b = <any>new a();
-var b = <any>new a.b();
+var b = (<any>new a());
+var b = (<any>new a.b());
 var b = (<any>new a()).b;
```
# typescript/compiler/castTest.ts
```diff
 var x: any = 0;
 var z = <number>x;
 var y = x + z;
 
 var a = <any>0;
 var b = <boolean>true;
 var s = <string>"";
 
 var ar = <any[]>null;
 
 var f = <(res: number) => void>null;
 
 declare class Point {
   x: number;
   y: number;
   add(dx: number, dy: number): Point;
   mult(p: Point): Point;
   constructor(x: number, y: number);
 }
 
-var p_cast = <Point>{
+var p_cast = <Point>({
   x: 0,
   y: 0,
   add: function (dx, dy) {
     return new Point(this.x + dx, this.y + dy);
   },
   mult: function (p) {
     return p;
   },
-};
+});
```
# typescript/compiler/commentInNamespaceDeclarationWithIdentifierPathName.ts
```diff
-﻿namespace hello.hi.world {
+namespace hello.hi.world {
   function foo() {}
 }
```
# typescript/compiler/commentsInterface.ts
```diff
 interface i2 {
-  foo: (/**param help*/ b: number) => string;
+  foo: ( /**param help*/ b: number) => string;
 }
```
# typescript/compiler/contextualSignatureInstantiation2.ts
```diff
 // dot f g x = f(g(x))
 var dot: <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T) => (_: U) => S;
 dot =
-  <T, S>(f: (_: T) => S) =>
-  <U>(g: (_: U) => T): ((r: U) => S) =>
-  (x) =>
-    f(g(x));
+  <T, S>(f: (_: T) => S) => <U>(g: (_: U) => T): (r: U) => S => (x) => f(g(x));
 var id: <T>(x: T) => T;
 var r23 = dot(id)(id);
```
# typescript/compiler/decrementAndIncrementOperators.ts
```diff
 var x = 0;
 
 // errors
 1++;
 
-1++;
-1--;
+(1)++;
+(1)--;
 
-++1;
---1;
+++(1);
+--(1);
 
 (1 + 2)++;
 (1 + 2)--;
 
 ++(1 + 2);
 --(1 + 2);
 
 (x + x)++;
 (x + x)--;
 
 ++(x + x);
 --(x + x);
 
 //OK
 x++;
 x--;
 
 ++x;
 --x;
 
-x++;
---x;
+(x)++;
+--(x);
 
-x++;
-x--;
+((x))++;
+((x))--;
 
 x[x++]++;
```
# typescript/compiler/downlevelLetConst1.ts
```diff
-const;
+const
```
# typescript/compiler/errorOnInitializerInInterfaceProperty.ts
```diff
 interface Foo {
-  bar: number = 5;
+    bar: number = 5
 }
```
# typescript/compiler/functionOverloadsOnGenericArity1.ts
```diff
 // overloading on arity not allowed
 interface C {
   f<T>(): string;
   f<T, U>(): string;
-
   <T>(): string;
   <T, U>(): string;
-
   new <T>(): string;
   new <T, U>(): string;
 }
```
# typescript/compiler/globalIsContextualKeyword.ts
```diff
 function a() {
   let global = 1;
 }
 function b() {
   class global {}
 }
 
 namespace global {}
 
 function foo(global: number) {}
 
-let obj = {
-  global: "123",
-};
+let obj = { global: "123" };
```
# typescript/compiler/mappedTypeWithCombinedTypeMappers.ts
```diff
 // Repro from #13351
 
 type Meta<T, A> = {
-  [P in keyof T]: {
-    value: T[P];
-    also: A;
-    readonly children: Meta<T[P], A>;
-  };
+  [P in keyof T]: { value: T[P]; also: A; readonly children: Meta<T[P], A> };
 };
 
 interface Input {
   x: string;
   y: number;
 }
 
 declare const output: Meta<Input, boolean>;
 
 const shouldFail: { important: boolean } = output.x.children;
```
# typescript/conditional-types/comments.ts
```diff
-type A = B extends T
-  ? // comment
-    foo
-  : bar;
+type A = B extends T ? foo : bar; // comment
 
 type A = B extends test /* comment
   comment
       comment
-*/
-  ? foo
-  : bar;
+*/ ? foo : bar;
 
-type T = test extends B
-  ? /* comment
+type T = test extends B ? foo : bar; /* comment
           comment
     comment
           comment
   */
-    foo
-  : bar;
 
 type T = test extends B
-  ? /* comment
+  ? foo /* comment
        comment
        comment
        comment
     */
-    foo
   : test extends B
-  ? /* comment
+    ? foo /* comment
   comment
     comment */
-    foo
-  : bar;
+    : bar;
 
 type T = test extends B ? /* comment */ foo : bar;
 
-type T = test extends B
-  ? foo
-  : /* comment
+type T = test extends B ? foo : bar; /* comment
          comment
      comment
            comment
     */
-    bar;
 
 type T = test extends B
   ? foo
-  : /* comment
+  : test extends B
+    /* comment
          comment
      comment
            comment
     */
-  test extends B
-  ? foo
-  : /* comment
+    ? foo
+    : bar; /* comment
   comment
     comment
    */
-    bar;
 
 type T = test extends B ? foo : /* comment */ bar;
 
 type T = test extends B
-  ? test extends B /* c
+  ? test extends B
+    /* c
 c */
     ? foo
     : bar
   : bar;
```
# typescript/conditional-types/conditonal-types.ts
```diff
 export type DeepReadonly<T> = T extends any[]
   ? DeepReadonlyArray<T[number]>
   : T extends object
-  ? DeepReadonlyObject<T>
-  : T;
+    ? DeepReadonlyObject<T>
+    : T;
 
 type NonFunctionPropertyNames<T> = {
   [K in keyof T]: T[K] extends Function ? never : K;
 }[keyof T];
 
 interface DeepReadonlyArray<T> extends ReadonlyArray<DeepReadonly<T>> {}
 
 type DeepReadonlyObject<T> = {
-  readonly [P in NonFunctionPropertyNames<T>]: DeepReadonly<T[P]>;
+    readonly [P in NonFunctionPropertyNames<T>]: DeepReadonly<T[P]>;
 };
 
 type TypeName<T> = T extends string
   ? "string"
   : T extends number
-  ? "number"
-  : T extends boolean
-  ? "boolean"
-  : T extends undefined
-  ? "undefined"
-  : T extends Function
-  ? "function"
-  : "object";
+    ? "number"
+    : T extends boolean
+      ? "boolean"
+      : T extends undefined
+        ? "undefined"
+        : T extends Function
+          ? "function"
+          : "object";
 
 type Type01 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;
-type Type02 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;
-type Type03 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;
-type Type04 = 0 extends (1 extends 2 ? 3 : 4) ? 5 : 6;
+type Type02 = 0 extends ((1 extends 2 ? 3 : 4)) ? 5 : 6;
+type Type03 = 0 extends (((1 extends 2 ? 3 : 4))) ? 5 : 6;
+type Type04 = 0 extends ((((1 extends 2 ? 3 : 4)))) ? 5 : 6;
 type Type05 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;
-type Type06 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;
-type Type07 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;
-type Type08 = (0 extends 1 ? 2 : 3) extends 4 ? 5 : 6;
+type Type06 = ((0 extends 1 ? 2 : 3)) extends 4 ? 5 : 6;
+type Type07 = (((0 extends 1 ? 2 : 3))) extends 4 ? 5 : 6;
+type Type08 = ((((0 extends 1 ? 2 : 3)))) extends 4 ? 5 : 6;
 
 type T1 = () => void extends T ? U : V;
-type T1a = () => void extends T ? U : V;
-type T1b = () => void extends T ? U : V;
+type T1a = () => (void extends T ? U : V);
+type T1b = () => (void) extends T ? U : V;
 type T2 = (() => void) extends T ? U : V;
 
-type U1 = new () => X extends T ? U : V;
-type U1a = new () => X extends T ? U : V;
-type U1b = new () => X extends T ? U : V;
-type U2 = (new () => X) extends T ? U : V;
+type U1 = new() => X extends T ? U : V;
+type U1a = new() => (X extends T ? U : V);
+type U1b = new() => (X) extends T ? U : V;
+type U2 = (new() => X) extends T ? U : V;
```
# typescript/conditional-types/infer-type.ts
```diff
 type TestReturnType<T extends (...args: any[]) => any> = T extends (
   ...args: any[]
-) => infer R
-  ? R
-  : any;
+) => infer R ? R : any;
 
 type Unpacked<T> = T extends (infer U)[]
   ? U
   : T extends (...args: any[]) => infer U
-  ? U
-  : T extends Promise<infer U>
-  ? U
-  : T;
+    ? U
+    : T extends Promise<infer U>
+      ? U
+      : T;
```
# typescript/conditional-types/nested-in-condition.ts
```diff
 type Foo = (
-  ThingamabobberFactory extends AbstractThingamabobberFactory
-    ? GobbledygookProvider
-    : CompositeGobbledygookProvider
-) extends DoubleGobbledygookProvider
-  ? UniqueDalgametreService
-  : CompositeZamazingoResolver;
+  ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider
+) extends DoubleGobbledygookProvider ? UniqueDalgametreService : CompositeZamazingoResolver;
 
 type Foo2 = DoubleGobbledygookProvider extends (
-  ThingamabobberFactory extends AbstractThingamabobberFactory
-    ? GobbledygookProvider
-    : CompositeGobbledygookProvider
-)
-  ? UniqueDalgametreService
-  : CompositeZamazingoResolver;
+  ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider
+) ? UniqueDalgametreService : CompositeZamazingoResolver;
 
 type Foo3 = (
-  ThingamabobberFactory extends AbstractThingamabobberFactory
-    ? GobbledygookProvider
-    : CompositeGobbledygookProvider
+  ThingamabobberFactory extends AbstractThingamabobberFactory ? GobbledygookProvider : CompositeGobbledygookProvider
 ) extends (
-  DoubleGobbledygookProvider extends MockGobbledygookProvider
-    ? MockThingamabobberFactory
-    : ThingamabobberFactory
-)
-  ? UniqueDalgametreService
-  : CompositeZamazingoResolver;
+  DoubleGobbledygookProvider extends MockGobbledygookProvider ? MockThingamabobberFactory : ThingamabobberFactory
+) ? UniqueDalgametreService : CompositeZamazingoResolver;
```
# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractAssignabilityConstructorFunction.ts
```diff
 abstract class A {}
 
-var AAA: new () => A;
+var AAA: new() => A;
 
 AAA = A;
 AAA = "asdf";
```
# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractInstantiations2.ts
```diff
 class A {}
 
 abstract class B {
   foo(): number {
     return this.bar();
   }
   abstract bar(): number;
 }
 
 new B();
 
 var BB: typeof B = B;
 var AA: typeof A = BB;
 new AA();
 
 function constructB(Factory: typeof B) {
   new Factory();
 }
 
 var BB = B;
 new BB();
 
 var x: any = C;
 new x();
 
 class C extends B {}
 
 abstract class D extends B {}
 
 class E extends B {
   bar() {
     return 1;
   }
 }
 
 abstract class F extends B {
   abstract foo(): number;
   bar() {
     return 2;
   }
 }
 
 abstract class G {
   abstract qux(x: number): string;
   abstract qux(): number;
   y: number;
   abstract quz(x: number, y: string): boolean;
 
   abstract nom(): boolean;
   nom(x: number): boolean;
 }
 
 class H {
-  abstract baz(): number;
+  abstract baz() : number;
 }
```
# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractMixedWithModifiers.ts
```diff
 abstract class A {
   abstract foo_a();
 
   public abstract foo_b();
   protected abstract foo_c();
   private abstract foo_d();
 
-  public abstract foo_bb();
-  protected abstract foo_cc();
-  private abstract foo_dd();
+  abstract public foo_bb();
+  abstract protected foo_cc();
+  abstract private foo_dd();
 
-  static abstract foo_d();
+  abstract static foo_d();
 
   static abstract foo_e();
 }
```
# typescript/conformance/classes/classDeclarations/classAbstractKeyword/classAbstractProperties.ts
```diff
 abstract class A {
   abstract x: number;
   public abstract y: number;
   protected abstract z: number;
-  private abstract w: number;
+  private abstract w : number;
 
   abstract m: () => void;
 
   abstract foo_x(): number;
   public abstract foo_y(): number;
   protected abstract foo_z(): number;
-  private abstract foo_w(): number;
+  private abstract foo_w() : number;
 }
```
# typescript/conformance/classes/classExpression.ts
```diff
 var x = class C {};
 
-var y = {
-  foo: class C2 {},
-};
+var y = { foo: class C2 {} };
 
 var z = class C4 {};
```
# typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyInConstructorParameters.ts
```diff
 class C {
   constructor(readonly x: number) {}
 }
 new C(1).x = 2;
 
 class E {
-  constructor(public readonly x: number) {}
+  constructor(readonly public x: number) {}
 }
 
 class F {
   constructor(private readonly x: number) {}
 }
 new F(1).x;
```
# typescript/conformance/classes/constructorDeclarations/constructorParameters/readonlyReadonly.ts
```diff
 class C {
-  readonly x: number;
-  constructor(readonly y: number) {}
+  readonly readonly x: number;
+  constructor(readonly readonly y: number) {}
 }
```
# typescript/conformance/classes/mixinAccessModifiers.ts
```diff
 // @declaration: true
 
-type Constructable = new (...args: any[]) => object;
+type Constructable = new(...args: any[]) => object;
 
 class Private {
   constructor(...args: any[]) {}
   private p: string;
 }
 
 class Private2 {
   constructor(...args: any[]) {}
   private p: string;
 }
 
 class Protected {
   constructor(...args: any[]) {}
   protected p: string;
   protected static s: string;
 }
 
 class Protected2 {
   constructor(...args: any[]) {}
   protected p: string;
   protected static s: string;
 }
 
 class Public {
   constructor(...args: any[]) {}
   public p: string;
   public static s: string;
 }
 
 class Public2 {
   constructor(...args: any[]) {}
   public p: string;
   public static s: string;
 }
 
 function f1(x: Private & Private2) {
   x.p; // Error, private constituent makes property inaccessible
 }
 
 function f2(x: Private & Protected) {
   x.p; // Error, private constituent makes property inaccessible
 }
 
 function f3(x: Private & Public) {
   x.p; // Error, private constituent makes property inaccessible
 }
 
 function f4(x: Protected & Protected2) {
   x.p; // Error, protected when all constituents are protected
 }
 
 function f5(x: Protected & Public) {
   x.p; // Ok, public if any constituent is public
 }
 
 function f6(x: Public & Public2) {
   x.p; // Ok, public if any constituent is public
 }
 
 declare function Mix<T, U>(c1: T, c2: U): T & U;
 
 // Can't derive from type with inaccessible properties
 
 class C1 extends Mix(Private, Private2) {}
 class C2 extends Mix(Private, Protected) {}
 class C3 extends Mix(Private, Public) {}
 
 class C4 extends Mix(Protected, Protected2) {
   f(c4: C4, c5: C5, c6: C6) {
     c4.p;
     c5.p;
     c6.p;
   }
   static g() {
     C4.s;
     C5.s;
     C6.s;
   }
 }
 
 class C5 extends Mix(Protected, Public) {
   f(c4: C4, c5: C5, c6: C6) {
     c4.p; // Error, not in class deriving from Protected2
     c5.p;
     c6.p;
   }
   static g() {
     C4.s; // Error, not in class deriving from Protected2
     C5.s;
     C6.s;
   }
 }
 
 class C6 extends Mix(Public, Public2) {
   f(c4: C4, c5: C5, c6: C6) {
     c4.p; // Error, not in class deriving from Protected2
     c5.p;
     c6.p;
   }
   static g() {
     C4.s; // Error, not in class deriving from Protected2
     C5.s;
     C6.s;
   }
 }
```
# typescript/conformance/classes/mixinClassesAnnotated.ts
```diff
 // @declaration: true
 
-type Constructor<T> = new (...args: any[]) => T;
+type Constructor<T> = new(...args: any[]) => T;
 
 class Base {
   constructor(public x: number, public y: number) {}
 }
 
 class Derived extends Base {
   constructor(x: number, y: number, public z: number) {
     super(x, y);
   }
 }
 
-const Printable = <T extends Constructor<Base>>(
-  superClass: T
-): Constructor<Printable> & { message: string } & T =>
+const Printable = <T extends Constructor<Base>>(superClass: T):
+  & Constructor<Printable>
+  & { message: string }
+  & T =>
   class extends superClass {
     static message = "hello";
     print() {
       const output = this.x + "," + this.y;
     }
   };
 
-function Tagged<T extends Constructor<{}>>(
-  superClass: T
-): Constructor<Tagged> & T {
+function Tagged<T extends Constructor<{}>>(superClass: T):
+  & Constructor<Tagged>
+  & T {
   class C extends superClass {
     _tag: string;
     constructor(...args: any[]) {
       super(...args);
       this._tag = "hello";
     }
   }
   return C;
 }
 
 const Thing1 = Tagged(Derived);
 const Thing2 = Tagged(Printable(Derived));
 Thing2.message;
 
 function f1() {
   const thing = new Thing1(1, 2, 3);
   thing.x;
   thing._tag;
 }
 
 function f2() {
   const thing = new Thing2(1, 2, 3);
   thing.x;
   thing._tag;
   thing.print();
 }
 
 class Thing3 extends Thing2 {
   constructor(tag: string) {
     super(10, 20, 30);
     this._tag = tag;
   }
   test() {
     this.print();
   }
 }
```
# typescript/conformance/classes/mixinClassesAnonymous.ts
```diff
-type Constructor<T> = new (...args: any[]) => T;
+type Constructor<T> = new(...args: any[]) => T;
 
 class Base {
   constructor(public x: number, public y: number) {}
 }
 
 class Derived extends Base {
   constructor(x: number, y: number, public z: number) {
     super(x, y);
   }
 }
 
 const Printable = <T extends Constructor<Base>>(superClass: T) =>
   class extends superClass {
     static message = "hello";
     print() {
       const output = this.x + "," + this.y;
     }
   };
 
 function Tagged<T extends Constructor<{}>>(superClass: T) {
   class C extends superClass {
     _tag: string;
     constructor(...args: any[]) {
       super(...args);
       this._tag = "hello";
     }
   }
   return C;
 }
 
 const Thing1 = Tagged(Derived);
 const Thing2 = Tagged(Printable(Derived));
 Thing2.message;
 
 function f1() {
   const thing = new Thing1(1, 2, 3);
   thing.x;
   thing._tag;
 }
 
 function f2() {
   const thing = new Thing2(1, 2, 3);
   thing.x;
   thing._tag;
   thing.print();
 }
 
 class Thing3 extends Thing2 {
   constructor(tag: string) {
     super(10, 20, 30);
     this._tag = tag;
   }
   test() {
     this.print();
   }
 }
 
 // Repro from #13805
 
 const Timestamped = <CT extends Constructor<object>>(Base: CT) => {
   return class extends Base {
     timestamp = new Date();
   };
 };
```
# typescript/conformance/es6/templates/templateStringWithEmbeddedTypeAssertionOnAdditionES6.ts
```diff
-﻿// @target: ES6
+// @target: ES6
 var x = `abc${<any>(10 + 10)}def`;
```
# typescript/conformance/expressions/functionCalls/callWithSpreadES6.ts
```diff
 // @target: ES6
 
 interface X {
   foo(x: number, y: number, ...z: string[]);
 }
 
 function foo(x: number, y: number, ...z: string[]) {}
 
 var a: string[];
 var z: number[];
 var obj: X;
 var xa: X[];
 
 foo(1, 2, "abc");
 foo(1, 2, ...a);
 foo(1, 2, ...a, "abc");
 
 obj.foo(1, 2, "abc");
 obj.foo(1, 2, ...a);
 obj.foo(1, 2, ...a, "abc");
 
-obj.foo(1, 2, "abc");
-obj.foo(1, 2, ...a);
-obj.foo(1, 2, ...a, "abc");
+(obj.foo)(1, 2, "abc");
+(obj.foo)(1, 2, ...a);
+(obj.foo)(1, 2, ...a, "abc");
 
 xa[1].foo(1, 2, "abc");
 xa[1].foo(1, 2, ...a);
 xa[1].foo(1, 2, ...a, "abc");
 
 (<Function>xa[1].foo)(...[1, 2, "abc"]);
 
 class C {
   constructor(x: number, y: number, ...z: string[]) {
     this.foo(x, y);
     this.foo(x, y, ...z);
   }
   foo(x: number, y: number, ...z: string[]) {}
 }
 
 class D extends C {
   constructor() {
     super(1, 2);
     super(1, 2, ...a);
   }
   foo() {
     super.foo(1, 2);
     super.foo(1, 2, ...a);
   }
 }
```
# typescript/conformance/interfaces/interfaceDeclarations/interfaceWithMultipleBaseTypes2.ts
```diff
 interface Base {
-  x: {
-    a?: string;
-    b: string;
-  };
+  x: { a?: string; b: string };
 }
 
 interface Base2 {
-  x: {
-    b: string;
-    c?: number;
-  };
+  x: { b: string; c?: number };
 }
 
 interface Derived extends Base, Base2 {
   x: { b: string };
 }
 
 interface Derived2 extends Base, Base2 {
   x: { a: number; b: string };
 }
 
 interface Derived3 extends Base, Base2 {
   x: { a: string; b: string };
 }
```
# typescript/conformance/internalModules/importDeclarations/invalidImportAliasIdentifiers.ts
```diff
 // none of these should work, since non are actually modules
 
 var V = 12;
 
 import v = V;
 
 class C {
   name: string;
 }
 
 import c = C;
 
-enum E {
-  Red,
-  Blue,
-}
+enum E { Red, Blue }
 
 import e = E;
 
 interface I {
   id: number;
 }
 
 import i = I;
```
# typescript/conformance/parser/ecmascript5/Statements/parserES5ForOfStatement21.ts
```diff
 //@target: ES5
-for (var of of) {
-}
+for (var of of) { }
```
# typescript/conformance/types/abstractKeyword/abstractKeyword.ts
```diff
-abstract interface I {}
+abstract;
+interface I {}
```
# typescript/conformance/types/constKeyword/constKeyword.ts
```diff
-const enum E {
-  A,
-  B,
-  C,
-}
+const enum E { A, B, C }
```
# typescript/conformance/types/constructorType/cunstructorType.ts
```diff
-var d: new (x: number) => void;
+var d: new(x: number) => void;
```
# typescript/conformance/types/enumDeclaration/enumDeclaration.ts
```diff
-enum E {
-  A,
-  B,
-  C,
-}
+enum E { A, B, C }
```
# typescript/conformance/types/firstTypeNode/firstTypeNode.ts
```diff
 export function fooWithTypePredicate(a: any): a is number {
   return true;
 }
-export function fooWithTypePredicateAndMulitpleParams(
-  a: any,
-  b: any,
-  c: any
-): a is number {
+export function fooWithTypePredicateAndMulitpleParams(a: any, b: any, c: any): a is number {
   return true;
 }
 export function fooWithTypeTypePredicateAndGeneric<T>(a: any): a is T {
   return true;
 }
-export function fooWithTypeTypePredicateAndRestParam(
-  a: any,
-  ...rest
-): a is number {
+export function fooWithTypeTypePredicateAndRestParam(a: any, ...rest): a is number {
   return true;
 }
```
# typescript/conformance/types/functions/TSFunctionTypeNoUnnecessaryParentheses.ts
```diff
 class Foo {
-  bar: () => boolean;
+  bar: (() => boolean);
 }
```
# typescript/conformance/types/functions/functionImplementationErrors.ts
```diff
 // @allowUnreachableCode: true
 
 // FunctionExpression with no return type annotation with multiple return statements with unrelated types
 var f1 = function () {
   return "";
   return 3;
 };
 var f2 = function x() {
   return "";
   return 3;
 };
 var f3 = () => {
   return "";
   return 3;
 };
 
 // FunctionExpression with no return type annotation with return branch of number[] and other of string[]
 var f4 = function () {
   if (true) {
     return [""];
   } else {
     return [1];
   }
 };
 
 // Function implementation with non -void return type annotation with no return
 function f5(): number {}
 
 var m;
 // Function signature with parameter initializer referencing in scope local variable
 function f6(n = m) {
   var m = 4;
 }
 
 // Function signature with initializer referencing other parameter to the right
 function f7(n = m, m?) {}
 
 // FunctionExpression with non -void return type annotation with a throw, no return, and other code
 // Should be error but isn't
-undefined ===
-  function (): number {
-    throw undefined;
-    var x = 4;
-  };
+undefined === function (): number {
+  throw undefined;
+  var x = 4;
+};
 
 class Base {
   private x;
 }
 class AnotherClass {
   private y;
 }
 class Derived1 extends Base {
   private m;
 }
 class Derived2 extends Base {
   private n;
 }
 function f8() {
   return new Derived1();
   return new Derived2();
 }
 var f9 = function () {
   return new Derived1();
   return new Derived2();
 };
 var f10 = () => {
   return new Derived1();
   return new Derived2();
 };
 function f11() {
   return new Base();
   return new AnotherClass();
 }
 var f12 = function () {
   return new Base();
   return new AnotherClass();
 };
 var f13 = () => {
   return new Base();
   return new AnotherClass();
 };
```
# typescript/conformance/types/functions/functionImplementations.ts
```diff
 // @allowUnreachableCode: true
 
 // FunctionExpression with no return type annotation and no return statement returns void
-var v: void = (function () {})();
+var v: void = function () {}();
 
 // FunctionExpression f with no return type annotation and directly references f in its body returns any
 var a: any = function f() {
   return f;
 };
 var a: any = function f() {
   return f();
 };
 
 // FunctionExpression f with no return type annotation and indirectly references f in its body returns any
 var a: any = function f() {
   var x = f;
   return x;
 };
 
 // Two mutually recursive function implementations with no return type annotations
 function rec1() {
   return rec2();
 }
 function rec2() {
   return rec1();
 }
 var a = rec1();
 var a = rec2();
 
 // Two mutually recursive function implementations with return type annotation in one
 function rec3(): number {
   return rec4();
 }
 function rec4() {
   return rec3();
 }
 var n: number;
 var n = rec3();
 var n = rec4();
 
 // FunctionExpression with no return type annotation and returns a number
-var n = (function () {
+var n = function () {
   return 3;
-})();
+}();
 
 // FunctionExpression with no return type annotation and returns null
 var nu = null;
-var nu = (function () {
+var nu = function () {
   return null;
-})();
+}();
 
 // FunctionExpression with no return type annotation and returns undefined
 var un = undefined;
-var un = (function () {
+var un = function () {
   return undefined;
-})();
+}();
 
 // FunctionExpression with no return type annotation and returns a type parameter type
-var n = (function <T>(x: T) {
+var n = function <T>(x: T) {
   return x;
-})(4);
+}(4);
 
 // FunctionExpression with no return type annotation and returns a constrained type parameter type
-var n = (function <T extends {}>(x: T) {
+var n = function <T extends {}>(x: T) {
   return x;
-})(4);
+}(4);
 
 // FunctionExpression with no return type annotation with multiple return statements with identical types
-var n = (function () {
+var n = function () {
   return 3;
   return 5;
-})();
+}();
 
 // Otherwise, the inferred return type is the first of the types of the return statement expressions
 // in the function body that is a supertype of each of the others,
 // ignoring return statements with no expressions.
 // A compile - time error occurs if no return statement expression has a type that is a supertype of each of the others.
 // FunctionExpression with no return type annotation with multiple return statements with subtype relation between returns
 class Base {
   private m;
 }
 class Derived extends Base {
   private q;
 }
 var b: Base;
-var b = (function () {
+var b = function () {
   return new Base();
   return new Derived();
-})();
+}();
 
 // FunctionExpression with no return type annotation with multiple return statements with one a recursive call
-var a = (function f() {
+var a = function f() {
   return new Base();
   return new Derived();
   return f(); // ?
-})();
+}();
 
 // FunctionExpression with non -void return type annotation with a single throw statement
-undefined ===
-  function (): number {
-    throw undefined;
-  };
+undefined === function (): number {
+  throw undefined;
+};
 
 // Type of 'this' in function implementation is 'any'
 function thisFunc() {
   var x = this;
   var x: any;
 }
 
 // Function signature with optional parameter, no type annotation and initializer has initializer's type
 function opt1(n = 4) {
   var m = n;
   var m: number;
 }
 
 // Function signature with optional parameter, no type annotation and initializer has initializer's widened type
 function opt2(n = { x: null, y: undefined }) {
   var m = n;
   var m: { x: any; y: any };
 }
 
 // Function signature with initializer referencing other parameter to the left
 function opt3(n: number, m = n) {
   var y = m;
   var y: number;
 }
 
 // Function signature with optional parameter has correct codegen
 // (tested above)
 
 // FunctionExpression with non -void return type annotation return with no expression
 function f6(): number {
   return;
 }
 
 class Derived2 extends Base {
   private r: string;
 }
 class AnotherClass {
   private x;
 }
 // if f is a contextually typed function expression, the inferred return type is the union type
 // of the types of the return statement expressions in the function body,
 // ignoring return statements with no expressions.
 var f7: (x: number) => string | number = (x) => {
   // should be (x: number) => number | string
   if (x < 0) {
     return x;
   }
   return x.toString();
 };
 var f8: (x: number) => any = (x) => {
   // should be (x: number) => Base
   return new Base();
   return new Derived2();
 };
 var f9: (x: number) => any = (x) => {
   // should be (x: number) => Base
   return new Base();
   return new Derived();
   return new Derived2();
 };
 var f10: (x: number) => any = (x) => {
   // should be (x: number) => Derived | Derived1
   return new Derived();
   return new Derived2();
 };
 var f11: (x: number) => any = (x) => {
   // should be (x: number) => Base | AnotherClass
   return new Base();
   return new AnotherClass();
 };
 var f12: (x: number) => any = (x) => {
   // should be (x: number) => Base | AnotherClass
   return new Base();
   return; // should be ignored
   return new AnotherClass();
 };
```
# typescript/conformance/types/functions/parameterInitializersForwardReferencing.ts
```diff
 function left(a, b = a, c = b) {
   a;
   b;
 }
 
 function right(a = b, b = a) {
   a;
   b;
 }
 
 function right2(a = b, b = c, c = a) {
   a;
   b;
   c;
 }
 
 function inside(a = b) {
   var b;
 }
 
 function outside() {
   var b;
   function inside(a = b) {
     // Still an error because b is declared inside the function
     var b;
   }
 }
 
 function defaultArgFunction(
   a = function () {
     return b;
   },
-  b = 1
+  b = 1,
 ) {}
 function defaultArgArrow(a = () => () => b, b = 3) {}
 
 class C {
   constructor(a = b, b = 1) {}
   method(a = b, b = 1) {}
 }
 
 // Function expressions
 var x = (a = b, b = c, c = d) => {
   var d;
 };
 
 // Should not produce errors - can reference later parameters if they occur within a function expression initializer.
 function f(
   a,
   b = function () {
     return c;
   },
-  c = b()
+  c = b(),
 ) {}
```
# typescript/conformance/types/indexedAccesType/indexedAccesType.ts
```diff
-const a: Foo["bar"] = {
-  baz: "yawp",
-};
+const a: Foo["bar"] = { baz: "yawp" };
```
# typescript/conformance/types/mappedType/mappedType.ts
```diff
 type Keys = "option1" | "option2";
-type Flags = { [K in Keys]: boolean };
+type Flags = {
+  [K in Keys]: boolean;
+};
```
# typescript/conformance/types/methodSignature/methodSignature.ts
```diff
-var logger: {
-  log(val: any, val2: any);
-  error(val: any);
-};
+var logger: { log(val: any, val2: any); error(val: any) };
```
# typescript/conformance/types/parameterProperty/parameterProperty.ts
```diff
 class c3 {
   constructor(public arg: number = 10) {
     // fails because of comment
   }
-  not_constructor(public arg: number = 10) {}
+  not_constructor(public arg: number = 10) {
+  }
 }
```
# typescript/conformance/types/tuple/contextualTypeWithTuple.ts
```diff
-﻿// no error
+// no error
 var numStrTuple: [number, string] = [5, "hello"];
 var numStrTuple2: [number, string] = [5, "foo", true];
 var numStrBoolTuple: [number, string, boolean] = [5, "foo", true];
 var objNumTuple: [{ a: string }, number] = [{ a: "world" }, 5];
 var strTupleTuple: [string, [number, {}]] = ["bar", [5, { x: 1, y: 1 }]];
 class C {}
 class D {}
 var unionTuple: [C, string | number] = [new C(), "foo"];
 var unionTuple1: [C, string | number] = [new C(), "foo"];
 var unionTuple2: [C, string | number, D] = [new C(), "foo", new D()];
 var unionTuple3: [number, string | number] = [10, "foo"];
 
 numStrTuple = numStrTuple2;
 numStrTuple = numStrBoolTuple;
 
 // error
 objNumTuple = [{}, 5];
 numStrBoolTuple = numStrTuple;
 var strStrTuple: [string, string] = ["foo", "bar", 5];
 
 unionTuple = unionTuple1;
 unionTuple = unionTuple2;
 unionTuple2 = unionTuple;
 numStrTuple = unionTuple3;
```
# typescript/conformance/types/tuple/indexerWithTuple.ts
```diff
-﻿var strNumTuple: [string, number] = ["foo", 10];
+var strNumTuple: [string, number] = ["foo", 10];
 var numTupleTuple: [number, [string, number]] = [10, ["bar", 20]];
 var unionTuple1: [number, string | number] = [10, "foo"];
 var unionTuple2: [boolean, string | number] = [true, "foo"];
 
 // no error
 var idx0 = 0;
 var idx1 = 1;
 var ele10 = strNumTuple[0]; // string
 var ele11 = strNumTuple[1]; // number
 var ele12 = strNumTuple[2]; // string | number
 var ele13 = strNumTuple[idx0]; // string | number
 var ele14 = strNumTuple[idx1]; // string | number
 var ele15 = strNumTuple["0"]; // string
 var ele16 = strNumTuple["1"]; // number
 var strNumTuple1 = numTupleTuple[1]; //[string, number];
 var ele17 = numTupleTuple[2]; // number | [string, number]
 var eleUnion10 = unionTuple1[0]; // number
 var eleUnion11 = unionTuple1[1]; // string | number
 var eleUnion12 = unionTuple1[2]; // string | number
 var eleUnion13 = unionTuple1[idx0]; // string | number
 var eleUnion14 = unionTuple1[idx1]; // string | number
 var eleUnion15 = unionTuple1["0"]; // number
 var eleUnion16 = unionTuple1["1"]; // string | number
 
 var eleUnion20 = unionTuple2[0]; // boolean
 var eleUnion21 = unionTuple2[1]; // string | number
 var eleUnion22 = unionTuple2[2]; // string | number | boolean
 var eleUnion23 = unionTuple2[idx0]; // string | number | boolean
 var eleUnion24 = unionTuple2[idx1]; // string | number | boolean
 var eleUnion25 = unionTuple2["0"]; // boolean
 var eleUnion26 = unionTuple2["1"]; // string | number
```
# typescript/conformance/types/tuple/typeInferenceWithTupleType.ts
```diff
-﻿function combine<T, U>(x: T, y: U): [T, U] {
+function combine<T, U>(x: T, y: U): [T, U] {
   return [x, y];
 }
 
 var combineResult = combine("string", 10);
 var combineEle1 = combineResult[0]; // string
 var combineEle2 = combineResult[1]; // number
 
 function zip<T, U>(array1: T[], array2: U[]): [[T, U]] {
   if (array1.length != array2.length) {
     return [[undefined, undefined]];
   }
   var length = array1.length;
   var zipResult: [[T, U]];
   for (var i = 0; i < length; ++i) {
     zipResult.push([array1[i], array2[i]]);
   }
   return zipResult;
 }
 
 var zipResult = zip(["foo", "bar"], [5, 6]);
 var zipResultEle = zipResult[0]; // [string, number]
 var zipResultEleEle = zipResult[0][0]; // string
```
# typescript/conformance/types/tuple/wideningTuples3.ts
```diff
 //@noImplicitAny: true
 var a: [any];
 
-var b = (a = [undefined, null]);
+var b = a = [undefined, null];
```
# typescript/conformance/types/tuple/wideningTuples4.ts
```diff
 var a: [any];
 
-var b = (a = [undefined, null]);
+var b = a = [undefined, null];
 b = ["", ""];
```
# typescript/conformance/types/tuple/wideningTuples7.ts
```diff
 //@noImplicitAny: true
 var foo = function bar() {
   let intermediate: [string];
-  return (intermediate = [undefined]);
+  return intermediate = [undefined];
 };
```
# typescript/conformance/types/union/unionTypeCallSignatures.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var strOrBoolean: string | boolean;
 var strOrNum: string | number;
 
 // If each type in U has call signatures and the sets of call signatures are identical ignoring return types,
 // U has the same set of call signatures, but with return types that are unions of the return types of the respective call signatures from each type in U.
 var unionOfDifferentReturnType: { (a: number): number } | { (a: number): Date };
 numOrDate = unionOfDifferentReturnType(10);
 strOrBoolean = unionOfDifferentReturnType("hello"); // error
 unionOfDifferentReturnType1(true); // error in type of parameter
 
 var unionOfDifferentReturnType1:
   | { (a: number): number; (a: string): string }
   | { (a: number): Date; (a: string): boolean };
 numOrDate = unionOfDifferentReturnType1(10);
 strOrBoolean = unionOfDifferentReturnType1("hello");
 unionOfDifferentReturnType1(true); // error in type of parameter
 unionOfDifferentReturnType1(); // error missing parameter
 
 var unionOfDifferentParameterTypes:
   | { (a: number): number }
   | { (a: string): Date };
 unionOfDifferentParameterTypes(10); // error - no call signatures
 unionOfDifferentParameterTypes("hello"); // error - no call signatures
 unionOfDifferentParameterTypes(); // error - no call signatures
 
 var unionOfDifferentNumberOfSignatures:
   | { (a: number): number }
   | { (a: number): Date; (a: string): boolean };
 unionOfDifferentNumberOfSignatures(); // error - no call signatures
 unionOfDifferentNumberOfSignatures(10); // error - no call signatures
 unionOfDifferentNumberOfSignatures("hello"); // error - no call signatures
 
 var unionWithDifferentParameterCount:
   | { (a: string): string }
   | { (a: string, b: number): number };
 unionWithDifferentParameterCount(); // no  call signature
 unionWithDifferentParameterCount("hello"); // no  call signature
 unionWithDifferentParameterCount("hello", 10); // no  call signature
 
 var unionWithOptionalParameter1:
   | { (a: string, b?: number): string }
   | { (a: string, b?: number): number };
 strOrNum = unionWithOptionalParameter1("hello");
 strOrNum = unionWithOptionalParameter1("hello", 10);
 strOrNum = unionWithOptionalParameter1("hello", "hello"); // error in parameter type
 strOrNum = unionWithOptionalParameter1(); // error
 
 var unionWithOptionalParameter2:
   | { (a: string, b?: number): string }
   | { (a: string, b: number): number };
 strOrNum = unionWithOptionalParameter2("hello"); // error no call signature
 strOrNum = unionWithOptionalParameter2("hello", 10); // error no call signature
 strOrNum = unionWithOptionalParameter2("hello", "hello"); // error no call signature
 strOrNum = unionWithOptionalParameter2(); // error no call signature
 
 var unionWithOptionalParameter3:
   | { (a: string, b?: number): string }
   | { (a: string): number };
 strOrNum = unionWithOptionalParameter3("hello");
 strOrNum = unionWithOptionalParameter3("hello", 10); // error no call signature
 strOrNum = unionWithOptionalParameter3("hello", "hello"); // error no call signature
 strOrNum = unionWithOptionalParameter3(); // error no call signature
 
 var unionWithRestParameter1:
   | { (a: string, ...b: number[]): string }
   | { (a: string, ...b: number[]): number };
 strOrNum = unionWithRestParameter1("hello");
 strOrNum = unionWithRestParameter1("hello", 10);
 strOrNum = unionWithRestParameter1("hello", 10, 11);
 strOrNum = unionWithRestParameter1("hello", "hello"); // error in parameter type
 strOrNum = unionWithRestParameter1(); // error
 
 var unionWithRestParameter2:
   | { (a: string, ...b: number[]): string }
   | { (a: string, b: number): number };
 strOrNum = unionWithRestParameter2("hello"); // error no call signature
 strOrNum = unionWithRestParameter2("hello", 10); // error no call signature
 strOrNum = unionWithRestParameter2("hello", 10, 11); // error no call signature
 strOrNum = unionWithRestParameter2("hello", "hello"); // error no call signature
 strOrNum = unionWithRestParameter2(); // error no call signature
 
 var unionWithRestParameter3:
   | { (a: string, ...b: number[]): string }
   | { (a: string): number };
 strOrNum = unionWithRestParameter3("hello");
 strOrNum = unionWithRestParameter3("hello", 10); // error no call signature
 strOrNum = unionWithRestParameter3("hello", 10, 11); // error no call signature
 strOrNum = unionWithRestParameter3("hello", "hello"); // error no call signature
 strOrNum = unionWithRestParameter3(); // error no call signature
 
 var unionWithRestParameter4:
   | { (...a: string[]): string }
   | { (a: string, b: string): number };
 strOrNum = unionWithRestParameter4("hello"); // error supplied parameters do not match any call signature
 strOrNum = unionWithRestParameter4("hello", "world");
```
# typescript/conformance/types/union/unionTypeCallSignatures3.ts
```diff
-﻿function f1(s: string) {}
+function f1(s: string) {}
 function f2(s?: string) {}
 function f3(...s: string[]) {}
 function f4(s: string, s2?: string) {}
 function f5(s?: string, n?: number) {}
 function f6(s?: string, ...n: number[]) {}
 function f7(s: string, ...sRest: string[]) {}
 
 var fUnion:
   | typeof f1
   | typeof f2
   | typeof f3
   | typeof f4
   | typeof f5
   | typeof f6
   | typeof f7;
 
 fUnion(""); // All constituents can be called by passing a single string.
```
# typescript/conformance/types/union/unionTypeCallSignatures4.ts
```diff
-﻿type F1 = (a: string, b?: string) => void;
+type F1 = (a: string, b?: string) => void;
 type F2 = (a: string, b?: string, c?: string) => void;
 type F3 = (a: string, ...rest: string[]) => void;
 type F4 = (a: string, b?: string, ...rest: string[]) => void;
 type F5 = (a: string, b: string) => void;
 
 var f12: F1 | F2;
 f12("a");
 f12("a", "b");
 f12("a", "b", "c"); // error
 
 var f34: F3 | F4;
 f34("a");
 f34("a", "b");
 f34("a", "b", "c");
 
 var f1234: F1 | F2 | F3 | F4;
 f1234("a");
 f1234("a", "b");
 f1234("a", "b", "c"); // error
 
 var f12345: F1 | F2 | F3 | F4 | F5;
 f12345("a"); // error
 f12345("a", "b");
 f12345("a", "b", "c"); // error
```
# typescript/conformance/types/union/unionTypeConstructSignatures.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var strOrBoolean: string | boolean;
 var strOrNum: string | number;
 
 // If each type in U has construct signatures and the sets of construct signatures are identical ignoring return types,
 // U has the same set of construct signatures, but with return types that are unions of the return types of the respective construct signatures from each type in U.
 var unionOfDifferentReturnType:
   | { new (a: number): number }
   | { new (a: number): Date };
 numOrDate = new unionOfDifferentReturnType(10);
 strOrBoolean = new unionOfDifferentReturnType("hello"); // error
 new unionOfDifferentReturnType1(true); // error in type of parameter
 
 var unionOfDifferentReturnType1:
   | { new (a: number): number; new (a: string): string }
   | { new (a: number): Date; new (a: string): boolean };
 numOrDate = new unionOfDifferentReturnType1(10);
 strOrBoolean = new unionOfDifferentReturnType1("hello");
 new unionOfDifferentReturnType1(true); // error in type of parameter
 new unionOfDifferentReturnType1(); // error missing parameter
 
 var unionOfDifferentParameterTypes:
   | { new (a: number): number }
   | { new (a: string): Date };
 new unionOfDifferentParameterTypes(10); // error - no call signatures
 new unionOfDifferentParameterTypes("hello"); // error - no call signatures
 new unionOfDifferentParameterTypes(); // error - no call signatures
 
 var unionOfDifferentNumberOfSignatures:
   | { new (a: number): number }
   | { new (a: number): Date; new (a: string): boolean };
 new unionOfDifferentNumberOfSignatures(); // error - no call signatures
 new unionOfDifferentNumberOfSignatures(10); // error - no call signatures
 new unionOfDifferentNumberOfSignatures("hello"); // error - no call signatures
 
 var unionWithDifferentParameterCount:
   | { new (a: string): string }
   | { new (a: string, b: number): number };
 new unionWithDifferentParameterCount(); // no  call signature
 new unionWithDifferentParameterCount("hello"); // no  call signature
 new unionWithDifferentParameterCount("hello", 10); // no  call signature
 
 var unionWithOptionalParameter1:
   | { new (a: string, b?: number): string }
   | { new (a: string, b?: number): number };
 strOrNum = new unionWithOptionalParameter1("hello");
 strOrNum = new unionWithOptionalParameter1("hello", 10);
 strOrNum = new unionWithOptionalParameter1("hello", "hello"); // error in parameter type
 strOrNum = new unionWithOptionalParameter1(); // error
 
 var unionWithOptionalParameter2:
   | { new (a: string, b?: number): string }
   | { new (a: string, b: number): number };
 strOrNum = new unionWithOptionalParameter2("hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter2("hello", 10); // error no call signature
 strOrNum = new unionWithOptionalParameter2("hello", "hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter2(); // error no call signature
 
 var unionWithOptionalParameter3:
   | { new (a: string, b?: number): string }
   | { new (a: string): number };
 strOrNum = new unionWithOptionalParameter3("hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter3("hello", 10); // error no call signature
 strOrNum = new unionWithOptionalParameter3("hello", "hello"); // error no call signature
 strOrNum = new unionWithOptionalParameter3(); // error no call signature
 
 var unionWithRestParameter1:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string, ...b: number[]): number };
 strOrNum = new unionWithRestParameter1("hello");
 strOrNum = new unionWithRestParameter1("hello", 10);
 strOrNum = new unionWithRestParameter1("hello", 10, 11);
 strOrNum = new unionWithRestParameter1("hello", "hello"); // error in parameter type
 strOrNum = new unionWithRestParameter1(); // error
 
 var unionWithRestParameter2:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string, b: number): number };
 strOrNum = new unionWithRestParameter2("hello"); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", 10); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", 10, 11); // error no call signature
 strOrNum = new unionWithRestParameter2("hello", "hello"); // error no call signature
 strOrNum = new unionWithRestParameter2(); // error no call signature
 
 var unionWithRestParameter3:
   | { new (a: string, ...b: number[]): string }
   | { new (a: string): number };
 strOrNum = new unionWithRestParameter3("hello"); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", 10); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", 10, 11); // error no call signature
 strOrNum = new unionWithRestParameter3("hello", "hello"); // error no call signature
 strOrNum = new unionWithRestParameter3(); // error no call signature
```
# typescript/conformance/types/union/unionTypeEquivalence.ts
```diff
-﻿// A | B is equivalent to A if B is a subtype of A
+// A | B is equivalent to A if B is a subtype of A
 class C {}
 class D extends C {
   foo() {}
 }
 var x: C;
 var x: C | D;
 
 // A | B is equivalent to B | A.
 var y: string | number;
 var y: number | string;
 
 // AB | C is equivalent to A | BC, where AB is A | B and BC is B | C.
 var z: string | number | boolean;
 var z: (string | number) | boolean;
 var z: string | (number | boolean);
 var AB: string | number;
 var BC: number | boolean;
 var z1: typeof AB | boolean;
 var z1: string | typeof BC;
```
# typescript/conformance/types/union/unionTypeFromArrayLiteral.ts
```diff
-﻿// The resulting type an array literal expression is determined as follows:
+// The resulting type an array literal expression is determined as follows:
 // If the array literal is empty, the resulting type is an array type with the element type Undefined.
 // Otherwise, if the array literal is contextually typed by a type that has a property with the numeric name ‘0’, the resulting type is a tuple type constructed from the types of the element expressions.
 // Otherwise, the resulting type is an array type with an element type that is the union of the types of the element expressions.
 
 var arr1 = [1, 2]; // number[]
 var arr2 = ["hello", true]; // (string | number)[]
 var arr3Tuple: [number, string] = [3, "three"]; // [number, string]
 var arr4Tuple: [number, string] = [3, "three", "hello"]; // [number, string, string]
 var arrEmpty = [];
-var arr5Tuple: {
-  0: string;
-  5: number;
-} = ["hello", true, false, " hello", true, 10, "any"]; // Tuple
+var arr5Tuple: { 0: string; 5: number } = [
+  "hello", true, false, " hello", true, 10, "any",
+]; // Tuple
 class C {
   foo() {}
 }
 class D {
   foo2() {}
 }
 class E extends C {
   foo3() {}
 }
 class F extends C {
   foo4() {}
 }
 var c: C, d: D, e: E, f: F;
 var arr6 = [c, d]; // (C | D)[]
 var arr7 = [c, d, e]; // (C | D)[]
 var arr8 = [c, e]; // C[]
 var arr9 = [e, f]; // (E|F)[]
```
# typescript/conformance/types/union/unionTypeIndexSignature.ts
```diff
-﻿var numOrDate: number | Date;
+var numOrDate: number | Date;
 var anyVar: number;
 
 // If each type in U has a string index signature,
 // U has a string index signature of a union type of the types of the string index signatures from each type in U.
 
 var unionOfDifferentReturnType: { [a: string]: number } | { [a: string]: Date };
 numOrDate = unionOfDifferentReturnType["hello"]; // number | Date
 numOrDate = unionOfDifferentReturnType[10]; // number | Date
 
-var unionOfTypesWithAndWithoutStringSignature:
-  | { [a: string]: number }
-  | boolean;
+var unionOfTypesWithAndWithoutStringSignature: { [a: string]: number } | boolean;
 anyVar = unionOfTypesWithAndWithoutStringSignature["hello"]; // any
 anyVar = unionOfTypesWithAndWithoutStringSignature[10]; // any
 
 // If each type in U has a numeric index signature,
 // U has a numeric index signature of a union type of the types of the numeric index signatures from each type in U.
-var unionOfDifferentReturnType1:
-  | { [a: number]: number }
-  | { [a: number]: Date };
+var unionOfDifferentReturnType1: { [a: number]: number } | { [a: number]: Date };
 numOrDate = unionOfDifferentReturnType1["hello"]; // any
 numOrDate = unionOfDifferentReturnType1[10]; // number | Date
 
 var unionOfTypesWithAndWithoutStringSignature1:
   | { [a: number]: number }
   | boolean;
 anyVar = unionOfTypesWithAndWithoutStringSignature1["hello"]; // any
 anyVar = unionOfTypesWithAndWithoutStringSignature1[10]; // any
```
# typescript/custom/abstract/abstractProperties.ts
```diff
 abstract class Foo {
-  private abstract a: 1;
+  abstract private a: 1;
   private abstract b: 2;
   static abstract c: 3;
-  private abstract ["g"];
-  private abstract ["h"];
-  static abstract ["i"];
+  abstract private ['g'];
+  private abstract ['h'];
+  static abstract ['i'];
 }
```
# typescript/custom/computedProperties/string.ts
```diff
 interface I {
-  string: "I";
+  "string": "I";
 }
 
-type T = {
-  string: "T";
-};
+type T = { "string": "T" };
 
 interface A {
-  string: "A";
+  "string": "A";
 }
 
-type B = {
-  string: "B";
-};
+type B = { "string": "B" };
```
# typescript/custom/computedProperties/symbol.ts
```diff
 interface I {
   [Symbol.toStringTag]: "I";
 }
 
-type T = {
-  [Symbol.toStringTag]: "T";
-};
+type T = { [Symbol.toStringTag]: "T" };
```
# typescript/custom/modifiers/question.ts
```diff
 var x: {
-  [A in keyof B]?: any;
+    [A in keyof B]?: any;
 };
```
# typescript/custom/modifiers/readonly.ts
```diff
 var x: {
-  readonly [A in keyof B]: any;
+    readonly [A in keyof B]: any;
 };
```
# typescript/custom/new/newKeyword.ts
```diff
-var x: { y: new <T, U>() => [T, U] };
+var x: { y: new<T, U>() => [T, U] };
 
 interface I {
   new <T>(x: string);
   new (x: string);
   new (x: number): number;
 }
```
# typescript/custom/typeParameters/callAndConstructSignatureLong.ts
```diff
 interface Interface {
   <
     Voila,
     InViewHumbleVaudevillianVeteran,
-    CastVicariouslyAsBothVictimAndVillainByTheVicissitudesOfFate
+    CastVicariouslyAsBothVictimAndVillainByTheVicissitudesOfFate,
   >(): V;
   new <
     ThisVisage,
     NoMereVeneerOfVanity,
     IsAVestigeOfTheVoxPopuliNowVacant,
-    Vanished
+    Vanished,
   >(): V;
 }
```
# typescript/custom/typeParameters/functionTypeLong.ts
```diff
 type AwkwardlyLongFunctionTypeDefinition = <
   GenericTypeNumberOne,
   GenericTypeNumberTwo,
-  GenericTypeNumberThree
+  GenericTypeNumberThree,
 >(
   arg1: GenericTypeNumberOne,
   arg2: GenericTypeNumberTwo,
-  arg3: GenericTypeNumberThree
-) => GenericTypeNumberOne | GenericTypeNumberTwo | GenericTypeNumberThree;
+  arg3: GenericTypeNumberThree,
+) => (GenericTypeNumberOne | GenericTypeNumberTwo | GenericTypeNumberThree);
```
# typescript/custom/typeParameters/interfaceParamsLong.ts
```diff
 interface ReallyReallyLongName<
   TypeArgumentNumberOne,
   TypeArgumentNumberTwo,
-  TypeArgumentNumberThree
+  TypeArgumentNumberThree,
 > {}
```
# typescript/custom/typeParameters/typeParametersLong.ts
```diff
 type ReallyReallyReallyLongName<
   ReallyReallyReallyLongName1,
-  ReallyReallyReallyLongName2
+  ReallyReallyReallyLongName2,
 > = any;
```
# typescript/custom/typeParameters/variables.ts
```diff
 const foo: SomeThing<boolean> = func();
 const bar: SomeThing<boolean, boolean> = func();
-const fooo: SomeThing<{ [P in "x" | "y"]: number }> = func();
+const fooo: SomeThing<
+  {
+    [P in "x" | "y"]: number;
+  }
+> = func();
 const baar: SomeThing<K extends T ? G : S> = func();
-const fooooooooooooooo: SomeThing<boolean> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const baaaaaaaaaaaaaaaaaaaaar: SomeThing<boolean, boolean> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const baaaaaaaaaaaaaaar: SomeThing<{ [P in "x" | "y"]: number }> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const baaaaaaaaaaaaaaaar: SomeThing<K extends T ? G : S> =
-  looooooooooooooooooooooooooooooongNameFunc();
+const fooooooooooooooo: SomeThing<boolean> = looooooooooooooooooooooooooooooongNameFunc();
+const baaaaaaaaaaaaaaaaaaaaar: SomeThing<boolean, boolean> = looooooooooooooooooooooooooooooongNameFunc();
+const baaaaaaaaaaaaaaar: SomeThing<
+  {
+    [P in "x" | "y"]: number;
+  }
+> = looooooooooooooooooooooooooooooongNameFunc();
+const baaaaaaaaaaaaaaaar: SomeThing<K extends T ? G : S> = looooooooooooooooooooooooooooooongNameFunc();
 const isAnySuccessfulAttempt$: Observable<boolean> = this._quizService
   .isAnySuccessfulAttempt$()
   .pipe(
-    tap((isAnySuccessfulAttempt: boolean) => {
-      this.isAnySuccessfulAttempt = isAnySuccessfulAttempt;
-    })
+    tap(
+      (isAnySuccessfulAttempt: boolean) => {
+        this.isAnySuccessfulAttempt = isAnySuccessfulAttempt;
+      },
+    ),
   );
-const isAnySuccessfulAttempt2$: Observable<boolean> =
-  this._someMethodWithLongName();
-const fooooooooooooooo: SomeThing<boolean | string> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const fooooooooooooooo: SomeThing<boolean & string> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const fooooooooooooooo: SomeThing<keyof string> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const fooooooooooooooo: SomeThing<string[]> =
-  looooooooooooooooooooooooooooooongNameFunc();
-const fooooooooooooooo: SomeThing<string["anchor"]> =
-  looooooooooooooooooooooooooooooongNameFunc();
+const isAnySuccessfulAttempt2$: Observable<boolean> = this._someMethodWithLongName();
+const fooooooooooooooo: SomeThing<boolean | string> = looooooooooooooooooooooooooooooongNameFunc();
+const fooooooooooooooo: SomeThing<boolean & string> = looooooooooooooooooooooooooooooongNameFunc();
+const fooooooooooooooo: SomeThing<keyof string> = looooooooooooooooooooooooooooooongNameFunc();
+const fooooooooooooooo: SomeThing<string[]> = looooooooooooooooooooooooooooooongNameFunc();
+const fooooooooooooooo: SomeThing<string["anchor"]> = looooooooooooooooooooooooooooooongNameFunc();
```
# typescript/declare/declare_function_with_body.ts
```diff
 // Invalid, but recoverable
-declare function foo() {};
+declare function foo() {}
 declare function bar() {
   // comment
-};
+}
```
# typescript/decorators-ts/angular.ts
```diff
 @Component({
-  selector: "toh-hero-button",
-  template: `<button>{{ label }}</button>`,
+  selector: 'toh-hero-button',
+  template: `<button>{{label}}</button>`
 })
 export class HeroButtonComponent {
   @Output() change = new EventEmitter<any>();
   @Input() label: string;
 }
```
# typescript/decorators-ts/typeorm.ts
```diff
 @Entity()
 export class Board {
   @PrimaryGeneratedColumn()
   id: number;
 
   @Column()
   slug: string;
 
   @Column()
   name: string;
 
   @Column()
   theme: string;
 
   @Column()
   description: string;
 
-  @OneToMany((type) => Topic, (topic) => topic.board)
+  @OneToMany(type => Topic, topic => topic.board)
   topics: Topic[];
 }
```
# typescript/decorators/argument-list-preserve-line.ts
```diff
 class Foo {
   constructor(
     @inject(Bar)
     private readonly bar: IBar,
-
     @inject(MyProcessor)
     private readonly myProcessor: IMyProcessor,
-
     @inject(InjectionTypes.AnotherThing)
-    private readonly anotherThing: IAnotherThing | undefined
+    private readonly anotherThing: IAnotherThing | undefined,
   ) {}
 }
```
# typescript/decorators/decorators-comments.ts
```diff
 class Foo1 {
   @foo
   // comment
   async method() {}
 }
 
 class Foo2 {
   @foo
   // comment
   private method() {}
 }
 
 class Foo3 {
   @foo
   // comment
   *method() {}
 }
 
 class Foo4 {
   @foo
   // comment
   async *method() {}
 }
 
 class Something {
   @foo()
   // comment
   readonly property: Array<string>;
 }
 
 class Something2 {
   @foo()
-  // comment
-  abstract property: Array<string>;
+    // comment
+    abstract property: Array<string>
 }
 
 class Something3 {
   @foo()
-  // comment
-  abstract method(): Array<string>;
+    // comment
+    abstract method(): Array<string>
 }
```
# typescript/decorators/decorators.ts
```diff
 export class TestTextFileService {
   constructor(@ILifecycleService lifecycleService) {}
 }
 
 @commonEditorContribution
 export class TabCompletionController {}
 
 @Component({
-  selector: "angular-component",
+  selector: 'angular-component',
 })
 class AngularComponent {
   @Input() myInput: string;
 }
 
 class Class {
   method(
     @Decorator
-    { prop1, prop2 }: Type
+    { prop1, prop2 }: Type,
   ) {
     doSomething();
   }
 }
 
 class Class2 {
   method(
     @Decorator1
     @Decorator2
-    { prop1, prop2 }: Type
+    { prop1, prop2 }: Type,
   ) {
     doSomething();
   }
 }
 
 class Class3 {
   method(
     @Decorator
     { prop1_1, prop1_2 }: Type,
-    { prop2_1, prop2_2 }: Type
+    { prop2_1, prop2_2 }: Type,
   ) {
     doSomething();
   }
 }
 
 class Class4 {
   method(
     param1,
     @Decorator
-    { prop1, prop2 }: Type
+    { prop1, prop2 }: Type,
   ) {}
 }
 
 class Class5 {
   method(@Decorator { prop1 }: Type) {}
 }
 
 class Class6 {
   method(@Decorator({}) { prop1 }: Type) {}
-  method(@Decorator({}) { prop1 }: Type) {}
+  method(
+    @Decorator(
+      {}) { prop1 }: Type,
+  ) {}
   method(@Decorator([]) { prop1 }: Type) {}
-  method(@Decorator([]) { prop1 }: Type) {}
+  method(
+    @Decorator(
+      []) { prop1 }: Type,
+  ) {}
 }
```
# typescript/decorators/inline-decorators.ts
```diff
 @d1
 @d2(foo)
 @d3.bar
 @d4.baz()
 class Class1 {}
 
 class Class2 {
   @d1
-  @d2(foo)
-  @d3.bar
-  @d4.baz()
+    @d2(foo)
+    @d3.bar
+    @d4.baz()
   method1() {}
 
   @d1
   method2() {}
 
   @d2(foo)
   method3() {}
 
   @d3.bar
   method4() {}
 }
 
 class Class3 {
   @d1 fieldA;
   @d2(foo) fieldB;
   @d3.bar fieldC;
   @d4.baz() fieldD;
 
   constructor(
     @d1 private x: number,
     @d2(foo) private y: number,
-    @d3("foo") private z: number,
+    @d3('foo') private z: number,
     @d4({
-      x: string,
-    })
-    private a: string
+            x: string
+        }) private a: string,
   ) {}
 }
 
-@decorated
-class Foo {}
+@decorated class Foo {}
 
 class Bar {
   @decorated method() {}
 }
 
 class MyContainerComponent {
-  @ContentChildren(MyComponent)
-  components: QueryListSomeBigName<MyComponentThat>;
+  @ContentChildren(MyComponent) components: QueryListSomeBigName<
+    MyComponentThat
+  >;
 }
```
# typescript/enum/enum.ts
```diff
-enum Direction {
-  Up = 1,
-  Down,
-  Left,
-  Right,
-}
+enum Direction { Up = 1, Down, Left, Right }
 
 enum FileAccess {
   // constant members
   None,
   Read = 1 << 1,
   Write = 1 << 2,
   ReadWrite = Read | Write,
   // computed member
   G = "123".length,
 }
 
 enum Empty {}
 
-const enum Enum {
-  A = 1,
-  B = A * 2,
-}
+const enum Enum { A = 1, B = A * 2 }
```
# typescript/error-recovery/index-signature.ts
```diff
 type A = { [key: string] };
 
 type TwoParams = {
   [a: string, b: string]: string;
-};
+}
 type ThreeParams = {
   [a: string, b: string, c: string]: string;
-};
+}
 
 type TooLong = {
-  [
-    loooooooooooooooooooooooooong: string,
-    looooooooooooooooooooooooooooooooooooooong: string,
-  ]: string;
-};
-type TooLong81 = {
-  [
-    loooooooooooooooooooooooooong: string,
-    loooooooooooooooooong: string,
-  ]: string;
-};
-type TooLong80 = {
-  [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;
-};
+  [loooooooooooooooooooooooooong: string, looooooooooooooooooooooooooooooooooooooong: string]: string;
+}
+type TooLong81 = { [loooooooooooooooooooooooooong: string, loooooooooooooooooong: string]: string;
+}
+type TooLong80 = { [loooooooooooooooooooooooooong: string, looooooooooooooooong: string]: string;
+}
 
 // note lack of trailing comma in the index signature
 type TooLongSingleParam = {
-  [
-    looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string
-  ]: string;
+  [looooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong: string]: string;
 };
```
# typescript/error-recovery/jsdoc_only_types.ts
```diff
-let a: *;
+let a:
+*
 function b(x: ?) {}
-let c: ?string;
-let d: ?string;
-let e: ?(string | number);
-let f: !string;
-let g: !string;
-let h: !(string | number);
+let c:
+?string
+let d: string;
+?
+let e:
+?(string | number)
+let f:
+!string;
+let g: string;
+!;
+let h:
+!(string | number);
```
# typescript/export/comment.ts
```diff
-export function match(): string; /* the matching pattern */
+export function match(): string /* the matching pattern */ ;
 a;
```
# typescript/function-type/single-parameter.ts
```diff
-type X = (options: {
-  a: string;
-  b: AbstractCompositeThingamabobberFactoryProvider;
-}) => {};
-type Y = new (options: {
-  a: string;
-  b: AbstractCompositeThingamabobberFactoryProvider;
-}) => {};
+type X = (
+  options: { a: string; b: AbstractCompositeThingamabobberFactoryProvider },
+) => {};
+type Y = new(
+  options: { a: string; b: AbstractCompositeThingamabobberFactoryProvider },
+) => {};
```
# typescript/function-type/type-annotation.ts
```diff
-const foo = (): (() => void) => (): void => null;
+const foo = (): () => void => (): void => null;
 const bar = (): (() => void) => (): void => null;
-const baz = (): (() => void) => (): void => null;
+const baz = (): ((() => void)) => (): void => null;
```
# typescript/function/single_expand.ts
```diff
-function onDidInsertSuggestion({
-  editor,
-  triggerPosition,
-  re,
-}): Promise<void> {}
+function onDidInsertSuggestion({ editor, triggerPosition, re }): Promise<void> {}
 
 class X {
-  async onDidInsertSuggestion({
-    editor,
-    triggerPosition,
-    suggestion,
-  }): Promise<void> {}
+  async onDidInsertSuggestion({ editor, triggerPosition, suggestion }): Promise<
+    void
+  > {}
 }
```
# typescript/functional-composition/pipe-function-calls-with-comments.ts
```diff
 // input with some comments added to avoid reformatting
 
 (() => {
   pipe(
     // add a descriptive comment here
     timelines,
     everyCommitTimestamps,
     A.sort(ordDate),
-    A.head
+    A.head,
   );
 
   pipe(
     // add a descriptive comment here
     serviceEventFromMessage(msg),
     TE.chain(
       flow(
         // add a descriptive comment here
         publishServiceEvent(analytics),
-        TE.mapLeft(nackFromError)
-      )
-    )
+        TE.mapLeft(nackFromError),
+      ),
+    ),
   )()
     .then(messageResponse(logger, msg))
-    .catch((err: Error) => {
-      logger.error(
-        pipe(
-          // add a descriptive comment here
-          O.fromNullable(err.stack),
-          O.getOrElse(constant(err.message))
-        )
-      );
-      process.exit(1);
-    });
+    .catch(
+      (err: Error) => {
+        logger.error(
+          pipe(
+            // add a descriptive comment here
+            O.fromNullable(err.stack),
+            O.getOrElse(constant(err.message)),
+          ),
+        );
+        process.exit(1);
+      },
+    );
 
   pipe(
     // add a descriptive comment here
     Changelog.timestampOfFirstCommit([[commit]]),
-    O.toUndefined
+    O.toUndefined,
   );
 
   chain(
     flow(
       // add a descriptive comment here
       getUploadUrl,
       E.mapLeft(Errors.unknownError),
-      TE.fromEither
-    )
+      TE.fromEither,
+    ),
   );
 })();
```
# typescript/functional-composition/pipe-function-calls.ts
```diff
 (() => {
   pipe(
     serviceEventFromMessage(msg),
-    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError)))
+    TE.chain(flow(publishServiceEvent(analytics), TE.mapLeft(nackFromError))),
   )()
     .then(messageResponse(logger, msg))
-    .catch((err: Error) => {
-      logger.error(
-        pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message)))
-      );
-      process.exit(1);
-    });
+    .catch(
+      (err: Error) => {
+        logger.error(
+          pipe(O.fromNullable(err.stack), O.getOrElse(constant(err.message))),
+        );
+        process.exit(1);
+      },
+    );
 })();
```
# typescript/generic/arrow-return-type.ts
```diff
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Descriptor> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Descriptor
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Descriptor | undefined> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Descriptor | undefined
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
   Collections.Parts.PrintedCircuitBoardAssembly["attributes"] | undefined
 > => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Descriptor & undefined> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Descriptor & undefined
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
   Collections.Parts.PrintedCircuitBoardAssembly["attributes"] & undefined
 > => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Descriptor["attributes"]> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Descriptor["attributes"]
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
   Collections.Parts.PrintedCircuitBoardAssembly["attributessssssssssssssssssssssss"]
 > => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<keyof Descriptor> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  keyof Descriptor
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
   keyof Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy
 > => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<Descriptor[]> => {};
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
+  Descriptor[]
+> => {};
 
-export const getVehicleDescriptor = async (
-  vehicleId: string
-): Promise<
+export const getVehicleDescriptor = async (vehicleId: string): Promise<
   Collections.Parts.PrintedCircuitBoardAssemblyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyyy[]
 > => {};
```
# typescript/generic/issue-6899.ts
```diff
-const getUnusedAuthorizationHoldDocuments = async (): Promise<
-  DocumentData[]
-> => {};
+const getUnusedAuthorizationHoldDocuments = async (): Promise<DocumentData[]> => {};
 
 const firestorePersonallyIdentifiablePaths: Array<
   keyof Collections.Users.Entity
 > = [];
 
 export const SUPPORTED_VEHICLE_TYPES: Array<
   Collections.VehiclesStates.Entity["type"]
 > = Object.values(Collections.VehiclesStates.Type);
```
# typescript/generic/ungrouped-parameters.ts
```diff
-function filterTooltipWithFoo<F extends Field>(
-  oldEncoding: Encoding<F>
-): {
+function filterTooltipWithFoo<F extends Field>(oldEncoding: Encoding<F>): {
   customTooltipWithoutAggregatedField?:
     | StringFieldDefWithCondition<F>
     | StringValueDefWithCondition<F>
     | StringFieldDef<F>[];
   filteredEncoding: Encoding<F>;
 } {
   const { tooltip, ...filteredEncoding } = oldEncoding;
   if (!tooltip) {
     return { filteredEncoding };
   }
   // ...
 }
```
# typescript/import-export/type-modifier.ts
```diff
 export type { SomeThing };
 export type { A as B };
 export type { B as C } from "./a";
 export type { foo } from "bar";
 export type { foo };
 
 // this should be treated as a normal import statement
 import type from "./foo";
 
 import type { SomeThing } from "./some-module.js";
 import type { foo, bar } from "baz";
 import type { foo as bar } from "baz";
 import type * as foo from "./bar";
 import type foo from "bar";
-import type foo, { bar } from "bar";
+import type foo, { bar } from 'bar';
```
# typescript/import-type/import-type.ts
```diff
 // ref: https://github.com/Microsoft/TypeScript/pull/22592
 
 export const x: import("./foo") = { x: 0, y: 0 };
 
 export let y: import("./foo2").Bar.I = { a: "", b: 0 };
 
-export let shim: typeof import("./foo2") = {
-  Bar: Bar2,
-};
+export let shim: typeof import("./foo2") = { Bar: Bar2 };
 
 export interface Foo {
   bar: import("immutable").Map<string, int>;
 }
 
 type X = A<import("B").C<any>>;
```
# typescript/interface/abstract.ts
```diff
-abstract interface I {}
+abstract;
+interface I {}
```
# typescript/interface/comments-generic.ts
```diff
 interface ReallyReallyLongName<
-    TypeArgumentNumberOne,
-    TypeArgumentNumberTwo,
-    TypeArgumentNumberThree
-  > // 1
-  extends BaseInterface {}
+  TypeArgumentNumberOne,
+  TypeArgumentNumberTwo,
+  TypeArgumentNumberThree,
+> extends BaseInterface {} // 1
 
 interface ReallyReallyLongName2<
-    TypeArgumentNumberOne,
-    TypeArgumentNumberTwo,
-    TypeArgumentNumberThree
-  > // 1
+  TypeArgumentNumberOne,
+  TypeArgumentNumberTwo,
+  TypeArgumentNumberThree,
+>
+  // 1
   // 2
-  extends BaseInterface {}
+  extends
+    BaseInterface
+{}
 
 interface ReallyReallyLongName3<
-    TypeArgumentNumberOne,
-    TypeArgumentNumberTwo,
-    TypeArgumentNumberThree
-  > // 1
+  TypeArgumentNumberOne,
+  TypeArgumentNumberTwo,
+  TypeArgumentNumberThree,
+>
+  // 1
   // 2
-  extends BaseInterface {
-  // 3
-}
+  extends
+    BaseInterface // 3
+{}
 
 interface Foo<
-    FOOOOOOOOOOOOOOOOOOOOOOOOOO,
-    FOOOOOOOOOOOOOOOOOOOOOOOOOO,
-    FOOOOOOOOOOOOOOOOOOOOOOOOOO
-  > // comments
-  extends Foo {}
+  FOOOOOOOOOOOOOOOOOOOOOOOOOO,
+  FOOOOOOOOOOOOOOOOOOOOOOOOOO,
+  FOOOOOOOOOOOOOOOOOOOOOOOOOO,
+> extends Foo {} // comments
```
# typescript/interface/generic.ts
```diff
-interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOO>
-  extends Foo {}
+interface Foo<FOOOOOOOOOOOOOOOOOOOOOOOOOO, FOOOOOOOOOOOOOOOOOOOOOOO> extends Foo {}
 
 interface Foo<
   FOOOOOOOOOOOOOOOOOOOOOOOOOO,
   FOOOOOOOOOOOOOOOOOOOOOOOOOO,
-  FOOOOOOOOOOOOOOOOOOOOOOOOOO
+  FOOOOOOOOOOOOOOOOOOOOOOOOOO,
 > extends Foo {}
```
# typescript/interface/long-extends.ts
```diff
 export interface I extends A, B, C {
   c: string;
 }
 
 export interface ThirdVeryLongAndBoringInterfaceName
-  extends ALongAndBoringInterfaceName {
+  extends
+    ALongAndBoringInterfaceName
+{
   c: string;
 }
 
 export interface ThirdVeryLongAndBoringInterfaceName
-  extends ALongAndBoringInterfaceName,
-    AnotherLongAndBoringInterfaceName {
+  extends
+    ALongAndBoringInterfaceName,
+    AnotherLongAndBoringInterfaceName
+{
   c: string;
 }
 
 export interface ThirdVeryLongAndBoringInterfaceName
-  extends AVeryLongAndBoringInterfaceName,
-    AnotherVeryLongAndBoringInterfaceName {
+  extends
+    AVeryLongAndBoringInterfaceName,
+    AnotherVeryLongAndBoringInterfaceName
+{
   c: string;
 }
 
 export interface ThirdVeryLongAndBoringInterfaceName
-  extends A_AVeryLongAndBoringInterfaceName,
+  extends
+    A_AVeryLongAndBoringInterfaceName,
     B_AVeryLongAndBoringInterfaceName,
-    C_AVeryLongAndBoringInterfaceName {
+    C_AVeryLongAndBoringInterfaceName
+{
   c: string;
 }
```
# typescript/interface/separator.ts
```diff
 declare module "selenium-webdriver" {
   export const until: {
     ableToSwitchToFrame(frame: number | WebElement | By): Condition<boolean>;
     alertIsPresent(): Condition<Alert>;
   };
 }
 
 export interface Edge {
   cursor: {};
-  node: {
-    id: {};
-  };
+  node: { id: {} };
 }
 
 interface Test {
   one: string;
   two: any[];
 }
```
# typescript/interface2/break.ts
```diff
-export interface Environment1
-  extends GenericEnvironment<SomeType, AnotherType, YetAnotherType> {
+export interface Environment1 extends GenericEnvironment<
+  SomeType,
+  AnotherType,
+  YetAnotherType,
+> {
   m(): void;
 }
 export class Environment2 extends GenericEnvironment<
   SomeType,
   AnotherType,
   YetAnotherType,
   DifferentType1,
   DifferentType2,
   DifferentType3,
-  DifferentType4
+  DifferentType4,
 > {
-  m() {}
+  m() {};
 }
 
 // Declare Interface Break
 declare interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 declare interface ExtendsLarge
-  extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
+  extends
+    ASingleInterfaceWithAReallyReallyReallyReallyLongName
+{
   x: string;
 }
 
 declare interface ExtendsMany
-  extends Interface1,
+  extends
+    Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
-    Interface7 {
+    Interface7
+{
   x: string;
 }
 
 // Interface declaration break
 interface ExtendsOne extends ASingleInterface {
   x: string;
 }
 
 interface ExtendsLarge
-  extends ASingleInterfaceWithAReallyReallyReallyReallyLongName {
+  extends
+    ASingleInterfaceWithAReallyReallyReallyReallyLongName
+{
   x: string;
 }
 
 interface ExtendsMany
-  extends Interface1,
+  extends
+    Interface1,
     Interface2,
     Interface3,
     Interface4,
     Interface5,
     Interface6,
-    Interface7 {
+    Interface7
+{
   s: string;
 }
 
 // Generic Types
 interface ExtendsOne extends ASingleInterface<string> {
   x: string;
 }
 
 interface ExtendsLarge
-  extends ASingleInterfaceWithAReallyReallyReallyReallyLongName<string> {
+  extends
+    ASingleInterfaceWithAReallyReallyReallyReallyLongName<string>
+{
   x: string;
 }
 
 interface ExtendsMany
-  extends ASingleGenericInterface<
-    Interface1,
-    Interface2,
-    Interface3,
-    Interface4,
-    Interface5,
-    Interface6,
-    Interface7
-  > {
+  extends
+    ASingleGenericInterface<
+      Interface1,
+      Interface2,
+      Interface3,
+      Interface4,
+      Interface5,
+      Interface6,
+      Interface7
+    >
+{
   x: string;
 }
 
 interface ExtendsManyWithGenerics
-  extends InterfaceOne,
+  extends
+    InterfaceOne,
     InterfaceTwo,
     ASingleGenericInterface<
       Interface1,
       Interface2,
       Interface3,
       Interface4,
       Interface5,
       Interface6,
       Interface7
     >,
-    InterfaceThree {
+    InterfaceThree
+{
   x: string;
 }
 
 export interface ExtendsLongOneWithGenerics
-  extends Bar<
-    SomeLongTypeSomeLongTypeSomeLongTypeSomeLongType,
-    ToBreakLineToBreakLineToBreakLine
-  > {}
+  extends
+    Bar<
+      SomeLongTypeSomeLongTypeSomeLongTypeSomeLongType,
+      ToBreakLineToBreakLineToBreakLine
+    >
+{}
```
# typescript/interface2/comments-declare.ts
```diff
-declare interface a // 1
-  extends b {
-  // 2
+declare interface a
+  // 1
+  extends
+    b // 2
+{
   foo: boolean;
 }
```
# typescript/interface2/comments.ts
```diff
 interface A1 {
   // comment
   foo(): bar;
 }
 
-interface A2 // comment
-  extends Base {
+interface A2 extends Base {
+  // comment
   foo(): bar;
 }
 
-interface A3 // comment1
-  extends Base {
-  // comment2
+interface A3
+  // comment1
+  extends
+    Base // comment2
+{
   foo(): bar;
 }
 
-interface A4 // comment1
-  extends Base {
-  // comment2
-  // comment3
+interface A4
+  // comment1
+  extends
+    Base // comment2
+// comment3
+{
   foo(): bar;
 }
 
-interface A5 // comment1
-  extends Base {
-  // comment2
-  // comment3
+interface A5
+  // comment1
+  extends
+    Base // comment2
+// comment3
+{
   // comment4
   foo(): bar;
 }
 
-interface A6 // comment1
-  extends Base {
-  // comment2
-  // comment3
+interface A6
+  // comment1
+  extends
+    Base // comment2
+// comment3
+{
   // comment4
   foo(): bar;
 }
```
# typescript/intersection/intersection-parens.ts
```diff
 type A = (number | string) & boolean;
-type B = (number | string) & boolean;
-type C = (number | string) & boolean;
-type D = (number | string) & boolean;
+type B = ((number | string)) & boolean;
+type C = (((number | string))) & boolean;
+type D = ((((number | string)))) & boolean;
 
 let b1: C;
 let b2: C;
-let b3: C;
-let b4: C;
-let b5: C;
+let b3: (C);
+let b4: (C);
+let b5: ((C));
 let b6: /*1*/ C;
-let b7: /*1*/ C;
-let b8: /*1*/ C;
-let b9: /*1*/ C;
+let b7: /*1*/ (C);
+let b8: /*1*/ (C);
+let b9: ( /*1*/ C);
 let b10: /*1*/ /*2*/ C;
-let b11: /*1*/ /*2*/ C;
+let b11: /*1*/ ( /*2*/ C);
 
 let bb1: /*1*/ /*2*/ C & D;
 let bb2: /*1*/ /*2*/ C & /*3*/ D;
-let bb3: /*1*/ /*2*/ C & /*3*/ D /*5*/;
+let bb3: /*1*/ /*2*/ C & /*3*/ D /*5*/ ;
 
 type B2 = C;
-type B3 = C;
-type B4 = C;
-type B5 = C;
+type B3 = (C);
+type B4 = (C);
+type B5 = ((C));
 type B6 = /*1*/ C;
-type B7 = /*1*/ C;
-type B8 = /*1*/ C;
-type B9 = /*1*/ C;
+type B7 = /*1*/ (C);
+type B8 = /*1*/ (C);
+type B9 = ( /*1*/ C);
 type B10 = /*1*/ /*2*/ C;
-type B11 = /*1*/ /*2*/ C;
-type B12 = /*1*/ C;
+type B11 = /*1*/ ( /*2*/ C);
+type B12 = /*1*/ ((C));
 
 type Bb1 = /*1*/ /*2*/ C & D;
 type Bb2 = /*1*/ /*2*/ C & /*3*/ D;
-type Bb3 = /*1*/ /*2*/ C & /*3*/ D /*4*/;
+type Bb3 = /*1*/ /*2*/ C & /*3*/ D /*4*/ ;
 
 type D1 = /*1*/ a & b;
-type D2 = /*1*/ a & b;
-type D3 = /*1*/ a & b;
-type D4 = /*1*/ a & b;
-type D5 = /*1*/ a & b;
-type D6 /*0*/ = /*1*/ a & b;
+type D2 = /*1*/ a & (b);
+type D3 = /*1*/ a & (b);
+type D4 = /*1*/ (a & b);
+type D5 = /*1*/ (a & b);
+type D6 /*0*/ = /*1*/ (a & b);
```
# typescript/keyof/keyof.ts
```diff
 type A = keyof (T | U);
 type B = keyof (X & Y);
 type C = keyof T | U;
 type D = keyof X & Y;
 type E = (keyof T)[];
-type F = (keyof T)[];
+type F = ((keyof T))[];
 type G = (keyof T1)["foo"];
-type H = (keyof T1)["foo"];
-type I = (keyof T1)["foo"];
-type J = (keyof T1)["foo"];
+type H = ((keyof T1))["foo"];
+type I = (((keyof T1)))["foo"];
+type J = ((((keyof T1))))["foo"];
```
# typescript/keyword-types/conditional-types.ts
```diff
 export type UnwrappedResultRow<T> = {
-  [P in keyof T]: T[P] extends Req<infer a>
-    ? a
-    : T[P] extends Opt<infer b>
-    ? b
-    : // TEST
-      never;
+  [P in keyof T]: (
+    T[P] extends Req<infer a> ? (a) : (
+      T[P] extends Opt<infer b> ? (b) : (
+        // TEST
+        never
+      )
+    )
+  );
 };
```
# typescript/keyword-types/keyword-types-with-parens-comments.ts
```diff
-let foo: // comment
-any;
-let foo: // comment
-null;
-let foo: // comment
-this;
-let foo: // comment
-number;
-let foo: // comment
-void;
-let foo: // comment
-boolean;
-let foo: // comment
-bigint;
-let foo: // comment
-symbol;
-let foo: // comment
-string;
-let foo: // comment
-never;
-let foo: // comment
-object;
-let foo: // comment
-undefined;
-let foo: // comment
-unknown;
+let foo: (
+  // comment
+  any
+);
+let foo: (
+  // comment
+  null
+);
+let foo: (
+  // comment
+  this
+);
+let foo: (
+  // comment
+  number
+);
+let foo: (
+  // comment
+  void
+);
+let foo: (
+  // comment
+  boolean
+);
+let foo: (
+  // comment
+  bigint
+);
+let foo: (
+  // comment
+  symbol
+);
+let foo: (
+  // comment
+  string
+);
+let foo: (
+  // comment
+  never
+);
+let foo: (
+  // comment
+  object
+);
+let foo: (
+  // comment
+  undefined
+);
+let foo: (
+  // comment
+  unknown
+);
```
# typescript/keywords/keywords.ts
```diff
 // All of these should be an error
 
 module Y3 {
-  public module Module {
+  public
+  module Module {
     class A {
       s: string;
     }
   }
 
   // Apparently this parses :P
-  export private public protected static readonly abstract async enum X {}
+    export
+  private
+  public
+  protected
+  static
+  readonly;
+  abstract;
+  async;
+  enum X {}
 
   interface x {
-    export private static readonly [x: any]: any;
-  }
+        export private static readonly [x: any]: any;
+    }
 }
 
 module Y4 {
-  public enum Color {
-    Blue,
-    Red,
-  }
+  public
+  enum Color { Blue, Red }
 }
 
 module YY3 {
-  private module Module {
+  private
+  module Module {
     class A {
       s: string;
     }
   }
 }
 
 module YY4 {
-  private enum Color {
-    Blue,
-    Red,
-  }
+  private
+  enum Color { Blue, Red }
 }
 
 module YYY3 {
-  static module Module {
+  static
+  module Module {
     class A {
       s: string;
     }
   }
 }
 
 module YYY4 {
-  static enum Color {
-    Blue,
-    Red,
-  }
+  static
+  enum Color { Blue, Red }
 }
```
# typescript/keywords/module.ts
```diff
 module Y3 {
-  public module Module {
+  public
+  module Module {
     class A {
       s: string;
     }
   }
 
   // Apparently this parses :P
-  export private public protected static readonly abstract async enum X {}
+  export
+  private
+  public
+  protected
+  static
+  readonly;
+  abstract;
+  async;
+  enum X {}
 
   interface x {
-    export private static readonly [x: any]: any;
+      export private static readonly [x: any]: any;
   }
 }
```
# typescript/last-argument-expansion/break.ts
```diff
 export default class AddAssetHtmlPlugin {
   apply(compiler: WebpackCompilerType) {
-    compiler.plugin("compilation", (compilation: WebpackCompilationType) => {
-      compilation.plugin(
-        "html-webpack-plugin-before-html",
-        (callback: Callback<any>) => {
-          addAllAssetsToCompilation(
-            this.assets,
-            compilation,
-            htmlPluginData,
-            callback
-          );
-        }
-      );
-    });
+    compiler.plugin(
+      "compilation",
+      (compilation: WebpackCompilationType) => {
+        compilation.plugin(
+          "html-webpack-plugin-before-html",
+          (callback: Callback<any>) => {
+            addAllAssetsToCompilation(
+              this.assets,
+              compilation,
+              htmlPluginData,
+              callback,
+            );
+          },
+        );
+      },
+    );
   }
 }
```
# typescript/last-argument-expansion/edge_case.ts
```diff
 var listener = DOM.listen(
   introCard,
   "click",
   sigil,
   (event: JavelinEvent): void =>
-    BanzaiLogger.log(config, {
-      ...logData,
-      ...DataStore.get(event.getNode(sigil)),
-    })
+    BanzaiLogger.log(
+      config,
+      { ...logData, ...DataStore.get(event.getNode(sigil)) },
+    ),
 );
```
# typescript/mapped-type/intersection.ts
```diff
-type Example = {
-  [A in B]: T;
-} & {
-  [A in B]: T;
-};
+type Example =
+  & {
+    [A in B]: T;
+  }
+  & {
+    [A in B]: T;
+  };
```
# typescript/mapped-type/mapped-type.ts
```diff
 type Keys = "option1" | "option2";
-type A = { [K in Keys] };
-type B = { [K in Keys]+? };
+type A = {
+  [K in Keys];
+};
+type B = {
+  [K in Keys]+?;
+};
```
# typescript/method-chain/comment.ts
```diff
 this.firebase
   .object(`/shops/${shopLocation.shop}`)
   // keep distance info
   .first(
-    (
-      shop: ShopQueryResult,
-      index: number,
-      source: Observable<ShopQueryResult>
-    ): any => {
+    (shop: ShopQueryResult, index: number, source: Observable<ShopQueryResult>): any => {
       // add distance to result
       const s = shop;
       s.distance = shopLocation.distance;
       return s;
-    }
+    },
   );
```
# typescript/method/issue-10352-consistency.ts
```diff
 export interface Store {
-  getRecord(
-    collectionName: string,
-    documentPath: string
-  ): TaskEither<Error, Option<GenericRecord>>;
+  getRecord(collectionName: string, documentPath: string): TaskEither<
+    Error,
+    Option<GenericRecord>
+  >;
 }
 
 export default class StoreImpl extends Service implements Store {
-  getRecord(
-    collectionName: string,
-    documentPath: string
-  ): TaskEither<Error, Option<GenericRecord>> {
+  getRecord(collectionName: string, documentPath: string): TaskEither<
+    Error,
+    Option<GenericRecord>
+  > {
     // Do some stuff.
   }
 }
 
-export function loadPlugin(
-  name: string,
-  dirname: string
-): { filepath: string; value: mixed } {
+export function loadPlugin(name: string, dirname: string): {
+  filepath: string;
+  value: mixed;
+} {
   // ...
 }
```
# typescript/method/method-signature.ts
```diff
 type Foo = {
   get(key: "foo"): `
   `;
 };
-type Foo = {
-  get(key: "foo"): ``;
-};
+type Foo = { get(key: "foo"): `` };
 
-type Bar = {
-  get(key: "bar"): {
-    bar: "bar";
-  };
-};
-type Bar = {
-  get(key: "bar"): { bar: "bar" };
-};
+type Bar = { get(key: "bar"): { bar: "bar" } };
+type Bar = { get(key: "bar"): { bar: "bar" } };
```
# typescript/module/empty.ts
```diff
-declare module "autoprefixer";
+declare module "autoprefixer" ;
```
# typescript/multiparser-css/issue-6259.ts
```diff
-const yesFrame = (
-  ...args: Interpolation<ThemedStyledProps<{}, Theme>>[]
-) => css`
-  ${ChatRoot}[data-frame="yes"] & {
-    ${css({}, ...args)}
-  }
+const yesFrame = (...args: Interpolation<ThemedStyledProps<{}, Theme>>[]) =>
+  css`
+    ${ChatRoot}[data-frame="yes"] & {
+        ${css({}, ...args)}
+    }
 `;
```
# typescript/new/new-signature.ts
```diff
 interface FooConstructor {
   new (
     a: number,
     b: number,
     c: number,
     d: number,
     e: number,
     f: number,
     g: number,
-    h: number
+    h: number,
   ): Foo;
 }
 
 interface BarConstructor {
   new <A, B, C>(
     a: number,
     b: number,
     c: number,
     d: number,
     e: number,
     f: number,
     g: number,
-    h: number
+    h: number,
   ): Foo;
 }
 
 type BazConstructor = {
   new (
     a: number,
     b: number,
     c: number,
     d: number,
     e: number,
     f: number,
     g: number,
-    h: number
+    h: number,
   ): Foo;
 };
 
 interface ConstructorBigGenerics {
   // comment
   new <
     AAAAAAAAAAAAAAAAAAAAAAAA,
     AAAAAAAAAAAAAAAAAAAAAAAA,
-    AAAAAAAAAAAAAAAAAAAAAAAA
+    AAAAAAAAAAAAAAAAAAAAAAAA,
   >(
     a: number,
     b: number,
     c: number,
     d: number,
     e: number,
     f: number,
     g: number,
-    h: number
+    h: number,
   ): Foo;
 }
 
 interface ConstructorInline {
   // https://github.com/prettier/prettier/issues/2163
   (i): any;
 }
 
 interface TimerConstructor {
   // Line-splitting comment
   new (interval: number, callback: (handler: Timer) => void): Timer;
 }
```
# typescript/non-null/braces.ts
```diff
-const myFunction2 = (key: string): number =>
-  ({
-    a: 42,
-    b: 42,
-  }[key]!);
+const myFunction2 = (key: string): number => ({ a: 42, b: 42 }[key]!);
 
 const myFunction3 = (key) => ({}!.a);
 
 const f = ((a) => {
   log(a);
 })!;
 
-if (a) ({ a, ...b }.a()!.c());
+if (a) {
+  ({ a, ...b }.a())!.c();
+}
 
-(function () {}!());
+(function () {})!();
 
 class a extends ({}!) {}
```
# typescript/non-null/member-chain.ts
```diff
-const { somePropThatHasAReallyLongName, anotherPropThatHasALongName } =
-  this.props.imReallySureAboutThis!;
+const { somePropThatHasAReallyLongName, anotherPropThatHasALongName } = this.props.imReallySureAboutThis!;
 
-const { somePropThatHasAReallyLongName2, anotherPropThatHasALongName2 } =
-  this.props.imReallySureAboutThis!.anotherObject;
+const { somePropThatHasAReallyLongName2, anotherPropThatHasALongName2 } = this.props.imReallySureAboutThis!.anotherObject;
 
 this.foo.get("bar")!.doThings().more();
 
 foo!.bar().baz().what();
```
# typescript/non-null/optional-chain.ts
```diff
 a?.b!.c;
 a?.b!.c.d;
 a?.b.c!.d;
 a!.b?.c;
 a?.b!?.c;
 a?.b!.c?.c;
-(a?.b)!.c;
+(a?.b!).c;
 (a?.b)!.c;
 
 a?.().b!.c;
 a?.().b!.c.d;
 a?.().b.c!.d;
 a?.().b!?.c;
 a?.().b!.c?.c;
-(a?.().b)!.c;
+(a?.().b!).c;
 (a?.().b)!.c;
 
 (a?.b)![c?.d!];
```
# typescript/non-null/parens.ts
```diff
 (a ? b : c)![tokenKey];
 (a || b)![tokenKey];
 (void 0)!;
 
 async function f() {
   return (await foo())!;
 }
 
 function* g() {
   return (yield* foo())!;
 }
 
-const a = b()!(); // parens aren't necessary
+const a = (b()!)(); // parens aren't necessary
 const b = c!();
 
 // parens are necessary if the expression result is called as a constructor
 const c1 = new (d()!)();
 const c2 = new (d()!)();
 const c3 = new (d()!.e)();
 new (x()``.y!)();
 new (x()``!.y)();
 new (x()!``.y)();
 new (x!()``.y)();
 
 xyz.a(b!).a(b!).a(b!);
```
# typescript/prettier-ignore/mapped-types.ts
```diff
 type a = {
-    // prettier-ignore
-    [A in B]: C  |  D
-  };
+  // prettier-ignore
+  [A in B]: C | D;
+};
 
 type a = {
-    [
-      // prettier-ignore
-      A in B
-    ]: C  |  D
-  };
+  [
+  // prettier-ignore
+      A in B]: C | D;
+};
 
 type a = {
-  [A in // prettier-ignore
-  B]: C | D;
+  [A in
+  // prettier-ignore
+      B]: C | D;
 };
 
 type a = {
-  [A in B]: // prettier-ignore
-  C | D;
+  [A in B]:
+  // prettier-ignore
+      C  |  D;
 };
 
 type a = {
-    [
-      /* prettier-ignore */
-      A in B
-    ]: C  |  D
-  };
+  [
+  /* prettier-ignore */
+      A in B]: C | D;
+};
 
 type a = {
-  [A in /* prettier-ignore */
-  B]: C | D;
+  [A in
+  /* prettier-ignore */
+      B]: C | D;
 };
 
 type a = {
-  [A in B]: /* prettier-ignore */
-  C | D;
+  [A in B]:
+  /* prettier-ignore */
+      C  |  D;
 };
 
 type a = {
-    /* prettier-ignore */ [A in B]: C  |  D
-  };
+  /* prettier-ignore */ [A in B]: C | D;
+};
 
 type a = {
-    [/* prettier-ignore */ A in B ]: C  |  D
-  };
+  [A in B]: C | D; /* prettier-ignore */
+};
 
 type a = {
-  [A in /* prettier-ignore */ B]: C | D;
+  [A in B]: C | D; /* prettier-ignore */
 };
 
 type a = {
-  [A in B /* prettier-ignore */]: C | D;
+  [A in B]: C | D; /* prettier-ignore */
 };
 
 type a = {
-    /* prettier-ignore */
-    [A in B]: C  |  D
-  };
+  /* prettier-ignore */
+  [A in B]: C | D;
+};
```
# typescript/private-fields-in-in/basic.ts
```diff
 class Person {
   #name: string;
   constructor(name: string) {
     this.#name = name;
   }
 
   equals(other: unknown) {
     return (
       other &&
-      typeof other === "object" &&
-      #name in other && // <- this is new!
-      this.#name === other.#name
+        typeof other === "object" &&
+        (#name in other) && // <- this is new!
+        this.#name === other.#name
     );
   }
 }
```
# typescript/range/export-assignment.ts
```diff
 f ( );
-export = f;
+export   =   f;
 g(  )
```
# typescript/range/issue-4926.ts
```diff
 class Foo {
     /** Does this key match a given MinimalKey extending object? */
     match(keyevent) {
-      // 'in' doesn't include prototypes, so it's safe for this object.
+        // 'in' doesn't include prototypes, so it's safe for this object.
       for (let attr in this) {
-        if (this[attr] !== keyevent[attr]) return false;
+        if (this[attr] !== keyevent[attr]) {
+          return false;
+        }
       }
       return true;
     }
 }
```
# typescript/range/issue-7148.ts
```diff
 export default class Test {
-  private obj = { isTest: true };
+  private obj = { isTest: true }
 }
```
# typescript/rest-type/infer-type.ts
```diff
 type Tail<T extends any[]> = T extends [infer U, ...infer R] ? R : never;
 
 // should remove parens from this, to avoid a type issue with TypeScript 4.0:
-type Tail2<T extends any[]> = T extends [infer U, ...infer R] ? R : never;
+type Tail2<T extends any[]> = T extends [infer U, ...(infer R)] ? R : never;
 
 // but not remove parens from this:
 type Tail3<T extends any[]> = T extends [infer U, ...(infer R)[]] ? R : never;
 
 type ReduceNextElement<T extends readonly unknown[]> = T extends readonly [
   infer V,
-  ...infer R
-]
-  ? [V, R]
-  : never;
+  ...infer R,
+] ? [V, R] : never;
```
# typescript/template-literal-types/template-literal-types.ts
```diff
 let x: `foo-${infer bar}`;
 type HelloWorld = `${Hello}, ${World}`;
 type SeussFish = `${Quantity | Color} fish`;
 declare function setAlignment(
-  value: `${VerticalAlignment}-${HorizontalAlignment}`
+  value: `${VerticalAlignment}-${HorizontalAlignment}`,
 ): void;
 type PropEventSource<T> = {
   on(eventName: `${string & keyof T}Changed`, callback: () => void): void;
 };
 type PropEventSource<T> = {
   on<K extends string & keyof T>(
     eventName: `${K}Changed`,
-    callback: (newValue: T[K]) => void
+    callback: (newValue: T[K]) => void,
   ): void;
 };
```
# typescript/template-literals/as-expression.ts
```diff
 const a = `${(foo + bar) as baz}`;
 const b = `${
   (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + bar) as baz
 }`;
 const b = `${
   (foo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as baz
 }`;
 const b = `${
   (foo + bar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz
 }`;
 const b = `${
-  (veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo +
-    veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz
+  (
+    veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongFoo + veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBar
+  ) as veryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryVeryLongBaz
 }`;
```
# typescript/ternaries/indent.ts
```diff
-foo = (
-  callNode.parent?.type === AST_NODE_TYPES.ChainExpression
-    ? callNode.parent.parent
-    : callNode.parent
-).TSESTree!.BinaryExpression;
+foo =
+  (
+    callNode.parent?.type === AST_NODE_TYPES.ChainExpression ? callNode.parent.parent : callNode.parent
+  ).TSESTree!.BinaryExpression;
 
-foo = (
-  callNode.parent?.type === AST_NODE_TYPES.ChainExpression
-    ? callNode.parent.parent
-    : callNode.parent
-).TSESTree!.BinaryExpression;
+foo =
+  (
+    callNode.parent?.type === AST_NODE_TYPES.ChainExpression ? callNode.parent.parent : callNode.parent
+  ).TSESTree!.BinaryExpression;
 
-bifornCringerMoshedPerplexSawder = (
-  glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-).annularCooeedSplicesWalksWayWay
-  .annularCooeedSplicesWalksWayWay(annularCooeedSplicesWalksWayWay)!
-  .annularCooeedSplicesWalksWayWay();
+bifornCringerMoshedPerplexSawder =
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+  ).annularCooeedSplicesWalksWayWay.annularCooeedSplicesWalksWayWay(
+    annularCooeedSplicesWalksWayWay,
+  )!.annularCooeedSplicesWalksWayWay();
 
-foo = (
-  callNode.parent?.type === AST_NODE_TYPES.ChainExpression
-    ? callNode.parent.parent
-    : callNode.parent
-).TSESTree!.BinaryExpression!;
+foo =
+  (
+    callNode.parent?.type === AST_NODE_TYPES.ChainExpression ? callNode.parent.parent : callNode.parent
+  ).TSESTree!.BinaryExpression!;
 
-foo = (
-  callNode.parent?.type === AST_NODE_TYPES.ChainExpression
-    ? callNode.parent.parent
-    : callNode.parent
-).TSESTree!.BinaryExpression!;
+foo =
+  (
+    callNode.parent?.type === AST_NODE_TYPES.ChainExpression ? callNode.parent.parent : callNode.parent
+  ).TSESTree!.BinaryExpression!;
 
-bifornCringerMoshedPerplexSawder = (
-  glimseGlyphsHazardNoopsTieTie === 0 &&
-  kochabCooieGameOnOboleUnweave === Math.PI
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
-).annularCooeedSplicesWalksWayWay
-  .annularCooeedSplicesWalksWayWay(annularCooeedSplicesWalksWayWay)!
-  .annularCooeedSplicesWalksWayWay()!;
+bifornCringerMoshedPerplexSawder =
+  (
+    glimseGlyphsHazardNoopsTieTie === 0 && kochabCooieGameOnOboleUnweave === Math.PI ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
+  ).annularCooeedSplicesWalksWayWay.annularCooeedSplicesWalksWayWay(
+    annularCooeedSplicesWalksWayWay,
+  )!.annularCooeedSplicesWalksWayWay()!;
 
 bifornCringerMoshedPerplexSawder =
-  askTrovenaBeenaDependsRowans +
-  (glimseGlyphsHazardNoopsTieTie === 0
-    ? averredBathersBoxroomBuggyNurl
-    : anodyneCondosMalateOverateRetinol
+  askTrovenaBeenaDependsRowans + (
+    glimseGlyphsHazardNoopsTieTie === 0 ? averredBathersBoxroomBuggyNurl : anodyneCondosMalateOverateRetinol
   ).Foo!.foo;
 
-foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)!;
+foo =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )!;
 
-foo = (
-  coooooooooooooooooooooooooooooooooooooooooooooooooooond
-    ? baaaaaaaaaaaaaaaaaaaaar
-    : baaaaaaaaaaaaaaaaaaaaaz
-)!!!!!;
+foo =
+  (
+    coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz
+  )!!!!!;
```
# typescript/test-declarations/test_declarations.ts
```diff
-test("does something really long and complicated so I have to write a very long name for the test", <T>(done) => {
-  console.log("hello!");
-});
+test(
+  "does something really long and complicated so I have to write a very long name for the test",
+  <T>(done) => {
+    console.log("hello!");
+  },
+);
```
# typescript/tsx/react.tsx
```diff
-const MyCoolList = ({ things }) => <ul>{things.map(MyCoolThing)}</ul>;
+const MyCoolList = ({ things }) =>
+  <ul>
+        {things.map(MyCoolThing)}
+    </ul>;
 
 const MyCoolThing = ({ thingo }) => <li>{thingo}</li>;
```
# typescript/tsx/url.tsx
```diff
 const link = <a href="example.com">http://example.com</a>;
 
 const first = <div>http://example.com</div>;
 
 const second = <>http://example.com</>;
 
-const third = (
-  <div>
-    <br />
-    http://example.com
-  </div>
-);
+const third = <div><br />http://example.com</div>;
 
-const fourth = (
-  <div>
-    <span></span>http://example.com
-  </div>
-);
+const fourth = <div><span></span>http://example.com</div>;
 
 const fifth = <div>{}http://example.com</div>;
```
# typescript/tuple/no-trailing-comma-after-rest.ts
```diff
-type ValidateArgs = [
-  {
-    [key: string]: any;
-  },
-  string,
-  string,
-  ...string[]
-];
+type ValidateArgs = [{ [key: string]: any }, string, string, ...string[]];
```
# typescript/tuple/trailing-comma-for-empty-tuples.ts
```diff
-type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong =
-  [];
+type Loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong = [];
 
-type Foo =
-  Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends []
-    ? Foo3
-    : Foo4;
+type Foo = Foooooooooooooooooooooooooooooooooooooooooooooooooooooooooo extends [] ? Foo3 : Foo4;
```
# typescript/tuple/trailing-comma.ts
```diff
 export interface ShopQueryResult {
   chic: boolean;
   location: number[];
   menus: Menu[];
   openingDays: number[];
   closingDays: [
-    {
-      from: string;
-      to: string;
-    } // <== this one
+    { from: string; to: string }, // <== this one
   ];
   shop: string;
   distance: number;
 }
```
# typescript/tuple/tuple.ts
```diff
 export type SCMRawResource = [
-  number /*handle*/,
-  string /*resourceUri*/,
-  modes.Command /*command*/,
-  string[] /*icons: light, dark*/,
-  boolean /*strike through*/,
-  boolean /*faded*/
+  number, /*handle*/
+  string, /*resourceUri*/
+  modes.Command, /*command*/
+  string[], /*icons: light, dark*/
+  boolean, /*strike through*/
+  boolean, /*faded*/
 ];
```
# typescript/type-alias/issue-100857.ts
```diff
-type FieldLayoutWith<
-  T extends string,
-  S extends unknown = { width: string }
-> = {
+type FieldLayoutWith<T extends string, S extends unknown = { width: string }> = {
   type: T;
   code: string;
   size: S;
 };
 
 type FieldLayoutWith<T extends string, S extends unknown> = {
   type: T;
   code: string;
   size: S;
 };
 
 type FieldLayoutWith<S extends unknown = { width: string }> = {
   type: T;
   code: string;
   size: S;
 };
 
-type FieldLayoutWith<
-  T extends stringggggggggggg,
-  T extends stringggggggggggg
-> = {
+type FieldLayoutWith<T extends stringggggggggggg, T extends stringggggggggggg> = {
   type: T;
   code: string;
   size: S;
 };
 
-type FieldLayoutWith<
-  T extends stringggggggggggg,
-  S = stringggggggggggggggggg
-> = {
+type FieldLayoutWith<T extends stringggggggggggg, S = stringggggggggggggggggg> = {
   type: T;
   code: string;
   size: S;
 };
```
# typescript/type-alias/issue-9874.ts
```diff
-export type RequestNextDealAction =
-  BaseAction<DealsActionTypes.REQUEST_NEXT_DEAL>;
+export type RequestNextDealAction = BaseAction<
+  DealsActionTypes.REQUEST_NEXT_DEAL
+>;
```
# typescript/type-member-get-set/type-member-get-set.ts
```diff
 interface Foo {
   get foo(): string;
   set bar(v);
 }
 
-type Foo = {
-  get foo(): string;
-  set bar(v);
-};
+type Foo = { get foo(): string; set bar(v) };
 
 interface Foo {
   set bar(foo: string);
 }
```
# typescript/typeparams/class-method.ts
```diff
 // https://github.com/prettier/prettier/issues/4070
 export class Thing implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType): Provider<Opts> => {}
+    (type: ObjectType): Provider<Opts> => {},
   );
 }
 
 export class Thing2 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
     (type: ObjectType): Provider<Opts> => {
       const someVar = doSomething(type);
       if (someVar) {
         return someVar.method();
       }
       return false;
-    }
+    },
   );
 }
 
 export class Thing3 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize((type) => {
     const someVar = doSomething(type);
     if (someVar) {
       return someVar.method();
     }
     return false;
   });
 }
 
 export class Thing4 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType): Provider<Opts> => type.doSomething()
+    (type: ObjectType): Provider<Opts> => type.doSomething(),
   );
 }
 
 export class Thing5 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType): Provider<Opts> => <any>type.doSomething()
+    (type: ObjectType): Provider<Opts> => <any>type.doSomething(),
   );
 }
 
 export class Thing6 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType): Provider<Opts> => <Provider<Opts>>type.doSomething()
+    (type: ObjectType): Provider<Opts> => <Provider<Opts>>type.doSomething(),
   );
 }
 
 export class Thing7 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType) => <Provider<Opts>>type.doSomething()
+    (type: ObjectType) => <Provider<Opts>>type.doSomething(),
   );
 }
 
 export class Thing8 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
     (type: ObjectType) =>
-      <Provider<Opts>>(
-        type.doSomething(withArgs, soIt, does, not, fit).extraCall()
-      )
+      <Provider<Opts>>type.doSomething(withArgs, soIt, does, not, fit).extraCall(),
   );
 }
 
 export class Thing9 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize((type: ObjectType) =>
-    type.doSomething()
+  do: (type: Type) => Provider<Prop> = memoize(
+    (type: ObjectType) => type.doSomething(),
   );
 }
 
 export class Thing10 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
     (veryLongArgName: ObjectType): Provider<Options, MoreOptions> =>
-      veryLongArgName
+      veryLongArgName,
   );
 }
 
 export class Thing11 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(
-    (type: ObjectType): Provider => {}
+    (type: ObjectType): Provider => {},
   );
 }
 
 // regular non-arrow functions
 
 export class Thing12 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider<Opts> {
-    return type;
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider<Opts> {
+      return type;
+    },
+  );
 }
 
 export class Thing13 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider<Opts> {
-    const someVar = doSomething(type);
-    if (someVar) {
-      return someVar.method();
-    }
-    return false;
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider<Opts> {
+      const someVar = doSomething(type);
+      if (someVar) {
+        return someVar.method();
+      }
+      return false;
+    },
+  );
 }
 
 export class Thing14 implements OtherThing {
   do: (type: Type) => Provider<Prop> = memoize(function (type) {
     const someVar = doSomething(type);
     if (someVar) {
       return someVar.method();
     }
     return false;
   });
 }
 
 export class Thing15 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider<Opts> {
-    return type.doSomething();
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider<Opts> {
+      return type.doSomething();
+    },
+  );
 }
 
 export class Thing16 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider<Opts> {
-    return <any>type.doSomething();
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider<Opts> {
+      return <any>type.doSomething();
+    },
+  );
 }
 
 export class Thing17 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider<Opts> {
-    return <Provider<Opts>>type.doSomething();
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider<Opts> {
+      return <Provider<Opts>>type.doSomething();
+    },
+  );
 }
 
 export class Thing18 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (type: ObjectType) {
-    return <Provider<Opts>>type.doSomething();
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType) {
+      return <Provider<Opts>>type.doSomething();
+    },
+  );
 }
 
 export class Thing19 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (type: ObjectType) {
-    return <Provider<Opts>>(
-      type.doSomething(withArgs, soIt, does, not, fit).extraCall()
-    );
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType) {
+      return <Provider<Opts>>type.doSomething(withArgs, soIt, does, not, fit).extraCall();
+    },
+  );
 }
 
 export class Thing20 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (type: ObjectType) {
-    return type.doSomething();
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType) {
+      return type.doSomething();
+    },
+  );
 }
 
 export class Thing21 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    veryLongArgName: ObjectType
-  ): Provider<Options, MoreOptions> {
-    return veryLongArgName;
-  });
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (veryLongArgName: ObjectType): Provider<Options, MoreOptions> {
+      return veryLongArgName;
+    },
+  );
 }
 
 export class Thing22 implements OtherThing {
-  do: (type: Type) => Provider<Prop> = memoize(function (
-    type: ObjectType
-  ): Provider {});
+  do: (type: Type) => Provider<Prop> = memoize(
+    function (type: ObjectType): Provider {},
+  );
 }
 
 // case from https://github.com/prettier/prettier/issues/2581
 
 const appIDs = createSelector(
   PubXURLParams.APP_IDS,
-  (rawAppIDs): Array<AppID> => deserializeList(rawAppIDs)
+  (rawAppIDs): Array<AppID> => deserializeList(rawAppIDs),
 );
```
# typescript/typeparams/consistent/issue-9501.ts
```diff
-const name: SomeGeneric<Pick<Config, "ONE_LONG_PROP" | "ANOTHER_LONG_PROP">> =
-  null;
+const name: SomeGeneric<Pick<Config, "ONE_LONG_PROP" | "ANOTHER_LONG_PROP">> = null;
```
# typescript/typeparams/consistent/simple-types.ts
```diff
-const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<any> =
-  a;
-const foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<null> =
-  a;
-const foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<this> =
-  a;
-const foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<void> =
-  a;
-const foo5: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<true> =
-  a;
-const foo6: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<false> =
-  a;
-const foo7: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<symbol> =
-  a;
-const foo8: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<true> =
-  a;
-const foo9: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<"STRING"> =
-  a;
-const foo10: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<0> =
-  a;
-const foo11: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<0xdeed_beef> =
-  a;
-const foo12: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<0xdeed_beefn> =
-  a;
+const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  any
+> = a;
+const foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  null
+> = a;
+const foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  this
+> = a;
+const foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  void
+> = a;
+const foo5: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  true
+> = a;
+const foo6: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  false
+> = a;
+const foo7: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  symbol
+> = a;
+const foo8: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  true
+> = a;
+const foo9: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  "STRING"
+> = a;
+const foo10: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  0
+> = a;
+const foo11: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  0xDeeD_Beef
+> = a;
+const foo12: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  0xDeeD_Beefn
+> = a;
```
# typescript/typeparams/consistent/template-literal-types.ts
```diff
-const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<`Hello, ${keyof World}`> =
-  a;
+const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  `Hello, ${keyof World}`
+> = a;
```
# typescript/typeparams/consistent/typescript-only.ts
```diff
-const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<never> =
-  a;
-const foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<object> =
-  a;
-const foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<undefined> =
-  a;
-const foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<unknown> =
-  a;
+const foo1: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  never
+> = a;
+const foo2: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  object
+> = a;
+const foo3: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  undefined
+> = a;
+const foo4: Fooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooo<
+  unknown
+> = a;
```
# typescript/typeparams/long-function-arg.ts
```diff
 export const forwardS = R.curry(
   <V, T>(
     prop: string,
     reducer: ReducerFunction<V, T>,
     value: V,
-    state: { [name: string]: T }
-  ) => R.assoc(prop, reducer(value, state[prop]), state)
+    state: { [name: string]: T },
+  ) => R.assoc(prop, reducer(value, state[prop]), state),
 );
 
 export const forwardS1 = R.curry(
   <
     VVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVVV,
-    TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT
+    TTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTTT,
   >(
     prop: string,
     reducer: ReducerFunction<V, T>,
     value: V,
-    state: { [name: string]: T }
-  ) => R.assoc(prop, reducer(value, state[prop]), state)
+    state: { [name: string]: T },
+  ) => R.assoc(prop, reducer(value, state[prop]), state),
 );
```
# typescript/union/comments.ts
```diff
 type A1 = a /* 1 */ | b;
 type A2 = a | /* 1 */ b;
-type A3 = a /* 1 */ | b;
-type A4 = a | /* 1 */ b;
-type A5 = a /* 1 */ | b;
-type A6 = a | /* 1 */ b;
+type A3 = (a /* 1 */ ) | b;
+type A4 = a | ( /* 1 */ b);
+type A5 = (a) /* 1 */ | b;
+type A6 = a | /* 1 */ (b);
 
 type B1 = a /* 1 */ /* 2 */ | b;
 type B2 = a /* 1 */ | /* 2 */ b;
 type B3 = a | /* 1 */ /* 2 */ b;
```
# typescript/union/inlining.ts
```diff
 interface RelayProps {
   articles: a | null;
 }
 interface RelayProps {
-  articles: Array<{
-    __id: string;
-  } | null> | null | void;
+  articles: Array<{ __id: string } | null> | null | void;
 }
 
-type UploadState<E, EM, D> =
-  // The upload hasnt begun yet
+type UploadState<E, EM, D>
+// The upload hasnt begun yet
+=
   | { type: "Not_begun" }
   // The upload timed out
   | { type: "Timed_out" }
   // Failed somewhere on the line
   | { type: "Failed"; error: E; errorMsg: EM }
   // Uploading to aws3 and CreatePostMutation succeeded
   | { type: "Success"; data: D };
 
-type UploadState2<E, EM, D> =
-  // The upload hasnt begun yet
+type UploadState2<E, EM, D>
+// The upload hasnt begun yet
+=
   | A
   // The upload timed out
   | B
   // Failed somewhere on the line
   | C
   // Uploading to aws3 and CreatePostMutation succeeded
   | D;
 
-type window = Window & {
-  __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: Function;
-};
+type window = Window & { __REDUX_DEVTOOLS_EXTENSION_COMPOSE__: Function };
 
 type T1 = (number | string)["toString"];
-type T2 = (number | string)["toString"];
-type T3 = (number | string)["toString"];
-type T4 = (number | string)["toString"];
+type T2 = ((number | string))["toString"];
+type T3 = (((number | string)))["toString"];
+type T4 = ((((number | string))))["toString"];
 type T5 = number | ((arg: any) => void);
-type T6 = number | ((arg: any) => void);
-type T7 = number | ((arg: any) => void);
-type T8 = number | ((arg: any) => void);
+type T6 = number | (((arg: any) => void));
+type T7 = number | ((((arg: any) => void)));
+type T8 = number | (((((arg: any) => void))));
```
# typescript/union/prettier-ignore.ts
```diff
 export type a =
   // foo
-  | (foo1 & foo2)
+  | foo1 & foo2
   // bar
-  | (bar1 & bar2)
+  | bar1 & bar2
   // prettier-ignore
-  | qux1&qux2;
+  | qux1 & qux2;
 
 export type a =
   // foo
-  | (foo1 & foo2)
+  | foo1 & foo2
   // bar
-  | (bar1 & bar2)
+  | bar1 & bar2
   // prettier-ignore
-  | qux1&qux2
+  | qux1 & qux2
   // baz
-  | (baz1 & baz2);
+  | baz1 & baz2;
 
 export type a =
-  // prettier-ignore
+// prettier-ignore
   | foo1&foo2
   // bar
-  | (bar1 & bar2)
+  | bar1&bar2
   // qux
-  | (qux1 & qux2);
+  | qux1&qux2;
```
# typescript/union/union-parens.ts
```diff
-export type A =
-  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
-  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
+export type A = (
+    | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
+    | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
+);
 
-export type B =
-  | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
-  | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
+export type B = (
+    | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
+    | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb
+);
 
 export type C =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type D =
   | aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa
   | bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb;
 
 export type Multi = (string | number)[];
 
-function f(): string | number {}
+function f(): (string | number) {}
 
-var x: string | number;
-var y: string | number;
+var x: (string | number);
+var y: ((string | number));
 
-class Foo<T extends string | number> {}
+class Foo<T extends (string | number)> {}
 
 interface Interface {
   i: (X | Y) & Z;
-  j: Partial<X | Y>;
+  j: Partial<(X | Y)>;
 }
 
-type State = {
-  sharedProperty: any;
-} & (
-  | { discriminant: "FOO"; foo: any }
-  | { discriminant: "BAR"; bar: any }
-  | { discriminant: "BAZ"; baz: any }
-);
+type State =
+  & { sharedProperty: any }
+  & (
+      | { discriminant: "FOO"; foo: any }
+      | { discriminant: "BAR"; bar: any }
+      | { discriminant: "BAZ"; baz: any }
+  );
 
 const foo1 = [abc, def, ghi, jkl, mno, pqr, stu, vwx, yz] as (
-  | string
-  | undefined
+  string | undefined
 )[];
 
 const foo2: (
-  | AAAAAAAAAAAAAAAAAAAAAA
-  | BBBBBBBBBBBBBBBBBBBBBB
-  | CCCCCCCCCCCCCCCCCCCCCC
-  | DDDDDDDDDDDDDDDDDDDDDD
+    | AAAAAAAAAAAAAAAAAAAAAA
+    | BBBBBBBBBBBBBBBBBBBBBB
+    | CCCCCCCCCCCCCCCCCCCCCC
+    | DDDDDDDDDDDDDDDDDDDDDD
 )[] = [];
 
 const foo3: keyof (
-  | AAAAAAAAAAAAAAAAAAAAAA
-  | BBBBBBBBBBBBBBBBBBBBBB
-  | CCCCCCCCCCCCCCCCCCCCCC
-  | DDDDDDDDDDDDDDDDDDDDDD
+    | AAAAAAAAAAAAAAAAAAAAAA
+    | BBBBBBBBBBBBBBBBBBBBBB
+    | CCCCCCCCCCCCCCCCCCCCCC
+    | DDDDDDDDDDDDDDDDDDDDDD
 ) = bar;
 
 const foo4:
   | foo
   | (
       | AAAAAAAAAAAAAAAAAAAAAA
       | BBBBBBBBBBBBBBBBBBBBBB
       | CCCCCCCCCCCCCCCCCCCCCC
       | DDDDDDDDDDDDDDDDDDDDDD
-    ) = bar;
+  ) = bar;
 
 let a1: C;
 let a2: C;
-let a3: C;
-let a4: C;
-let a5: C;
+let a3: (C);
+let a4: (C);
+let a5: ((C));
 let a6: /*1*/ C;
-let a7: /*1*/ C;
-let a8: /*1*/ C;
-let a9: /*1*/ C;
+let a7: /*1*/ (C);
+let a8: /*1*/ (C);
+let a9: ( /*1*/ C);
 let a10: /*1*/ /*2*/ C;
-let a11: /*1*/ /*2*/ C;
+let a11: /*1*/ ( /*2*/ C);
 
 let aa1: /*1*/ /*2*/ C | D;
 let aa2: /*1*/ /*2*/ C | /*3*/ D;
-let aa3: /*1*/ /*2*/ C | /*3*/ D /*4*/;
+let aa3: /*1*/ /*2*/ C | /*3*/ D; /*4*/
 
 type A1 = C;
 type A2 = C;
-type A3 = C;
-type A4 = C;
-type A5 = C;
+type A3 = (C);
+type A4 = (C);
+type A5 = ((C));
 type A6 = /*1*/ C;
-type A7 = /*1*/ C;
-type A8 = /*1*/ C;
-type A9 = /*1*/ C;
+type A7 = /*1*/ (C);
+type A8 = /*1*/ (C);
+type A9 = ( /*1*/ C);
 type A10 = /*1*/ /*2*/ C;
-type A11 = /*1*/ /*2*/ C;
-type A12 = /*1*/ C;
-type A13 = /*1*/ C;
+type A11 = /*1*/ ( /*2*/ C);
+type A12 = /*1*/ ((C));
+type A13 = /*1*/ ((C));
 
 type Aa1 = /*1*/ /*2*/ C | D;
 type Aa2 = /*1*/ /*2*/ C | /*3*/ D;
-type Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/;
+type Aa3 = /*1*/ /*2*/ C | /*3*/ D /*4*/ ;
 
 type C1 = /*1*/ a | b;
-type C2 = /*1*/ a | b;
-type C3 = /*1*/ a | b;
-type C4 = /*1*/ a | b;
-type C5 = /*1*/ a | b;
-type C6 /*0*/ = /*1*/ a | b;
+type C2 = /*1*/ a | (b);
+type C3 = /*1*/ a | (b);
+type C4 = /*1*/ (a | b);
+type C5 = /*1*/ (a | b);
+type C6 /*0*/ = /*1*/ (a | b);
 
-type Ctor = (new () => X) | Y;
+type Ctor = (new() => X) | Y;
```
# typescript/union/with-type-params.ts
```diff
 type GetChatsSagaEffects =
   | CallEffect
   | PutEffect<
       | GetUsersRequestedAction
       | GetChatsSucceededAction
       | GetChatsFailedAction
       | GetChatsStartedAction
-    >
+  >
   | SelectEffect;
```
# typescript/union/within-tuple.ts
```diff
 type A = [
-  | AAAAAAAAAAAAAAAAAAAAAA
-  | BBBBBBBBBBBBBBBBBBBBBB
-  | CCCCCCCCCCCCCCCCCCCCCC
-  | DDDDDDDDDDDDDDDDDDDDDD
+    | AAAAAAAAAAAAAAAAAAAAAA
+    | BBBBBBBBBBBBBBBBBBBBBB
+    | CCCCCCCCCCCCCCCCCCCCCC
+    | DDDDDDDDDDDDDDDDDDDDDD,
 ];
 
 type B = [
-  | AAAAAAAAAAAAAAAAAAAAAA
-  | BBBBBBBBBBBBBBBBBBBBBB
-  | CCCCCCCCCCCCCCCCCCCCCC
-  | DDDDDDDDDDDDDDDDDDDDDD
+    | AAAAAAAAAAAAAAAAAAAAAA
+    | BBBBBBBBBBBBBBBBBBBBBB
+    | CCCCCCCCCCCCCCCCCCCCCC
+    | DDDDDDDDDDDDDDDDDDDDDD,
 ];
 
 type B1 = [
-  | AAAAAAAAAAAAAAAAAAAAAA
-  | BBBBBBBBBBBBBBBBBBBBBB
-  | CCCCCCCCCCCCCCCCCCCCCC
-  | DDDDDDDDDDDDDDDDDDDDDD
+  (
+      | AAAAAAAAAAAAAAAAAAAAAA
+      | BBBBBBBBBBBBBBBBBBBBBB
+      | CCCCCCCCCCCCCCCCCCCCCC
+      | DDDDDDDDDDDDDDDDDDDDDD
+  ),
 ];
 
 type C = [
-  | [
+    | [
+        | AAAAAAAAAAAAAAAAAAAAAA
+        | BBBBBBBBBBBBBBBBBBBBBB
+        | CCCCCCCCCCCCCCCCCCCCCC
+        | DDDDDDDDDDDDDDDDDDDDDD,
+    ]
+    | [
+        | AAAAAAAAAAAAAAAAAAAAAA
+        | BBBBBBBBBBBBBBBBBBBBBB
+        | CCCCCCCCCCCCCCCCCCCCCC
+        | DDDDDDDDDDDDDDDDDDDDDD,
+    ],
+];
+
+type D = [
+  (
       | AAAAAAAAAAAAAAAAAAAAAA
       | BBBBBBBBBBBBBBBBBBBBBB
       | CCCCCCCCCCCCCCCCCCCCCC
       | DDDDDDDDDDDDDDDDDDDDDD
-    ]
-  | [
+  ),
+  (
       | AAAAAAAAAAAAAAAAAAAAAA
       | BBBBBBBBBBBBBBBBBBBBBB
       | CCCCCCCCCCCCCCCCCCCCCC
       | DDDDDDDDDDDDDDDDDDDDDD
-    ]
-];
-
-type D = [
-  (
-    | AAAAAAAAAAAAAAAAAAAAAA
-    | BBBBBBBBBBBBBBBBBBBBBB
-    | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
   ),
-  (
-    | AAAAAAAAAAAAAAAAAAAAAA
-    | BBBBBBBBBBBBBBBBBBBBBB
-    | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
-  )
 ];
 
 type D1 = [
   (
-    | AAAAAAAAAAAAAAAAAAAAAA
-    | BBBBBBBBBBBBBBBBBBBBBB
-    | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
+      | AAAAAAAAAAAAAAAAAAAAAA
+      | BBBBBBBBBBBBBBBBBBBBBB
+      | CCCCCCCCCCCCCCCCCCCCCC
+      | DDDDDDDDDDDDDDDDDDDDDD
   ),
   (
-    | AAAAAAAAAAAAAAAAAAAAAA
-    | BBBBBBBBBBBBBBBBBBBBBB
-    | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
-  )
+      | AAAAAAAAAAAAAAAAAAAAAA
+      | BBBBBBBBBBBBBBBBBBBBBB
+      | CCCCCCCCCCCCCCCCCCCCCC
+      | DDDDDDDDDDDDDDDDDDDDDD
+  ),
 ];
 
 type D2 = [
-  (
     | AAAAAAAAAAAAAAAAAAAAAA
     | BBBBBBBBBBBBBBBBBBBBBB
     | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
-  ),
-  (
+    | DDDDDDDDDDDDDDDDDDDDDD,
     | AAAAAAAAAAAAAAAAAAAAAA
     | BBBBBBBBBBBBBBBBBBBBBB
     | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
-  )
+    | DDDDDDDDDDDDDDDDDDDDDD,
 ];
 
 type E = [AA | BB, AA | BB];
 
 type F = [
-  (
     | AAAAAAAAAAAAAAAAAAAAAA
     | BBBBBBBBBBBBBBBBBBBBBB
     | CCCCCCCCCCCCCCCCCCCCCC
-    | DDDDDDDDDDDDDDDDDDDDDD
-  ),
-  AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB
+    | DDDDDDDDDDDDDDDDDDDDDD,
+  AAAAAAAAAAAAAAAAAAAAAA | BBBBBBBBBBBBBBBBBBBBBB,
 ];
```
# typescript/webhost/webtsc.ts
```diff
 /// <reference path='..\..\src\compiler\tsc.ts'/>
 
 namespace TypeScript.WebTsc {
   declare var RealActiveXObject: { new (s: string): any };
 
   function getWScriptSystem() {
     const fso = new RealActiveXObject("Scripting.FileSystemObject");
 
     const fileStream = new ActiveXObject("ADODB.Stream");
-    fileStream.Type = 2 /*text*/;
+    fileStream.Type = 2 /*text*/ ;
 
     const args: string[] = [];
     for (let i = 0; i < WScript.Arguments.length; i++) {
       args[i] = WScript.Arguments.Item(i);
     }
     return {
       args: args,
       newLine: "\r\n",
       write(s: string): void {
         WScript.StdOut.Write(s);
       },
       writeErr(s: string): void {
         WScript.StdErr.Write(s);
       },
       readFile(fileName: string, encoding?: string): string {
         if (!fso.FileExists(fileName)) {
           return undefined;
         }
         fileStream.Open();
         try {
           if (encoding) {
             fileStream.Charset = encoding;
             fileStream.LoadFromFile(fileName);
           } else {
             // Load file and read the first two bytes into a string with no interpretation
             fileStream.Charset = "x-ansi";
             fileStream.LoadFromFile(fileName);
             const bom = fileStream.ReadText(2) || "";
             // Position must be at 0 before encoding can be changed
             fileStream.Position = 0;
             // [0xFF,0xFE] and [0xFE,0xFF] mean utf-16 (little or big endian), otherwise default to utf-8
             fileStream.Charset =
-              bom.length >= 2 &&
-              ((bom.charCodeAt(0) === 0xff && bom.charCodeAt(1) === 0xfe) ||
-                (bom.charCodeAt(0) === 0xfe && bom.charCodeAt(1) === 0xff))
-                ? "unicode"
-                : "utf-8";
+              bom.length >= 2 && (
+                (bom.charCodeAt(0) === 0xFF && bom.charCodeAt(1) === 0xFE) || (
+                  bom.charCodeAt(0) === 0xFE && bom.charCodeAt(1) === 0xFF
+                )
+              ) ? "unicode" : "utf-8";
           }
           // ReadText method always strips byte order mark from resulting string
           return fileStream.ReadText();
         } catch (e) {
           throw e;
         } finally {
           fileStream.Close();
         }
       },
       writeFile(fileName: string, data: string): boolean {
         const f = fso.CreateTextFile(fileName, true);
         f.Write(data);
         f.Close();
         return true;
       },
       resolvePath(path: string): string {
         return fso.GetAbsolutePathName(path);
       },
       fileExists(path: string): boolean {
         return fso.FileExists(path);
       },
       directoryExists(path: string) {
         return fso.FolderExists(path);
       },
       createDirectory(directoryName: string) {
         if (!this.directoryExists(directoryName)) {
           fso.CreateFolder(directoryName);
         }
       },
       getExecutingFilePath() {
         return WScript.ScriptFullName;
       },
       getCurrentDirectory() {
         return "";
       },
       getMemoryUsage() {
         return 0;
       },
       exit(exitCode?: number): void {
         WScript.Quit(exitCode);
       },
       useCaseSensitiveFileNames: false,
     };
   }
 
   export function prepareCompiler(
     currentDir: string,
     stdOut: ITextWriter,
-    stdErr: ITextWriter
+    stdErr: ITextWriter,
   ) {
     const shell = new RealActiveXObject("WScript.Shell");
     shell.CurrentDirectory = currentDir;
     WScript.ScriptFullName = currentDir + "\\tc.js";
     WScript.StdOut = stdOut;
     WScript.StdErr = stdErr;
     sys = getWScriptSystem();
 
     return (commandLine: string) => {
       ts.executeCommandLine(commandLine.split(" "));
     };
   }
 }
```
