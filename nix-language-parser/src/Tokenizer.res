let parse = (raw: string) => {
  open CustomParser
  Js.log("action")

  // Js.log(str("valid")->run("valid"))
  // Js.log("---")
  // Js.log(
  //   sequence([str("valid"), str("_"), str("valid")])
  //   ->mapSuccessP(value => Success({
  //     ...value,
  //     results: [
  //       value.results->Array.reduce(String(""), (acc, value) =>
  //         switch value {
  //         | String(value) => String("HAHAH")
  //         }
  //       ),
  //     ],
  //   }))
  //   ->run("valid_valid")
  //   ->mapSuccessS(({results}) => ["haha"], []),
  // )
  // Js.log(sequence([str("valid"), str("invalid")])->run("validvalid"))

  Parser.sequenceOf([
    Parser.digits,
    Parser.letters,
    Parser.digits,
    Parser.sequenceOf([
      Parser.letters,
      Parser.digits,
      Parser.letters,
      Parser.digits,
      Parser.letters,
    ]),
  ])
  ->Parser.run("1a2bcd3ef456gh")
  ->State.log
}
