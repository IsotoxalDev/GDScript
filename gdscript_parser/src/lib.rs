extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;

#[derive(Parser)]
#[grammar = "gdscript.pest"]
pub struct GDScriptPestParser;

pub fn parse(what: &str) {
    let out = GDScriptPestParser::parse(Rule::file, what).unwrap();
    println!("{:#?}", out);
}
