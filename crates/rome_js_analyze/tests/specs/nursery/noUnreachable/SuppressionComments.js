// rome-ignore lint(js/noDeadCode): this comment does nothing
function SuppressionComments1() {
    beforeReturn();
    return;
    afterReturn();
}

function SuppressionComments2() {
    beforeReturn();
    return;
    // rome-ignore lint(js/noDeadCode): supress warning
    afterReturn();
}
