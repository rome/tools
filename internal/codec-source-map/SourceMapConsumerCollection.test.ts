import {TestHelper, test} from "rome";
import {ob1Coerce0, ob1Coerce1} from "@internal/ob1";
import {
	SourceMapConsumer,
	SourceMapConsumerCollection,
} from "@internal/codec-source-map/index";
import {ResolvedLocation} from "@internal/codec-source-map/types";
import { AnyPath, createUnknownPath } from "@internal/path";
import { SourceMap } from "@internal/codec-source-map";

/* Source test1
	*	const world = "world";
	*
	*	function foo() {
	*		return "bar";
	*	}
	*
	*	function hello() {
	*		return world;
	*	}
	*/

/* Minified test1
	*	parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"d6sW":[function(require,module,exports) {
	*	var r="world";function n(){return"bar"}function t(){return r}
	*	},{}]},{},["d6sW"], null)
	*	//# sourceMappingURL=/test1.js.map
	*/

const jsonSourceMap1: SourceMap = {
	version: 3,
	sources: ["js/test1.js"],
	names: ["world", "foo", "hello"],
	mappings: ";AAAA,IAAMA,EAAQ,QAEd,SAASC,IACA,MAAA,MAGT,SAASC,IACAF,OAAAA",
	file: "test.js",
	sourceRoot: "..",
	sourcesContent: [
		'const world = "world";\r\n\r\nfunction foo() {\r\n  return "bar";\r\n}\r\n\r\nfunction hello() {\r\n  return world;\r\n}\r\n',
	],
};

/* Source test2
	*	let firstName = "John";
	*  const lastname = "Doe";
	*
	*  function changeName() {
	*  	firstName = "Jane";
	*  }
	*
	*  function fullName() {
	*  	return `${firstName} ${lastname}`;
	*  }
	*/

/* Minified test2
	*  parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"prVx":[function(require,module,exports) {
	*  var n="John",c="Doe";function o(){n="Jane"}function t(){return"".concat(n," ").concat(c)}
	*  },{}]},{},["prVx"], null)
	*  //# sourceMappingURL=/test2.js.map
	*/

const jsonSourceMap2: SourceMap = {
	version: 3,
	sources: ["js/test2.js"],
	names: ["firstName", "lastname", "changeName", "fullName"],
	mappings: ";AAAA,IAAIA,EAAY,OACVC,EAAW,MAEjB,SAASC,IACPF,EAAY,OAGd,SAASG,IACGH,MAAAA,GAAAA,OAAAA,EAAaC,KAAAA,OAAAA",
	file: "test2.js",
	sourceRoot: "..",
	sourcesContent: [
		// rome-ignore lint/js/noTemplateCurlyInString: intentional string templates
		'let firstName = "John";\r\nconst lastname = "Doe";\r\n\r\nfunction changeName() {\r\n  firstName = "Jane";\r\n}\r\n\r\nfunction fullName() {\r\n  return `${firstName} ${lastname}`;\r\n}\r\n',
	],
};

test(
	"Verify hasAny, add, has are correct",
	async (t) => {
		const test1Path = createUnknownPath("test1");
		const test2Path = createUnknownPath("test2");
		const consumerCollection = new SourceMapConsumerCollection();

		t.false(consumerCollection.hasAny());

		consumerCollection.add(test1Path, SourceMapConsumer.fromJSON(jsonSourceMap1));

		t.true(consumerCollection.hasAny());
		t.true(consumerCollection.has(test1Path));
		t.false(consumerCollection.has(test2Path));

		consumerCollection.add(test2Path, SourceMapConsumer.fromJSON(jsonSourceMap2));

		t.true(consumerCollection.has(test1Path));
		t.true(consumerCollection.has(test2Path));
		t.false(consumerCollection.has(createUnknownPath("other")));
	},
);

test(
	"Should return the position of the targeted anchor in the sources files",
	async (t) => {
		const test1Path = createUnknownPath("test1");
		const test2Path = createUnknownPath("test2");

		const consumerCollection = new SourceMapConsumerCollection();
		consumerCollection.add(test1Path, SourceMapConsumer.fromJSON(jsonSourceMap1));
		consumerCollection.add(test2Path, SourceMapConsumer.fromJSON(jsonSourceMap2));

		function approxAndExact(
			t: TestHelper,
			path: AnyPath,
			line: number,
			column: number,
			expected: ResolvedLocation,
		): void {
			t.looksLike(
				consumerCollection.approxOriginalPositionFor(
					path,
					ob1Coerce1(line),
					ob1Coerce0(column + 1),
				),
				expected,
			);
			
			t.looksLike(
				consumerCollection.exactOriginalPositionFor(
					path,
					ob1Coerce1(line),
					ob1Coerce0(column),
				),
				expected,
			);
		}

		approxAndExact(t, test1Path, 2, 4, {
			found: true,
			source: createUnknownPath("js/test1.js"),
			line: ob1Coerce1(2),
			column: ob1Coerce0(6),
			name: "world",
		});

		approxAndExact(t, test1Path, 2, 23, {
			found: true,
			source: createUnknownPath("js/test1.js"),
			line: ob1Coerce1(4),
			column: ob1Coerce0(9),
			name: "foo",
		});

		approxAndExact(t, test1Path, 2, 48, {
			found: true,
			source: createUnknownPath("js/test1.js"),
			line: ob1Coerce1(8),
			column: ob1Coerce0(9),
			name: "hello",
		});

		approxAndExact(t, test2Path, 2, 4, {
			found: true,
			source: createUnknownPath("js/test2.js"),
			line: ob1Coerce1(2),
			column: ob1Coerce0(4),
			name: "firstName",
		});

		approxAndExact(t, test2Path, 2, 13, {
			found: true,
			source: createUnknownPath("js/test2.js"),
			line: ob1Coerce1(3),
			column: ob1Coerce0(6),
			name: "lastname",
		});

		approxAndExact(t, test2Path, 2, 30, {
			found: true,
			source: createUnknownPath("js/test2.js"),
			line: ob1Coerce1(5),
			column: ob1Coerce0(9),
			name: "changeName",
		});

		approxAndExact(t, test2Path, 2, 52, {
			found: true,
			source: createUnknownPath("js/test2.js"),
			line: ob1Coerce1(9),
			column: ob1Coerce0(9),
			name: "fullName",
		});
	},
);
