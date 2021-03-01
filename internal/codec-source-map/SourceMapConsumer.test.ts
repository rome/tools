import {test} from "rome";
import {getParsedMappingKey} from "@internal/codec-source-map/SourceMapConsumer";
import {OneIndexed, ZeroIndexed} from "@internal/numbers";
import {SourceMapConsumer} from "@internal/codec-source-map/index";
import {createRelativePath} from "@internal/path";

test(
	"Should return `line`:`column`",
	async (t) => {
		t.is(getParsedMappingKey(new OneIndexed(), new ZeroIndexed()), "1:0");
		t.is(getParsedMappingKey(new OneIndexed(5), new ZeroIndexed(76)), "5:76");
		t.is(
			getParsedMappingKey(new OneIndexed(2_780), new ZeroIndexed(4_392)),
			"2780:4392",
		);
	},
);

test(
	"Should return the position of the targeted anchor in the source file",
	async (t) => {
		/* Source
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

		/* Minified
		 *	parcelRequire=function(e,r,t,n){var i,o="function"==typeof parcelRequire&&parcelRequire,u="function"==typeof require&&require;function f(t,n){if(!r[t]){if(!e[t]){var i="function"==typeof parcelRequire&&parcelRequire;if(!n&&i)return i(t,!0);if(o)return o(t,!0);if(u&&"string"==typeof t)return u(t);var c=new Error("Cannot find module '"+t+"'");throw c.code="MODULE_NOT_FOUND",c}p.resolve=function(r){return e[t][1][r]||r},p.cache={};var l=r[t]=new f.Module(t);e[t][0].call(l.exports,p,l,l.exports,this)}return r[t].exports;function p(e){return f(p.resolve(e))}}f.isParcelRequire=!0,f.Module=function(e){this.id=e,this.bundle=f,this.exports={}},f.modules=e,f.cache=r,f.parent=o,f.register=function(r,t){e[r]=[function(e,r){r.exports=t},{}]};for(var c=0;c<t.length;c++)try{f(t[c])}catch(e){i||(i=e)}if(t.length){var l=f(t[t.length-1]);"object"==typeof exports&&"undefined"!=typeof module?module.exports=l:"function"==typeof define&&define.amd?define(function(){return l}):n&&(this[n]=l)}if(parcelRequire=f,i)throw i;return f}({"d6sW":[function(require,module,exports) {
		 *	var r="world";function n(){return"bar"}function t(){return r}
		 *	},{}]},{},["d6sW"], null)
		 *	//# sourceMappingURL=/main.7a692e5a.js.map
		 */

		const jsonSourceMap = {
			version: 3,
			sources: ["js/test.js"],
			names: ["world", "foo", "hello"],
			mappings: ";AAAA,IAAMA,EAAQ,QAEd,SAASC,IACA,MAAA,MAGT,SAASC,IACAF,OAAAA",
			file: "test.js",
			sourceRoot: "..",
			sourcesContent: [
				'const world = "world";\r\n\r\nfunction foo() {\r\n  return "bar";\r\n}\r\n\r\nfunction hello() {\r\n  return world;\r\n}\r\n',
			],
		};

		const consumer = SourceMapConsumer.fromJSON(jsonSourceMap);

		const world = {
			found: true,
			source: createRelativePath("js/test.js"),
			line: new OneIndexed(2),
			column: new ZeroIndexed(6),
			name: "world",
		};

		const foo = {
			found: true,
			source: createRelativePath("js/test.js"),
			line: new OneIndexed(4),
			column: new ZeroIndexed(9),
			name: "foo",
		};

		const hello = {
			found: true,
			source: createRelativePath("js/test.js"),
			line: new OneIndexed(8),
			column: new ZeroIndexed(9),
			name: "hello",
		};

		t.looksLike(
			consumer.approxOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(5)),
			world,
		);
		t.looksLike(
			consumer.exactOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(4)),
			world,
		);

		t.looksLike(
			consumer.approxOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(24)),
			foo,
		);
		t.looksLike(
			consumer.exactOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(23)),
			foo,
		);

		t.looksLike(
			consumer.approxOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(49)),
			hello,
		);
		t.looksLike(
			consumer.exactOriginalPositionFor(new OneIndexed(2), new ZeroIndexed(48)),
			hello,
		);
	},
);
