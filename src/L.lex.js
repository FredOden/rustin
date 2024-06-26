
{
	tokens: {
		@Comment: ^.*(\@.*$)
		,~Identifier : ([A-Za-z_][0-9A-Za-z_]*)
		,@Number : \b((?:0x[0-9a-f]+)|(?:\d*[.]?\d+(?:(?:[E|e][\+\-]?)\d*[.]?\d+)?))\b
		,print: \b(print)\b
		,if: \bif\b
		,begin: \bbegin\b
		,end: \bend\b
		,then: \bthen\b
		,else: \belse\b
		,for: \bfor\b
		,to: \bto\b
		,eq: \beq\b
		,ne: \bne\b
		,gt: \bgt\b
		,ge: \bge\b
		,lt: \blt\b
		,le: \ble\b
		,not: \bnot\b
		,step: \bstep\b
		,def: \bdef\b
		,call: \bcall\b
		,Math: (\bcos\b)|(\bsin\b)|(\btan\b)|(\bln\b)|(\bexp\b)
		,OperatorB: (\band\b)|(\bor\b)|(\bxor\b)
		,OperatorA: (\+|\-)
		,OperatorM: (\*|\/|\^)
		,=: =
		,;: ;
		,(: (\()
			,): (\))
			,"[": (\[)
				,"]": (\])
				,"{": (\{)
					,"}": (\})
					,pi__: π
					,sqr__: √
					,deg__°: °
					,rad__: ®
					,",": ","
	}
}
