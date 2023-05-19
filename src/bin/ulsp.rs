use jsonrpc_stdio_server::ServerBuilder;
use jsonrpc_stdio_server::jsonrpc_core::*;
use jsonrpc_stdio_server::jsonrpc_core::Call::MethodCall;
use jsonrpc_stdio_server::jsonrpc_core::futures::Future;
use unit_lsp::intelligence::{Language, TreeSitterFile};

fn main() {
    let mut io = IoHandler::default();

    // println!("initialize");
    io.add_method("initialize", |_params| {
        Ok(Value::String("hello".to_owned()))
    });

    ServerBuilder::new(io).build();
    // try_parse_by_language();
}

fn try_parse_by_language() {
    let src = r#"
            #include <stdio.h>ª

            #define PI 355/113
            #define AREA(r) PI * r * r

            int main() {
                int radius = 5;
                printf("%d", AREA(radius));
            }
            "#.as_bytes();

    let lang_id = "Rust";
    let language = match Language::from_id(lang_id) {
        Language::Supported(config) => config,
        _ => panic!("testing unsupported language"),
    };

    println!("{:?}", language);

    let tsf = TreeSitterFile::try_build(src, lang_id).unwrap();
    let scope_graph = tsf.scope_graph().unwrap();
    println!("{:?}", scope_graph);
    println!("{:?}", scope_graph.graph);
}