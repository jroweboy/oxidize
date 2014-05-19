//! oxidize is a practical and extensible web framework written in rust. The main goal
#![crate_id = "oxidize#0.2-pre"]

#![comment = "Rust Web Framework"]
#![license = "MIT/ASL2"]
#![crate_type = "dylib"]
#![crate_type = "rlib"]

#![doc(html_root_url = "http://www.rust-ci.org/jroweboy/oxidize/doc/")]

// TODO make sure that this is uncommented before committing
#![deny(missing_doc)]

// used by the macros from Ogeon
#![feature(default_type_params)]

#![feature(macro_rules, macro_registrar, managed_boxes, quote, phase)]
#![macro_escape]

#[phase(syntax, link)] extern crate log;
extern crate http;
extern crate time;
extern crate collections;
extern crate sync;
extern crate syntax;


use syntax::{ast, codemap};
use syntax::ext::base::{
    SyntaxExtension, ExtCtxt, MacResult, MacExpr,
    NormalTT, BasicMacroExpander
};
use syntax::ext::build::AstBuilder;
use syntax::ext::quote::rt::ExtParseUtils;
use syntax::parse;
use syntax::parse::token;
use syntax::parse::parser;
use syntax::parse::parser::Parser;


pub use app::App;
pub use conf::Config;
pub use http::status;
pub use request::Request;
pub use middleware::MiddleWare;
pub use http::server::ResponseWriter;
pub use http::headers::content_type::MediaType;
pub use oxidize::Oxidize;

pub mod oxidize;
pub mod app;
pub mod router;
pub mod request;
pub mod conf;
pub mod middleware;

// --- HERE BE MACROS ---

/*
The routes macro provides a convenient way to define the routes and implementing 
the required traits for oxidize. It will also automatically attempt to bind any
variables passed by the url and call the method with the name provided.

```
routes!(AppName Trierouter,
    ("GET", "/", "index", self.index),
    ("GET", "/test", "test_mustache", self.test_mustache),
    ("GET", "/users/user-<userid:uint>/post-<postid:uint>", "test_variable", self.test_variable),
)
```
*/

#[macro_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
    let expander = box BasicMacroExpander{expander: expand_router, span: None};
    register(token::intern("router"), NormalTT(expander, None));

    let expander = box BasicMacroExpander{expander: expand_routes, span: None};
    register(token::intern("routes"), NormalTT(expander, None));
}

fn expand_router(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let router_ident = cx.ident_of("router");
    let insert_method = cx.ident_of("insert_item");

    let mut calls: Vec<@ast::Stmt> = vec!(
        cx.stmt_let(sp, true, router_ident, quote_expr!(&cx, ::oxidize::router::Router::new()))
    );

    for (path, method, handler) in parse_routes(cx, tts).move_iter() {
        let path_expr = cx.parse_expr(format!("\"{}\"", path).to_strbuf());
        let method_expr = cx.expr_path(method);
        let handler_expr = cx.expr_path(handler);
        calls.push(cx.stmt_expr(
            cx.expr_method_call(sp, cx.expr_ident(sp, router_ident), insert_method, vec!(method_expr, path_expr, handler_expr))
        ));
    }

    let block = cx.expr_block(cx.block(sp, calls, Some(cx.expr_ident(sp, router_ident))));
    
    MacExpr::new(block)
}

fn expand_routes(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    let routes = parse_routes(cx, tts).move_iter().map(|(path, method, handler)| {
        let path_expr = cx.parse_expr(format!("\"{}\"", path).to_strbuf());
        let method_expr = cx.expr_path(method);
        let handler_expr = cx.expr_path(handler);
        mk_tup(sp, vec!(method_expr, path_expr, handler_expr))
    }).collect();

    MacExpr::new(cx.expr_vec(sp, routes))
}

fn parse_routes(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Vec<(~str, ast::Path, ast::Path)> {

    let mut parser = parse::new_parser_from_tts(
        cx.parse_sess(), cx.cfg(), Vec::from_slice(tts)
    );

    parse_subroutes("", cx, &mut parser)
}

fn parse_subroutes(base: &str, cx: &mut ExtCtxt, parser: &mut Parser) -> Vec<(~str, ast::Path, ast::Path)> {
    let mut routes = Vec::new();

    while !parser.eat(&token::EOF) {
        match parser.parse_optional_str() {
            Some((ref s, _)) => {
                if !parser.eat(&token::FAT_ARROW) {
                    parser.expect(&token::FAT_ARROW);
                    break;
                }

                let new_base = base + s.to_str().trim_chars('/').to_owned() + "/";


                if parser.eat(&token::EOF) {
                    cx.span_err(parser.span, "unexpected end of routing tree");
                }

                if parser.eat(&token::LBRACE) {
                    let subroutes = parse_subroutes(new_base, cx, parser);
                    routes.push_all(subroutes.as_slice());

                    if parser.eat(&token::RBRACE) {
                        if !parser.eat(&token::COMMA) {
                            break;
                        }
                    } else {
                        parser.expect_one_of([token::COMMA, token::RBRACE], []);
                    }
                } else {
                    for (method, handler) in parse_handler(parser).move_iter() {
                        routes.push((new_base.clone(), method, handler))
                    }

                    if !parser.eat(&token::COMMA) {
                        break;
                    }
                }
            },
            None => {
                for (method, handler) in parse_handler(parser).move_iter() {
                    routes.push((base.to_owned(), method, handler))
                }

                if !parser.eat(&token::COMMA) {
                    break;
                }
            }
        }
    }

    routes
}

fn parse_handler(parser: &mut Parser) -> Vec<(ast::Path, ast::Path)> {
    let mut methods = Vec::new();

    loop {
        methods.push(parser.parse_path(parser::NoTypesAllowed).path);

        if parser.eat(&token::COLON) {
            break;
        }

        if !parser.eat(&token::BINOP(token::OR)) {
            parser.expect_one_of([token::COLON, token::BINOP(token::OR)], []);
        }
    }

    let handler = parser.parse_path(parser::NoTypesAllowed).path;

    methods.move_iter().map(|m| (m, handler.clone())).collect()
}

fn mk_tup(sp: codemap::Span, content: Vec<@ast::Expr>) -> @ast::Expr {
    dummy_expr(sp, ast::ExprTup(content))
}

fn dummy_expr(sp: codemap::Span, e: ast::Expr_) -> @ast::Expr {
    @ast::Expr {
        id: ast::DUMMY_NODE_ID,
        node: e,
        span: sp,
    }
}


// Ugh, it looks like I'll need to change to the more complicated macro_registar instead
// #[macro_export]
// macro_rules! routes(
//     ($app_name:ident, $router_name:ident, $(($method:expr, $path:expr, $name:expr, $func:expr)),+) => (
//     impl App for $app_name {
//         fn handle_route(&self, info: RouteInfo, req: &mut Request, res: &mut ResponseWriter){
//             match (info.name, info.method) {
//                 $(($name, $method) => {$func(req, res)}, )+ 
//             }
//         }

//         fn get_router() -> ~Router:Send+Share {
//             let router = ~$router_name::new();
//             let routes = vec!( $(($method, $path, $name)),+);
//             for (m, p, n) in routes {
//                 router.add_route(m, p, n);
//             }
//             router as ~Router:Send+Share
//         }
//     }
//     )
// )

/**
A macro for assigning content types. This macro was written by Ogeon for rustful 
https://github.com/Ogeon/rustful but it is extremely useful and relevent so I had to 
add it to oxidize as well.

It takes a main type, a sub type and a parameter list. Instead of this:

```
response.headers.content_type = Some(MediaType {
type_: StrBuf::from_str("text"),
subtype: StrBuf::from_str("html"),
parameters: vec!((StrBuf::from_str("charset"), StrBuf::from_str("UTF-8")))
});
```

it can be written like this:

```
response.headers.content_type = content_type!("text", "html", "charset": "UTF-8");
```

The `"charset": "UTF-8"` part defines the parameter list for the content type.
It may contain more than one parameter, or be omitted:

```
response.headers.content_type = content_type!("application", "octet-stream", "type": "image/gif", "padding": "4");
```

```
response.headers.content_type = content_type!("image", "png");
```
**/
#[macro_export]
macro_rules! content_type(
    ($main_type:expr, $sub_type:expr) => ({
        Some(::http::headers::content_type::MediaType {
            type_: StrBuf::from_str($main_type),
            subtype: StrBuf::from_str($sub_type),
            parameters: Vec::new()
        })
    });

    ($main_type:expr, $sub_type:expr, $($param:expr: $value:expr),+) => ({
        Some(::http::headers::content_type::MediaType {
            type_: StrBuf::from_str($main_type),
            subtype: StrBuf::from_str($sub_type),
            parameters: vec!( $( (StrBuf::from_str($param), StrBuf::from_str($value)) ),+ )
        })
    });
)