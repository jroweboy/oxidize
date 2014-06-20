#![crate_id = "oxidize_macros#0.2-pre"]

#![comment = "Macros for oxidize"]
#![license = "MIT"]
#![crate_type = "dylib"]

#![feature(macro_rules, plugin_registrar, managed_boxes, quote, phase)]
#![feature(trace_macros)]
/**
router! is a macro that is intended to ease the implementation of App for your App Struct

The goal is to take this

```
impl App for HelloWorld {
    fn handle_route<'a>(&self, route: Option<&&'static str>, vars: Option<HashMap<String,String>>,
                         req: &mut Request) -> Response {
        if route.is_none() {
            // 404
            return self.default_404(req);
        } else {
            match *route.unwrap() {
                "index" => {
                    self.index()
                }
                "mustache" => {
                    Response::empty()
                }
                _ => {
                    unreachable!();
                }
            }
        }
    }
    fn get_router(&self) -> Router<&'static str> {
        let routes = [
            (method::Get, "/", "index"),
            (method::Get, "/mustache", "mustache"),
        ];
        Router::from_routes(routes)
    }
    fn default_404(&self, &mut Request) -> Response {
        //... 404 code here
    }
}
```

and simplify it to this 

```
routes!{ HelloWorld,
    (Get, "/", "index", self.index),
    (Get, "/mustache", "mustache", self.test_mustache),
    (Get, "/users/user-<userid:uint>/post-<postid:uint>", "test_variable", self.test_variable),
    fn default_404(&self, &mut Request, &mut ResponseWriter) {
        //... Optional 404 code if you want to override the default 404 page
    }
}
```

Additionally, it should support variable binding, such that <userid:uint> should attempt to bind the
captured variable and pass it to the function as a `uint` otherwise it will 404. Maybe it should Bad Request
instead, but thats easy enough to change in the future.

**/

extern crate syntax;

#[allow(unused_imports)]
use std::path::BytesContainer;

#[allow(unused_imports)]
use syntax::{ast, codemap};
#[allow(unused_imports)]
use syntax::ext::base::{
    SyntaxExtension, ExtCtxt, MacResult, MacExpr,
    NormalTT, BasicMacroExpander, MacItem,
};
#[allow(unused_imports)]
use syntax::ext::build::AstBuilder;
#[allow(unused_imports)]
use syntax::ext::quote::rt::ExtParseUtils;
#[allow(unused_imports)]
use syntax::parse;
#[allow(unused_imports)]
use syntax::parse::token;
#[allow(unused_imports)]
use syntax::parse::parser;
#[allow(unused_imports)]
use syntax::parse::parser::Parser;
#[allow(unused_imports)]
use syntax::ast::{Method};
#[allow(unused_imports)]
use syntax::print::pprust;
use syntax::ast;


#[plugin_registrar]
#[doc(hidden)]
pub fn macro_registrar(register: |ast::Name, SyntaxExtension|) {
    let expander = box BasicMacroExpander{expander: expand_router, span: None};
    register(token::intern("router"), NormalTT(expander, None));
}


// (04:53:50 PM) jroweboy: but I'm just asking about the outer impl block right now. Is that considered a block? Or maybe the mysterious trait_ref? I actually have no clue haha
// (04:53:58 PM) huon: jroweboy: an Item
// (04:54:05 PM) huon: jroweboy: however you can probably use a quote macro
// (04:54:19 PM) huon: jroweboy: quote_item!(cx, impl $trait for $type { $methods })
// (04:54:36 PM) jroweboy: how is that different than macro_rules! ?
// (04:54:50 PM) huon: this has to be inside your procedural macro
// (04:54:56 PM) jroweboy: oh, I see it goes inside the macro
// (04:55:04 PM) huon: i.e. you still need #[macro_registrar] etc.
// (04:55:31 PM) huon: (all the $'s are local variables of that name, trait might be an ident, type would be a type and methods would be a Vec<Method> or &[Method])
// (04:55:45 PM) Luqman: jroweboy: quote_item will just give you the AST structure representing what you pass it so you don't have to manually build it up yourself
// (04:55:52 PM) huon: (however, I guess the quoter may fail for methods, not sure... if it does, you'll just have to build the ItemImpl yourself.)


fn expand_router(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
    // fn parse_str(&mut self) -> (InternedString, StrStyle)
    // ItemImpl for the Impl
    // Trait as an Ident
    // Type as a Type 
    // methods as a Vec of Methods ?fn item_fn?

    // make a parser for router! { 
    // trace_macros!(true);
    let mut parser = parse::new_parser_from_tts(
        cx.parse_sess(), cx.cfg(), Vec::from_slice(tts)
    );
    // get the name of the struct to impl on
    let struct_ident = parser.parse_ident();
    parser.expect(&token::COMMA);
    parser.eat(&token::COMMA);

    let trait_name = cx.ident_of("App");

    // let methods = parser.parse_trait_methods();
    // let methods : Vec<Method> = Vec::new();
    // TODO: Change the App to be ::oxidize::App after testing
    let impl_item = quote_item!(cx, impl $trait_name for $struct_ident { }).unwrap();
    let methods = match impl_item.node {
        ast::ItemImpl(_, _, _, ref methods) => methods,
        _ => unreachable!()
    };
    //methods.push(
    MacItem::new(impl_item)
}

// #[allow(dead_code)] 
// fn ogeon_expand_router(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
//     let router_ident = cx.ident_of("router");
//     let insert_method = cx.ident_of("insert_item");

//     let mut calls: Vec<@ast::Stmt> = vec!(
//         cx.stmt_let(sp, true, router_ident, quote_expr!(&cx, ::rustful::Router::new()))
//     );

//     for (path, method, handler) in parse_routes(cx, tts).move_iter() {
//         let path_expr = cx.parse_expr(format!("\"{}\"", path));
//         let method_expr = cx.expr_path(method);
//         let handler_expr = cx.expr_path(handler);
//         calls.push(cx.stmt_expr(
//             cx.expr_method_call(sp, cx.expr_ident(sp, router_ident), insert_method, vec!(method_expr, path_expr, handler_expr))
//         ));
//     }

//     let block = cx.expr_block(cx.block(sp, calls, Some(cx.expr_ident(sp, router_ident))));
    
//     MacExpr::new(block)
// }

// #[allow(dead_code)] 
// fn expand_routes(cx: &mut ExtCtxt, sp: codemap::Span, tts: &[ast::TokenTree]) -> Box<MacResult> {
//     let routes = parse_routes(cx, tts).move_iter().map(|(path, method, handler)| {
//         let path_expr = cx.parse_expr(format!("\"{}\"", path));
//         let method_expr = cx.expr_path(method);
//         let handler_expr = cx.expr_path(handler);
//         mk_tup(sp, vec!(method_expr, path_expr, handler_expr))
//     }).collect();

//     MacExpr::new(cx.expr_vec(sp, routes))
// }

// #[allow(dead_code)] 
// fn parse_routes(cx: &mut ExtCtxt, tts: &[ast::TokenTree]) -> Vec<(String, ast::Path, ast::Path)> {

//     let mut parser = parse::new_parser_from_tts(
//         cx.parse_sess(), cx.cfg(), Vec::from_slice(tts)
//     );

//     parse_subroutes("", cx, &mut parser)
// }

// #[allow(dead_code)] 
// fn parse_subroutes(base: &str, cx: &mut ExtCtxt, parser: &mut Parser) -> Vec<(String, ast::Path, ast::Path)> {
//     let mut routes = Vec::new();

//     while !parser.eat(&token::EOF) {
//         match parser.parse_optional_str() {
//             Some((ref s, _)) => {
//                 if !parser.eat(&token::FAT_ARROW) {
//                     parser.expect(&token::FAT_ARROW);
//                     break;
//                 }

//                 let mut new_base = base.to_string();
//                 match s.container_as_str() {
//                     Some(s) => {
//                         new_base.push_str(s.trim_chars('/'));
//                         new_base.push_str("/");
//                     },
//                     None => cx.span_err(parser.span, "invalid path")
//                 }

//                 if parser.eat(&token::EOF) {
//                     cx.span_err(parser.span, "unexpected end of routing tree");
//                 }

//                 if parser.eat(&token::LBRACE) {
//                     let subroutes = parse_subroutes(new_base.as_slice(), cx, parser);
//                     routes.push_all(subroutes.as_slice());

//                     if parser.eat(&token::RBRACE) {
//                         if !parser.eat(&token::COMMA) {
//                             break;
//                         }
//                     } else {
//                         parser.expect_one_of([token::COMMA, token::RBRACE], []);
//                     }
//                 } else {
//                     for (method, handler) in parse_handler(parser).move_iter() {
//                         routes.push((new_base.clone(), method, handler))
//                     }

//                     if !parser.eat(&token::COMMA) {
//                         break;
//                     }
//                 }
//             },
//             None => {
//                 for (method, handler) in parse_handler(parser).move_iter() {
//                     routes.push((base.to_string(), method, handler))
//                 }

//                 if !parser.eat(&token::COMMA) {
//                     break;
//                 }
//             }
//         }
//     }

//     routes
// }

// #[allow(dead_code)] 
// fn parse_handler(parser: &mut Parser) -> Vec<(ast::Path, ast::Path)> {
//     let mut methods = Vec::new();

//     loop {
//         methods.push(parser.parse_path(parser::NoTypesAllowed).path);

//         if parser.eat(&token::COLON) {
//             break;
//         }

//         if !parser.eat(&token::BINOP(token::OR)) {
//             parser.expect_one_of([token::COLON, token::BINOP(token::OR)], []);
//         }
//     }

//     let handler = parser.parse_path(parser::NoTypesAllowed).path;

//     methods.move_iter().map(|m| (m, handler.clone())).collect()
// }

// #[allow(dead_code)] 
// fn mk_tup(sp: codemap::Span, content: Vec<@ast::Expr>) -> @ast::Expr {
//     dummy_expr(sp, ast::ExprTup(content))
// }

// #[allow(dead_code)] 
// fn dummy_expr(sp: codemap::Span, e: ast::Expr_) -> @ast::Expr {
//     @ast::Expr {
//         id: ast::DUMMY_NODE_ID,
//         node: e,
//         span: sp,
//     }
// }




/**
A macro for assigning content types. Written by Ogeon (github.com/ogeon)

It takes a main type, a sub type and a parameter list. Instead of this:

```
response.headers.content_type = Some(MediaType {
    type_: String::from_str("text"),
    subtype: String::from_str("html"),
    parameters: vec!((String::from_str("charset"), String::from_str("UTF-8")))
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
        Some(::oxidize::common::content_type::MediaType {
            type_: String::from_str($main_type),
            subtype: String::from_str($sub_type),
            parameters: Vec::new()
        })
    });

    ($main_type:expr, $sub_type:expr, $($param:expr: $value:expr),+) => ({
        Some(::oxidize::common::content_type::MediaType {
            type_: String::from_str($main_type),
            subtype: String::from_str($sub_type),
            parameters: vec!( $( (String::from_str($param), String::from_str($value)) ),+ )
        })
    });
)
