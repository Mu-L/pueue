use pest_derive::Parser;

#[derive(Parser)]
#[grammar_inline = r#"
WHITESPACE = _{ " " }
COMMA = _{ "," }

id = { ^"id" }
status = { ^"status" }
command = { ^"command" }
label = { ^"label" }
path = { ^"path" }
enqueue_at = { ^"enqueue_at" }
dependencies = { ^"dependencies" }
start = { ^"start" }
end = { ^"end" }

column = { id | status | command | label | path | enqueue_at | dependencies | start | end }
multiple_columns = { column ~ (COMMA ~ column )* }

select = { ^"select" }
select_query = { select ~ multiple_columns }

query = { SOI ~ select_query? ~ EOI }
"#]
pub struct QueryParser;
