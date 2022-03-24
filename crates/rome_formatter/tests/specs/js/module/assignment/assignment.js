a  =    b
a  +=   b
a  -=   b
a  *=   b
a  /=   b
a  %=   b
a  <<=  b
a  >>=  b
a  >>>= b
a  &=   b
a  |=    b
a  ^=   b
a  &&=  b
a  ||=  b
a  ??=  b
a  **=  b
a.b  =  c.#d
a[ b ]  =  c[ d ]
;( a )  =  b
;[a, b = "b", ...c] = d
;[fooooooooooooooooooooooooooooooooooooooooooooooooo, barrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrrr, bazzzzzzzzzzzzzzzzzzzzzzzzzz] = d
;({a,b=c,d:e,f:g=h,...j} = x)
;({aaaaaaaaaa,bbbbbbbbbb=cccccccccc,dddddddddd:eeeeeeeeee,ffffffffff:gggggggggg=hhhhhhhhhh,...jjjjjjjjjj} = x);

(s||(s=Object.create(null)))[i]=!0;
(s||(s=Object.create(null))).test=!0;
