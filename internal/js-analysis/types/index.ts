/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 */

import T from "./T";
import AnyT from "./AnyT";
import BooleanLiteralT from "./BooleanLiteralT";
import BooleanT from "./BooleanT";
import CallT from "./CallT";
import ClassT from "./ClassT";
import EmptyT from "./EmptyT";
import FunctionT from "./FunctionT";
import GetPropT from "./GetPropT";
import IntersectionT from "./IntersectionT";
import MaybeT from "./MaybeT";
import MixedT from "./MixedT";
import NullT from "./NullT";
import NumericLiteralT from "./NumericLiteralT";
import NumericT from "./NumericT";
import ObjPropT from "./ObjPropT";
import ObjT from "./ObjT";
import OpenIntrinsicT from "./OpenIntrinsicT";
import ImportT from "./ImportT";
import OpenT from "./OpenT";
import StringLiteralT from "./StringLiteralT";
import StringT from "./StringT";
import UnionT from "./UnionT";
import UnknownT from "./UnknownT";
import VoidT from "./VoidT";
import ExhaustiveT from "./ExhaustiveT";
import InstanceT from "./InstanceT";
import GenericT from "./GenericT";
import ObjIndexPropT from "./ObjIndexPropT";
import BinaryOpT from "./BinaryOpT";
import RefinedT from "./RefinedT";
import RefineTypeofT from "./RefineTypeofT";
import TypeofT from "./TypeofT";
import SideEffectT from "./SideEffectT";
import BlockT from "./BlockT";
import E from "./errors/E";
import NotCallableE from "./errors/NotCallableE";
import UndeclaredVarE from "./errors/UndeclaredVarE";
import UnknownPropE from "./errors/UnknownPropE";
import UnknownImportE from "./errors/UnknownImportE";
import MissingUnionE from "./errors/MissingUnionE";
import {Class} from "@internal/typescript-helpers";
import {ExtendedMap} from "@internal/collections";

// rome-ignore lint/ts/noExplicitAny
const types: ExtendedMap<string, Class<T, Array<any>>> = new ExtendedMap(
	"types",
);
export default types;

types.set("AnyT", AnyT);
types.set("BooleanLiteralT", BooleanLiteralT);
types.set("BooleanT", BooleanT);
types.set("CallT", CallT);
types.set("ClassT", ClassT);
types.set("EmptyT", EmptyT);
types.set("FunctionT", FunctionT);
types.set("GetPropT", GetPropT);
types.set("IntersectionT", IntersectionT);
types.set("MaybeT", MaybeT);
types.set("MixedT", MixedT);
types.set("NullT", NullT);
types.set("NumericLiteralT", NumericLiteralT);
types.set("NumericT", NumericT);
types.set("ObjPropT", ObjPropT);
types.set("ObjT", ObjT);
types.set("OpenIntrinsicT", OpenIntrinsicT);
types.set("ImportT", ImportT);
types.set("OpenT", OpenT);
types.set("StringLiteralT", StringLiteralT);
types.set("StringT", StringT);
types.set("UnionT", UnionT);
types.set("UnknownT", UnknownT);
types.set("VoidT", VoidT);
types.set("ExhaustiveT", ExhaustiveT);
types.set("InstanceT", InstanceT);
types.set("GenericT", GenericT);
types.set("ObjIndexPropT", ObjIndexPropT);
types.set("BinaryOpT", BinaryOpT);
types.set("RefinedT", RefinedT);
types.set("RefineTypeofT", RefineTypeofT);
types.set("TypeofT", TypeofT);
types.set("SideEffectT", SideEffectT);
types.set("BlockT", BlockT);
types.set("E", E);
types.set("NotCallableE", NotCallableE);
types.set("UndeclaredVarE", UndeclaredVarE);
types.set("UnknownPropE", UnknownPropE);
types.set("UnknownImportE", UnknownImportE);
types.set("MissingUnionE", MissingUnionE);
