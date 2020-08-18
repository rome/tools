/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import {Scope} from "./scopes";
import T from "./types/T";
import OpenIntrinsicT from "./types/OpenIntrinsicT";
import {ExtendedMap} from "@internal/collections";

export default class Intrinsics {
	constructor(scope: Scope) {
		this.scope = scope;

		this.intrinsicByName = new ExtendedMap("intrinsicByName");

		this.NumberPrototype = this.createOpenT("NumberPrototype");
		this.Number = this.createOpenT("Number");

		this.StringPrototype = this.createOpenT("StringPrototype");
		this.String = this.createOpenT("String");

		this.ObjectPrototype = this.createOpenT("ObjectPrototype");
		this.Object = this.createOpenT("Object");

		this.ArrayPrototype = this.createOpenT("ArrayPrototype");
		this.Array = this.createOpenT("Array");

		this.RegExpPrototype = this.createOpenT("RegExpPrototype");
		this.RegExp = this.createOpenT("RegExp");
	}

	private scope: Scope;
	private intrinsicByName: ExtendedMap<string, T>;

	public String: T;
	public StringPrototype: T;

	public Object: T;
	public ObjectPrototype: T;

	public Array: T;
	public ArrayPrototype: T;

	public RegExp: T;
	public RegExpPrototype: T;

	public Number: T;
	public NumberPrototype: T;

	public get(name: string): T {
		return this.intrinsicByName.assert(name);
	}

	private createOpenT(name: string) {
		const t = new OpenIntrinsicT(this.scope, undefined, name);
		this.intrinsicByName.set(name, t);
		return t;
	}

	public link() {
		this.String.shouldMatch(this.scope.query(["String"]));
		this.StringPrototype.shouldMatch(this.scope.query(["String", "prototype"]));

		this.Object.shouldMatch(this.scope.query(["Object"]));
		this.ObjectPrototype.shouldMatch(this.scope.query(["Object", "prototype"]));

		this.Array.shouldMatch(this.scope.query(["Array"]));
		this.ArrayPrototype.shouldMatch(this.scope.query(["Array", "prototype"]));

		this.RegExp.shouldMatch(this.scope.query(["RegExp"]));
		this.RegExpPrototype.shouldMatch(this.scope.query(["RegExp", "prototype"]));

		this.Number.shouldMatch(this.scope.query(["Number"]));
		this.NumberPrototype.shouldMatch(this.scope.query(["Number", "prototype"]));
	}
}
