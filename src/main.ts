import { NectarInterpreter } from "./interpreter.ts"

if(Deno.args.length === 0)
	(await import('./repl.ts')).repl()

else if(Deno.args.length === 1)
	new NectarInterpreter().evaluate(
		new TextDecoder('utf-8').decode(
			Deno.readFileSync(Deno.args[0])
		)
	)