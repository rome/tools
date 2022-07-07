//break left-hand side layout
const map: Map<Function, Map<string | void, { value: UnloadedDescriptor }>> =
    new Map();

const map: Map<Function, Condition extends Foo ? FooFooFoo : BarBarBar> =
    new Map();

const map: Map<Function, Foo<S>> = new Map();

//fluid layout

const map: Map<Map<string | void, { value: UnloadedDescriptor }>> =
    new Map();

const map: Map<Function, FunctionFunctionFunctionFunctionffFunction> =
    new Map();

const map: Map<Foo<S>> =
    new Map();

const map: Map<Condition extends Foo ? FooFooFoo : BarBarBar> =
    new Map();

const {
	id, static: isStatic, method: isMethod,
	methodId, getId, setId,
}:
	Map<Function, Map<string | void, {
		value: UnloadedDescriptor
	}>> =
	anodyneCondosMalateOverateRetinol.get(
		bifornCringerMoshedPerplexSawder
	);

// rome-ignore format: test
const {
	id, static: isStatic, method: isMethod,
	methodId, getId, setId,
}:
	// rome-ignore format: test
	Map<Function, Map<string | void, {
		value: UnloadedDescriptor
	}>> =
	// rome-ignore format: test
	anodyneCondosMalateOverateRetinol.get(
		bifornCringerMoshedPerplexSawder
	);

//break after operator layout
const loooooooooooooooooooooooooong1 = "looooooooooooooooooooooooooooooooooooooooooog"!;
const loooooooooooooooooooooooooong2 = void void "looooooooooooooooooooooooooooooooooooooooooog"!;
// rome-ignore format: test
const   loooooooooooooooooooooooooong6    =
	void    "looooooooooooooooooooooooooooooooooooooooooog"!;
const loooooooooooooooooooooooooong7    =
	// rome-ignore format: test
	!      "looooooooooooooooooooooooooooooooooooooooooog"!;

//poorly breakable member or call chain (fluid layout layout)
//JsIdentifierExpression
var looooooooooooooooooooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn<number>();
let looooooooooooooooooooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect<[number, boolean]>().ewqeqewqweqweqweqweqweqweqw;
let looooooooooooooooooooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect()[dsadsadsadsadsadsadsa]<'key'>().ewqoewqoeiowqieopwqie

//JsThisExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this<number>();
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this<[number, boolean]>().ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this()[dsadsadsadsadsadsadsa]<'key'>().ewqoewqoeiowqieopwqie

//not poorly breakable member or call chain (fluid layout layout)
//JsIdentifierExpression
var looooooooooooooooooooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn<A, B>();
let looooooooooooooooooooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect<A, B, C>().ewqeqewqweqweqweqweqweqweqw;
let looooooooooooooooooooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect()[dsadsadsadsadsadsadsa]<A, B, G>().ewqoewqoeiowqieopwqie

var looooooooooooooooooooooooooooooooooooooooooong1 = fnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfnfn<A | B>();
let looooooooooooooooooooooooooooooooooooooooooong2 = objdddddddddectobjdddddddddect<A & B>().ewqeqewqweqweqweqweqweqweqw;
let looooooooooooooooooooooooooooooooooooooooooong3 = objdddddddddectobjdddddddddect()[dsadsadsadsadsadsadsa]<{ }>().ewqoewqoeiowqieopwqie

//JsThisExpression
var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this<A, B>();
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this<A, B>().ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this()[dsadsadsadsadsadsadsa]<A, B>().ewqoewqoeiowqieopwqie

var loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong1 = this<A | B>();
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong2 = this<A & B>().ewqeqewqweqweqweqweqweqweqw;
let loooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooooong3 = this()[dsadsadsadsadsadsadsa]<{ }>().ewqoewqoeiowqieopwqie
