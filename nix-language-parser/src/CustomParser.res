module InnerHelpers = {
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

  type successValue = {
    input: string,
    index: int,
    result: option<value>,
  }
  type errorValue = {
    input: string,
    index: int,
    result: option<value>,
    message: string,
  }
  type output = {
    input: string,
    index: int,
    result: option<value>,
    message: string,
  }

  type t =
    | Success(successValue)
    | Error(errorValue)

  let map = (state: t, default: 'output, transformFn: successValue => 'output) =>
    switch state {
    | Success(successValue) => transformFn(successValue)
    | _ => default
    }

  let mapErr = (state: t, default: 'output, transformFn: errorValue => 'output) =>
    switch state {
    | Error(errorValue) => transformFn(errorValue)
    | _ => default
    }

  let transformToErrStr = (state: t, message: string) =>
    switch state {
    | Success({input, index}) => Error({input, index, message, result: None})
    | Error(errorValue) => Error({...errorValue, message})
    }

  let mapString = (state: t, default: 'output, transformFn: string => 'output) =>
    state->map(default, ({result}) =>
      switch result {
      | Some(String(string)) => transformFn(string)
      | _ => default
      }
    )

  let mapList = (state: t, default: 'output, transformFn: array<value> => 'output) =>
    state->map(default, ({result}) =>
      switch result {
      | Some(List(results)) => transformFn(results)
      | _ => default
      }
    )

  let input = (state: t) =>
    switch state {
    | Success({input}) => input
    | Error({input}) => input
    }

  let index = (state: t) =>
    switch state {
    | Success({index}) => index
    | Error({index}) => index
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
          : state->State.transformToErrStr(`Couldn't parse string ${target}`)
      | rest => rest
      }
    )

  let sequenceOf = (targets: array<t>) =>
    new(state =>
      state->State.map(state, _ => {
        let (results, state) = targets->Array.reduce(
          ([], state),
          ((results, state), {transformFn}) =>
            switch transformFn(state) {
            | Success(successValue) => (
                [...results, ...successValue.result->Option.mapOr([], v => [v])],
                State.Success(successValue),
              )
            | Error(errorValue) => (results, State.Error(errorValue))
            },
        )

        switch state {
        | State.Success(successValue) =>
          State.Success({...successValue, result: Some(List(results))})
        | rest => rest
        }
      })
    )

  let regexp = (expression: RegExp.t, transformFn: RegExp.Result.t => option<State.value>) =>
    new(state => {
      Js.log(state)
      Js.log(state->State.input)
      Js.log("---")
      state->State.mapString(
        state->State.transformToErrStr("Expected type String as input"),
        string =>
          expression
          ->RegExp.exec(string->String.sliceToEnd(~start=state->State.index))
          ->Option.mapOr(
            state->State.transformToErrStr(
              `Failed to parse regular expression ${expression->RegExp.source}`,
            ),
            result => {
              let result = transformFn(result)

              State.Success({
                input: state->State.input,
                index: state->State.index +
                  state->State.mapString(0, string => string->String.length),
                result,
              })
            },
          ),
      )
    })

  let letter = regexp(%re("/^[a-zA-Z]/"), result => Some(String(result->RegExp.Result.fullMatch)))
  let letters = regexp(%re("/^[a-zA-Z]+/"), result => Some(String(result->RegExp.Result.fullMatch)))
  let digit = regexp(%re("/^[0-9]/"), result => Some(String(result->RegExp.Result.fullMatch)))
  let digits = regexp(%re("/^[0-9]+/"), result => Some(String(result->RegExp.Result.fullMatch)))
}

// str = new Parser((parserState) => { ... })
// str.run(rawInput)
