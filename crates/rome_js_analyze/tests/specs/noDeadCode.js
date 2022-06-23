// JsLabeledStatement
function JsLabeledStatement1() {
    label: while (true) {
        while (true) {
            if (true) {
                continue label;
            } else {
                break label;
            }
        }

        afterLabelJump();
    }
}

// JsTryStatement
function JsTryStatement1() {
    try {
        test();
        return;
    } catch(err) {
        test();
        return;
    }

    afterTryCatchReturn();
}

// JsTryFinallyStatement
function JsTryFinallyStatement1() {
    try {
        test();
    } catch(err) {
        test();
    } finally {
        test();
        return;
    }

    afterFinallyReturn();
}

// JsIfStatement
function JsIfStatement1() {
    if(true) {
        return;
    } else {
        return;
    }

    afterIfElseReturn();
}

// JsSwitchStatement
function JsSwitchStatement1() {
    switch(value) {
        case 0:
            case0();
            break;
        default:
            caseDefault();
            break;
        case 1: // afterDefault
            afterDefault();
    }
}

function JsSwitchStatement2() {
    switch(value) {
        case 0:
            break;
            afterBreak();
    }
}

// JsForStatement
function JsForStatement1() {
    for(let i = 0; i < 10; ++i) {
        break;
    }
}

// JsForInStatement
function JsForInStatement1() {
    for(const key in value) {
        break;
        afterBreak();
    }
}

function JsForInStatement2() {
    for(const key in value) {
        continue;
        afterContinue();
    }
}

// JsForOfStatement
function JsForOfStatement1() {
    for(const key of value) {
        break;
        afterBreak();
    }
}

function JsForOfStatement2() {
    for(const key of value) {
        continue;
        afterContinue();
    }
}

// JsWhileStatement
function JsWhileStatement1() {
    while(true) {
        break;
        afterBreak();
    }
}

// JsDoWhileStatement
function JsDoWhileStatement1() {
    do {
        break;
    } while(true); // afterBreak
}

function JsDoWhileStatement2() {
    do {
        continue;
        afterContinue();
    } while(true);
}

// JsBreakStatement
function JsBreakStatement1() {
    while(true) {
        break;
        break; // afterBreak
    }
}

function JsBreakStatement2() {
    while(true) {
        break;
        continue; // afterBreak
    }
}

// JsContinueStatement
function JsContinueStatement1() {
    while(true) {
        continue;
        continue; // afterContinue
    }
}

function JsContinueStatement2() {
    while(true) {
        continue;
        break; // afterContinue
    }
}

// JsReturnStatement
function JsReturnStatement1() {
    return;
    afterReturn();
}

function JsReturnStatement2() {
    return;
    return; // afterReturn
}

// JsThrowStatement
function JsThrowStatement1() {
    throw new Error();
    afterThrow();
}
