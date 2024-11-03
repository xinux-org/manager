module InnerHelpers = {
  let regexTransform = (
    regexp: RegExp.t,
    ~input: string,
    ~index: int=0,
    ~some: string => 'output,
    ~none: unit => 'output,
  ): 'output =>
    switch regexp->RegExp.exec(input->String.sliceToEnd(~start=index)) {
    | Some(value) => some(value->RegExp.Result.fullMatch)
    | None => none()
    }

  @val @module("node:util")
  external inspect: ('a, NodeJs.Util.inspectOptions) => string = "inspect"

  let defaultInspectOptions: NodeJs.Util.inspectOptions = {
    depth: 999,
    // showHidden: true,
    colors: true,
    // customInspect: true,
    // showProxy: true,
    // compact: true,
    // sorted: true,
    // getters: true,
  }
}

module State = {
  type rec value = List(array<value>) | String(string)
  type successValue = {input: string, index: int, result: option<value>}
  type errorValue = {input: string, index: int, result: option<value>, message: string}
  type output = {input: string, index: int, result: option<value>, message: string}

  type t =
    | Success(successValue)
    | Error(errorValue)

  let map = (state: t, transformFn: successValue => 'anything, otherwise: 'anything) => {
    switch state {
    | Success(successValue) => transformFn(successValue)
    | _ => otherwise
    }
  }

  let mapErr = (state: t, transformFn: errorValue => 'anything, otherwise: 'anything) => {
    switch state {
    | Error(errorValue) => transformFn(errorValue)
    | _ => otherwise
    }
  }

  let log = (state: t) => {
    let output: output = switch state {
    | Success({input, index, result}) => {input, index, result, message: ""}
    | Error({input, index, result, message}) => {
        input,
        index,
        result,
        message,
      }
    }

    Js.log(InnerHelpers.inspect(output, InnerHelpers.defaultInspectOptions))
  }
}

module Parser = {
  type state = State.t
  type t = {
    state: state,
    transformFn: state => state,
  }

  let new = (transformFn: state => state): t => {
    {
      state: Error({input: "", result: None, message: "Unreachable state", index: 0}),
      transformFn,
    }
  }

  let map = (parser: t, transformFn: State.successValue => state) =>
    new(state =>
      switch parser.transformFn(state) {
      | Success(successValue) => transformFn(successValue)
      | rest => rest
      }
    )

  let mapErr = (parser: t, transformFn: State.errorValue => state) =>
    new(state =>
      switch parser.transformFn(state) {
      | Error(errorValue) => transformFn(errorValue)
      | rest => rest
      }
    )

  let run = (parser: t, input: string): state =>
    parser.transformFn(
      Success({
        input,
        index: 0,
        result: None,
      }),
    )

  let str = (target: string) =>
    new(state =>
      switch state {
      | Success({input, index}) =>
        input
        ->String.slice(~start=index, ~end=index + target->String.length)
        ->String.startsWith(target)
          ? Success({
              input,
              index: index + target->String.length,
              result: Some(
                State.String(input->String.slice(~start=index, ~end=index + target->String.length)),
              ),
            })
          : Error({
              input,
              index,
              message: `Couldn't parse string ${target}`,
              result: None,
            })
      | rest => rest
      }
    )

  let sequenceOf = (targets: array<t>) =>
    new(state => state->State.map(_ => {
        let (results, state) =
          targets->Array.reduce(
            ([], state),
            ((results, state), {transformFn}) =>
              transformFn(state)->State.map(
                successValue => (
                  [...results, ...successValue.result->Option.mapOr([], v => [v])],
                  State.Success(successValue),
                ),
                (results, state),
              ),
          )

        switch state {
        | State.Success(successValue) =>
          State.Success({...successValue, result: Some(List(results))})
        | rest => rest
        }
      }, state))

  let lettersRegExp = %re("/^[a-zA-Z]+/")
  let letters = new(state => state->State.map(({input, index, result}) =>
      InnerHelpers.regexTransform(
        lettersRegExp,
        ~input,
        ~index,
        ~some=value => State.Success({
          input,
          index: index + value->String.length,
          result: Some(String(value)),
        }),
        ~none=() => State.Error({
          input,
          index,
          result: None,
          message: `Can't parse letters from input`,
        }),
      )
    , state))

  let digitsRegExp = %re("/^[0-9]+/")
  let digits = new(state => state->State.map(({input, index, result}) =>
      InnerHelpers.regexTransform(
        digitsRegExp,
        ~input,
        ~index,
        ~some=value => State.Success({
          input,
          index: index + value->String.length,
          result: Some(String(value)),
        }),
        ~none=() => State.Error({
          input,
          index,
          result: None,
          message: `Can't parse digits from input`,
        }),
      )
    , state))
}

// str = new Parser((parserState) => { ... })
// str.run(rawInput)
