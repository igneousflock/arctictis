use crate::command::{command, macros::string_response};

string_response!(ModelInfo => ModelInfoError);

command!(b"MDL": GetModelInfo => ModelInfo);
