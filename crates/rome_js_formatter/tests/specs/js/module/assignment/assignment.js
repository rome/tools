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


// Assigning to function/classes with an empty yields the desired formatting
a = function() {}
a = class {}

// Assigning to a function/class with a non-empty body groups the whole assignment expression.
a = function () {
    console.log(1)
};

extraLongLeftHandSideLetsSeeHowPrettierBreaksThatAcrossMultipleLinesStillNotLongEnoughMustBeLongerAndLora = function () {
    console.log(1)
};

a = function extraLongLeftHandSideLetsSeeHowPrettierBreaksThatAcrossMultipleLinesStillNotLongEnoughMustBeLongerfefefefefefefefeefAndara () {
    console.log(1)
};

a = class {
    prop;
}

// Function with non-empty body is printed on the next line
A.prototype.f = function() {
    console.log(1)
}

foo = (coooooooooooooooooooooooooooooooooooooooooooooooooooond ? baaaaaaaaaaaaaaaaaaaaar : baaaaaaaaaaaaaaaaaaaaaz)!!!!!;


computedDescriptionLines = (focused && !loading && descriptionLinesFocused) || descriptionLines;

this.someObject.someOtherNestedObject = this.someOtherObject.whyNotNestAnotherOne.someLongFunctionName();
