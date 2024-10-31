type actions = Search | Continue
type tokens = Comment(string)
type state = (actions, array<tokens>)

let match_search = (state: state, program: string): (state, string) => {
  (state, program)
}

let rec match = (state: state, program: string): (state, string) => {
  if program->String.length == 0 {
    ()
  }

  let (action, tokens) = state

  switch action {
  | Search => {
      let (state, program) = match_search(state, program)
      match(state, program)
    }
  | Continue => (state, program)
  }
}

let parse = (raw: string) => {
  Js.log("action")
  let ((action, tokens), program) = match((Search, []), raw)

  Js.log2("action", action)
}
