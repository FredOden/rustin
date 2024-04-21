{
	"rules" : {

		variable: {
			~Identifier:
			"$0"
			,"~Identifier [ &factor ]" :
			"$0[$2]"
		}

		,"atom" : {
			"@Number":
			"$0f64"
			, "pi__" : "std::f32::consts::PI"
			, "&variable":
			"$0"
			, "( &factor )":
			"$1"
			,"sqr__ &atom":
			"$1.sqrt()"
			,"&atom rad__":
			"$0*std::f32::consts::PI/180"
			,"&atom deg__":
			"$0*180/std::f32::consts::PI"
			,"Math ( &factorA )":
			"($2).$0()"
			, "~Identifier &callParams":
			"{ var call = $0;  var expr = vars.get(call); var callList = $1; if (expr.defList === undefined) { throw \"Call::error::\" + call + \" is not a function !\"; } vars = new Scope(vars); expr.defList.forEach((def, i) => { vars.declare(def); vars.set(def, callList[i]?callList[i]:0; }); var r = expr;  vars = vars.getEnclosing; return r; }"

			, "Comment":
			"Lourah.utils.text.Parser.NOTHING"

		}

		,"factor" : {
			"&factorL":
			"$0"
			,"not &factorL":
			"!$1"
		}

		,"operatorL": {
			"eq" : "=="
			,"ne" : "!="
			,"ge" : ">="
			,"gt" : ">"
			,"le" : "<="
			,"lt" : "<"
		}

		,"factorL" : {
			"&factorL &operatorL &factorL":
			"$0 $1 $2"
			,"&factorB":
			"$0"
		}


		,"factorB" : {
			"&factorB OperatorB &factorB":
			"$0 $1 $2"
			,"&factorA":
			"$0"
		}

		,"factorA": {
			"&factorA OperatorA &factorA":
			"$0 $1 $2"

			,"OperatorA &atom":
			"operate($0, 0, $1)"


			,"&factorM":
			"$0"
		}

		,"factorM": {
			"&factorM OperatorM &factorM":
			"$0 $2 $2"
			,"&atom":
			"$0"
		}

		,"expr" : {
			"&variable = &expr":
			"let mut $0 = $2;"
			,"&factor ;":
			"$0"
			,";":
			"//Nothing"
			, "print &atom ;":
			"{ let t__ = $1; println!(\"{t__}\");t__ }"
			, "if &factor then &expr":
			" if $1 { $3 }"
			, "if &factor then &expr else &expr":
			'''
			if $1 {
				$3
			}
			else {
				$5
			}
			'''
			, "&block":
			"$0"
			, "for &variable = &factor to &factor &expr":
			"{ var v = $1; var from = Number($3); var to = Number($5); if (to <  from) { throw \"for::error::to(\"+to+\") < from(\" + from +\")\";} vars = new Scope(vars); vars.declare(v); for(vars.set(v, from); vars.get(v) <= to; vars.set(v, vars.get(v) + 1) { $6; } vars = vars.getEnclosing; return $6; //Lourah.utils.text.Parser.NOTHING; }"
			, "for &variable = &factor to &factor step &factor &expr":
			"{ var v = $1; var step = Number($7); var from = Number($3); var to = Number($5); if (step === 0 { throw \"for::error::step = \" + step; } vars = new Scope(vars); vars.declare(v); if ((step >0?(to <  from):(to > from)) { throw \"for::error::to(\"+to+ ((step>0?\") < from(\":\") > from(\") + from +\")\"; } for(vars.set(v, from); (step>0?(vars.get(v) <= to):(vars.get(v) >= to); vars.set(v, vars.get(v) + step)) { $8; } vars = vars.getEnclosing; return $8; }"
			,"def ~Identifier &defParams &expr":
			"{ var def = $1; vars.declare(def); var expr = $3; expr.defList = $2; vars.set(def, expr); return 0; }"
		}

		, "defParams" : {
			"( )":
			"()"
			,"( &defList )":
			"$1"
		}

		, "defList" : {
			"~Identifier":
			"$0"
			,"~Identifier , &defList":
			"$0, $2"
		}

		, "callParams" : {
			"( )":
			"()"
			,"( &callList )":
			"$1"
		}

		, "callList" : {
			"&factor":
			"$0"
			,"&factor , &callList":
			"$0, $2"
		}

		, "statement" : {
			"&expr":
			"$0"
			, "&expr &statement":
			"{ $0;  $1 }"
			, "&expr &statement &statement":
			"{ $0; $1; $2 }"
		}

		, "block" : {
			"{ &statement }":
			"$1"
		}

		, "program" : {
			"&statement" :
			"fn main() { $0; }"
		}

	}
}

