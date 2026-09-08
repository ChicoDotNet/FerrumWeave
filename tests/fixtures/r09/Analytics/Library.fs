namespace Analytics

module Marker =
    [<EntryPoint>]
    let main _ =
        printfn "fs-risk-score=%d" (FerrumWeave.RustApi.Answer())
        0
