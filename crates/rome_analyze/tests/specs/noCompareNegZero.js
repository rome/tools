// valid
x === 0;
0 === x;
x == 0;
0 == x;
x === "0";
"0" === x;
x == "0";
"0" == x;
x === "-0";
"-0" === x;
x == "-0";
"-0" == x;
x === -1;
-1 === x;
x < 0;
0 < x;
x <= 0;
0 <= x;
x > 0;
0 > x;
x >= 0;
0 >= x;
x != 0;
0 != x;
x !== 0;
0 !== x;
Object.is(x, -0);
x || -0;
x === +0;
// invalid
x === -0;
x == -0;
-0 == x;
x < -0;
-0 < x;
x <= -0;
-0 <= x;
x > -0;
-0 > x;
x >= -0;
-0 >= x;
x != -0;
-0 != x;
-0 !== x;
