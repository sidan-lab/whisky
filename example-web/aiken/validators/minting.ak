use aiken/transaction.{Mint, ScriptContext, placeholder}

validator {
  fn always_succeed_minting_policy(_redeemer: Data, context: ScriptContext) {
    expect Mint(_) = context.purpose
    True
  }
}

test test_always_succeed_minting_policy() {
  let data = Void
  let context = ScriptContext { purpose: Mint(#""), transaction: placeholder() }
  always_succeed_minting_policy(data, context)
}
