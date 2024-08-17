use std::{fs, io};
use tsr_parser::input::{new_input, Span};
use tsr_parser::lexer::ast::PredefinedType;

use tsr_parser::Parser;

use tsr_runtime::{
    api::{reflection::Reflection, util::Util},
    value::{builders::ObjectBuilder, Value},
    FunctionBuilder, Runtime,
};
use tsr_runtime::api::events::Events;

#[test]
fn main() -> io::Result<()> {
    let path = "ts/enum.ts";
    let code = fs::read_to_string(path)?;
    let  code = new_input(&code);

    println!("{:?}",code);
   /// let mut parser =Parser::new();
    let mut runtime = Runtime::default();
    runtime.set_variable(
        "print",
        Span::default().wrap(FunctionBuilder::new("log")
            .param("data", PredefinedType::Any)
            .returns(PredefinedType::Void)
            .build(|args| {
                if let Some(data) = args.get("data") {
                    //println!("{:?}",data);
                    match data {
                        Value::String(data) => println!("{data}"),
                        data => println!("{data:#}"),
                    }
                }
            })),
    );
    runtime.set_variable(
        "console",
        Span::default().wrap(
            ObjectBuilder::default()
                .prop(
                    "log",
                    FunctionBuilder::new("log")
                        .param("data", PredefinedType::Any)
                        .returns(PredefinedType::Void)
                        .build(|args| {
                            if let Some(data) = args.get("data") {
                                match data {
                                    Value::String(data) => println!("{data}"),
                                    data => println!("{data:#}"),
                                }
                            }
                        }),
                )
                .build(),
        ),
    );

    runtime.add_module(&Reflection);
    runtime.add_module(&Util);
    runtime.add_module(&Events);


    let (_, ast) = Parser::parse_ast(code.clone()).unwrap();
    //
    println!("{:#?}", ast);
    //  println!("{}", runtime.eval_program(ast).format(path, &code));
    //println!("{:#?}", runtime.get_context());

    Ok(())
}
