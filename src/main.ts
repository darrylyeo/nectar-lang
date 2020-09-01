import { NectarInterpreter } from "./interpreter.ts"

if(Deno.args.length === 0)
	(await import('./repl.ts')).repl()

else for(const file of Deno.args){
	const interpreter = new NectarInterpreter()

	const results = interpreter.evaluate(
		new TextDecoder('utf-8').decode(Deno.readFileSync(file))
	)
	if(results.length)
		for(const result of results)
			console.log(result)
	else
		interpreter.debug()
	
	console.log()
}