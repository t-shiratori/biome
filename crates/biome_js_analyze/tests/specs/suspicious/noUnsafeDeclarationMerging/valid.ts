/* should not generate diagnostics */
interface Foo {}
class Bar implements Foo {}

namespace Foo2 {}
namespace Foo2 {}

enum Foo3 {}
namespace Foo3 {}

namespace Foo4 {}
function Foo4() {}

const Foo5 = class {};

interface Foo6 {}
function f6(){
  class Foo6 {};
}

class Foo7 {}
function f7(){
  interface Foo7 {}
}

interface Foo8 {}
declare class Foo8 {}

export interface Foo9 {}
export declare class Foo9 {}

declare namespace A {
  namespace B {
    export interface Foo10 {}
    export class Foo10 {}
  }
}

export declare namespace A {
  namespace B {
    export interface Foo11 {}
    export class Foo11 {}
  }
}
