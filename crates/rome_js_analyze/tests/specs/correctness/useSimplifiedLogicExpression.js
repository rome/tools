// valid 
const boolExpr3 = true;
const boolExpr4 = false;
const r5 = !(boolExpr1 && boolExpr2);
const boolExpr5 = true;
const boolExpr6 = false;
const r6 = !!boolExpr1 || !!boolExpr2;
// invalid
const boolExp = true;
const r = true && boolExp;
const boolExp2 = true;
const r2 = boolExp || true;
const nonNullExp = 123;
const r3 = null ?? nonNullExp;
const boolExpr1 = true;
const boolExpr2 = false;
const r4 = !boolExpr1 || !boolExpr2;
!!x
