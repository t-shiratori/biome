function f1() { }

// recursive functions are never called,
// so they are invalid
function f2() {
    f2();
}

function f3() {
    function g() {
        f3();
    }
    g();
}

// f5 is not used
const f5 = () => { };

// f6 is recursive, but never called
const f6 = () => { f6() };

// e is not used
try { } catch (e) { }
