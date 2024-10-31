open NodeJs
@val external importMetaUrl: string = "import.meta.url"

let args = Process.process.argv->Array.sliceToEnd(~start=2)

let fileName = args->Array.at(0)
if fileName == None {
  Js.log("Enter a nix script relative path")
  Process.process->Process.exitWithCode(1)
}
let fileName = fileName->Option.getOr("")
let filePath = Path.resolve([fileName])

let file = Fs.readFileSync(filePath)->Buffer.toString

// Js.log(file)

Tokenizer.parse(file)
