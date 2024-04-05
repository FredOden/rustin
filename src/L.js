//console.log(Lourah.jsFramework.root());
Activity.importScript(Lourah.jsFramework.parentDir() + "/Lourah.utils.text.Parser.js");
Activity.importScript(Lourah.jsFramework.dir() + "/L.sqrt.js");

//var ws = new WeakSet();

var lexicon = {
  Comment : {
    re : /(#.*$)/g
    }
  ,Number:Lourah.utils.text.Parser.TOKENS.Number
  , print : { re : /\b(print)\b/g }
  , if : { re : /\bif\b/g }
  , then : { re: /\bthen\b/g }
  , else : { re : /\belse\b/g }
  , for : { re : /\bfor\b/g }
  , to : { re : /\bto\b/g }
  , not: { re : /\bnot\b/g }
  , step: { re : /\bstep\b/g }
  , def: { re : /\bdef\b/g }
  , call: { re : /\bcall\b/g }
  ,Math: {
    re : /(\bcos\b)|(\bsin\b)|(\btan\b)|(\bln\b)|(\bexp\b)/g
    }
  ,OperatorB : {
    re : /(\band\b)|(\bor\b)|(\bxor\b)/g
    }
  ,OperatorL : {
    re : /(\beq\b)|(\bne\b)|(\bgt\b)|(\bge\b)|(\blt\b)|(\ble\b)/g
    }

  ,Identifier : Lourah.utils.text.Parser.TOKENS.Identifier
  ,OperatorA : {
    re : /(\+|\-)/g
    }
  ,OperatorM : {
    re : /(\*|\/|\^)/g
    }
  ,'=' : {
    re : /=/g
    }
  ,';' : {
    re : /;/g
    }
  ,'(' : { re : /(\()/g }
      ,')' : { re : /(\))/g }
  ,'[': { re : /(\[)/g }
      ,']' : { re : /(\])/g }
  ,'{': { re : /(\{)/g }
      ,'}' : { re : /(\})/g }
  ,'π': { re : /π/g }
  ,'√': { re : /\√/g }
  ,'°': { re : /°/g }
  ,'®': { re : /®/g }
  ,',': { re : /,/g }
  };

function operate(o, a, b) {

  //var a = fa();
  //var b = fb();
  //console.log("operate(" + [o, a, b] + ")");
  a = isNaN(a)?vars.get(a):a;
  b = isNaN(b)?vars.get(b):b;
  switch(o) {
    case '+': return a+b;
    case '-': return a-b;
    case '*': return a*b;
    case '/': return a/b;
    case '^': return Math.pow(a, b);
    case 'and': return a && b;
    case 'or': return a || b;
    case 'xor':return (a || b) && !(a && b);
    case 'gt': return a > b;
    case 'ge': return a >= b;
    case 'le': return a <= b;
    case 'lt': return a < b;
    case 'ne': return a != b;
    case 'eq': return a == b;
    }
  }

function callMath(math, val) {
  val = isNaN(val)?vars[val]:val;
  switch(math) {
    case "cos" : return Math.cos(val);
    case "sin" : return Math.sin(val);
    case "tan" : return Math.tan(val);
    case "ln" : return Math.log(val);
    case "exp" : return Math.exp(val);
    }
  }

function $N(a) {
  return isNaN(a)?vars[a]:Number(a);
  }

function Scope(enclosing) {
  var _vars = {};
  this.declare = (name) => {
    _vars[name] = null;
    }
  this.set = (name, value) => {
    if (_vars[name] === undefined) {
      if (enclosing) {
        enclosing.set(name, value);
        } else {
        _vars[name] = value;
        }
      } else {
      _vars[name] = value;
      }
    };
  this.get = (name) => {
    if (_vars[name] === undefined) {
      //console.log("Scope::get:not in::" + name + "::" + enclosing);
      if (enclosing) {
        return enclosing.get(name);
        } else {
        // @@@ to be discussed
        return undefined;
        }
      } else {
      return _vars[name];
      }
    };
  this.getEnclosing = () => enclosing;
  this.toString = () => {
    var s = "Scope(";
    for (k in _vars) {
        s += k + ":<" + _vars[k] + ">\n";
        }
      s += ")";
    return s;
    }
  }

var vars = new Scope();

var rules = {

  variable: {
    "Identifier":
    (p) => p.$(0)
    ,"Identifier [ &factor ]" :
    p => () => p.$(0)() + "[" + p.$(2)() + "]"
    }

  ,atom : {
    "Number":
    (p) => () => Number(p.$(0)())//{p.val = p.$(0)(); console.log("p.val::"+p.val); return p.$(0) is }
    , "π" : (p) => () => Math.PI
    , "&variable":
    (p) => () => {
      /*
      if (vars[p.$(0)()] === undefined) {
        vars[p.$(0)()] = 0;
        }
      */
      return vars.get(p.$(0)());
      }
    , "( &factor )":
    p => {
      return p.$(1);
      }
    ,"√ &atom":
    p => () => L.sqrt(p.$(1)())
    ,"&atom °":
    p => () => p.$(0)()*Math.PI/180
    ,"&atom ®":
    p => () => p.$(0)()*180/Math.PI
    ,"Math ( &factorA )":
    p => () => callMath(p.$(0)(), p.$(2)())
    , "Identifier &callParams":
    p => () => {
      
      var call = p.$(0)();
      //console.log("call::" + call);
      var expr = vars.get(call);
      var callList = p.$(1)();
      //console.log("in call::" + call);
      
      //console.log("in call::" + call + "::" + expr + "::[" + callList + "]");//::[" + expr.defList + "]");

      if (expr.defList === undefined) {
        throw "Call::error::" + call + " is not a function !";
        }
      vars = new Scope(vars);
      expr.defList.forEach((def, i) => {
          vars.declare(def);
          vars.set(def, callList[i]?callList[i]:0);
          });
      
      var r = expr();
      //console.log("did call::" + call);
      vars = vars.getEnclosing();
      return r;
      }

    /*
    ,"not &atom":
    p => () => !p.$(1)
    */
    
    , "Comment":
    p => Lourah.utils.text.Parser.NOTHING
    
    }

  
  ,factor : {
    "&factorL":
    p => p.$(0)
    ,"not &factorL":
    p => () => !p.$(1)()
    }

  ,factorL : {
    "&factorL OperatorL &factorL":
     p => () => operate(p.$(1)(), p.$(0)(), p.$(2)())
    ,"&factorB":
      p => p.$(0)
    }
  
  
  ,factorB : {
    "&factorB OperatorB &factorB":
    p => () => operate(p.$(1)(), p.$(0)(), p.$(2)())
    ,"&factorA":
    p => p.$(0)
    }

  ,factorA: {
    "&factorA OperatorA &factorA":
    p => () => operate(p.$(1)(), p.$(0)(), p.$(2)())

    ,"OperatorA &atom":
    p => () => operate(p.$(0)(), 0, p.$(1)())
     

    ,"&factorM":
    p => p.$(0)
    }

  ,factorM: {
    "&factorM OperatorM &factorM":
    p => () => operate(p.$(1)(), p.$(0)(), p.$(2)())
    ,"&atom":
    p => p.$(0)
    }

  ,expr : {
    "&variable = &expr":
    (p) => () => {
      p2 = p.$(2)();
      vars.set(p.$(0)(), p2);
      p.val = p2;
      return p2;
      }
    ,"&factor ;":
    (p) => p.$(0)
    /*
    ,"&atom ;":
    (p) => p.$(0)
    */
    ,";":
    Lourah.utils.text.Parser.NOTHING
    , "print &atom ;":
    (p) => () => { console.log(p.$(1)());return p.$(1)(); }
    , "if &factor then &expr":
    p => () => {
      if ($N(p.$(1)())) return p.$(3)();
      else return Lourah.utils.text.Parser.NOTHING;
      }
    , "if &factor then &expr else &expr":
    p => () => $N(p.$(1)())?p.$(3)():p.$(5)()
    , "&block":
    (p) => p.$(0)
    , "for &variable = &factor to &factor &expr":
    (p) => () => {
      var v = p.$(1)();
      var from = Number(p.$(3)());
      var to = Number(p.$(5)());
      if (to <  from) {
        throw "for::error::to("+to+") < from(" + from +")";
        }
      vars = new Scope(vars);
      vars.declare(v);
      for(vars.set(v, from); vars.get(v) <= to; vars.set(v, vars.get(v) + 1)) {
        p.$(6)();
        }
      vars = vars.getEnclosing();
      return p.$(6)(); //Lourah.utils.text.Parser.NOTHING;
      }
    , "for &variable = &factor to &factor step &factor &expr":
    (p) => () => {
      var v = p.$(1)();
      var step = Number(p.$(7)());
      var from = Number(p.$(3)());
      var to = Number(p.$(5)());
      if (step === 0) {
        throw "for::error::step = " + step;
        }
      vars = new Scope(vars);
      vars.declare(v);
      if ((step >0)?(to <  from):(to > from)) {
        throw "for::error::to("+to+ ((step>0)?") < from(":") > from(")
          + from +")";
        }
      for(vars.set(v, from); (step>0)?(vars.get(v) <= to):(vars.get(v) >= to); vars.set(v, vars.get(v) + step)) {
        p.$(8)();
        }
      vars = vars.getEnclosing();
      return p.$(8)();
      }
    ,"def Identifier &defParams &expr":
    p => () => {
      var def = p.$(1)();
      vars.declare(def);
      var expr = p.$(3);
      expr.defList = p.$(2)();
      //console.log("in::def::" + def + "::" + expr + "::[" + expr.defList + "]");
      vars.set(def, expr);
      return 0;
      }
    }
  
  , defParams : {
    "( )":
    p => () => []
    ,"( &defList )":
    p => () => p.$(1)()
    }
  
  , defList : {
    "Identifier":
    p => () => {
      //console.log("defList::one");
      return [ p.$(0)() ];
      }
    ,"Identifier , &defList":
    p => () => {
      //console.log("defList::more");
      return [ p.$(0)() ].concat(p.$(2)());
      }
    }
  
  , callParams : {
    "( )":
    p => () => []
    ,"( &callList )":
    p => () => p.$(1)()
    }
  
  , callList : {
    "&factor":
    p => () => [ p.$(0)() ]
    ,"&factor , &callList":
    p => () => [ p.$(0)() ].concat(p.$(2)())
    }
    
  , statement : {
    "&expr":
    (p) => p.$(0)
    , "&expr &statement":
    (p) => () => {
      p.$(0)();
      return p.$(1)();
      }
    }

  , block : {
    "{ &statement }":
    (p) => p.$(1)
    }

  }



var parser = new Lourah.utils.text.Parser(lexicon, rules, "statement");

var now = java.lang.System.currentTimeMillis;

var benchResult = "";
var start;
var stop;

//console.log = () => {};

try {
  
  var source = Activity.path2String(Lourah.jsFramework.dir() + "/parserDemo4.txt");
  start = now();
  var p = parser.compile(source);
    //source = "if x-6 then {(4*(1+2)*3*2+1)*π;} else f = for i = 3 to -1 step x-8 f = f + √2*√(√81+4^2)*3*√(π-i);a[f]=√(f+2);√a[f];;");//"for k = 5 +1 to 2*5 {x=x+k;\nif x-7 then print(x*1111); else print(x);} p[π] = 180;1+2*3 + p[π];", "statement");
  top = now() - start;
  benchResult += "compile::time::" + top;
  } catch (e) {
  console.log("compile::error::" + e + "::" + e.stack);
  }

for (var i = 0; i < 5; i++)
try {
  //vars = {};
  vars.set("x", 6);

  var r;
  start = now();
  r = p.run();
  top = now() - start;
  benchResult += "\nr::" + r +"::exécution time::" + top;
  //console.log("p.run::" + p.generated);
  } catch(e) {
  throw "parser::execution::" + e;
  }

console.log(benchResult);

//key = { nc: 0.6721*,96879 }
