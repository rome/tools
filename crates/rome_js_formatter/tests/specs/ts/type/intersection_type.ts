type ShortIntersection =
    & A
    & B


type LongIntersection = A & B & C & D & E & F & G & H & I & J & K & L & M & N & O & P & Q & R & S & T & U & V & W & X & Y & Z

// cases from prettier
type Example =
	& {
		[A in B]: T;
	}
	& {
	[A in B]: T;
};

const originalPrototype = originalConstructor.prototype as TComponent & InjectionTarget;

export type AsyncExecuteOptions =
	& child_process$execFileOpts
	& {
	// The contents to write to stdin.
	stdin?: string;
	dontLogInNuclide?: boolean;
};

type State = {
	sharedProperty: any;
} & (
	| { discriminant: "FOO"; foo: any }
	| { discriminant: "BAR"; bar: any }
	| { discriminant: "BAZ"; baz: any }
	);

// spec cases
//retain comment case
type TypeWithComments = /*1*/&/*2*/ /*3*/{}/*4*/ & /*5*/number[]/*6*/ & /*7*/SomeType/*8*/;

type IndentAfterDifferentType1 = {} & SomeLongType & {somelonglonglongkey: number;} & {somelonglonglongkey: string;} & {somelonglonglongkey: SomeLongLongType} & {somelonglonglongkey: SomeLongLongType}
type IndentAfterDifferentType2 = SomeLongType1 & {} & SomeLongType2 & {somelonglonglongkey: number;} & {somelonglonglongkey: string;} & {somelonglonglongkey: SomeLongLongType} & {somelonglonglongkey: SomeLongLongType};

type NotIndent1 = {} & {somelonglonglongkey: number;} & {somelonglonglongkey: string;} & {somelonglonglongkey: SomeLongLongType} & {somelonglonglongkey: SomeLongLongType}
type NotIndent2 = SomeLongType1 & {somelonglonglongkey: number;} & {somelonglonglongkey: string;} & {somelonglonglongkey: SomeLongLongType} & {somelonglonglongkey: SomeLongLongType};

type FormatSequenceTwoObjects = {somelonglonglonglonglonglonglongkey1: number;} & {somelonglonglonglonglonglonglongkey2: number};
type FormatSequenceObjects = {somelonglonglongkey1: number; somelonglonglongkey2: string} & {somelonglonglongkey1: string; somelonglonglongkey2: number} & {somelonglonglongkey1: string; somelonglonglongkey2: number};
type FormatSequenceNotObjects = NotObjectLongLongLongLongLongLongType1 & NotObjectLongLongLongLongLongLongType2 & NotObjectLongLongLongLongLongLongType3;

type FormatObjectArray = {somelonglonglonglonglonglonglongkey1: number;} & Array<NotObjectLongLongLongLongLongLongType2 & NotObjectLongLongLongLongLongLongType3>;
type FormatArrayObject = Array<NotObjectLongLongLongLongLongLongType2 & NotObjectLongLongLongLongLongLongType3> & {somelonglonglonglonglonglonglongkey1: number;};


type SoftBreakBetweenNotObjectTypeInChain = {} & SomeLongType & {
		somelonglonglongkey: number;
	} & { somelonglonglongkey: string } & NotObjectLongLongLongLongLongLongType1 &
	NotObjectLongLongLongLongLongLongType2 & {
		somelonglonglongkey: SomeLongLongType;
	} & { somelonglonglongkey: SomeLongLongType };
