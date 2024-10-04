`make gen` to generate code from backstage <br/> <br/>

`make run` to run the program <br/> <br/>
`make args="-- -y 2023 -d 5" run` to run a specific day and years problem using make command <br/> <br/>
`cargo run -- -y 2023 -d 5` to run a specific day question without prompts, you can also run with year or day <br/> <br/>

`cargo test y_2022::d_9 --lib --release -- --show-output` to run tests for a particular year and problem with outputs
`cargo test --lib --release --verbose -- --show-output` runs all the tests for all the years

<br/>

<b>Note</b>: <i>Make sure to provide day number with no leading zero when generating code from template.</i>