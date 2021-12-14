mod checker;

use neon::prelude::*;
use std::collections::BTreeSet;

fn hello(mut cx: FunctionContext) -> JsResult<JsString> {
    Ok(cx.string("hello node"))
}

fn has_sensitive_word(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let content: Handle<JsString> = cx.argument(0)?;
    // TODO unwrap
    let rust_str = content
        .downcast::<JsString, _>(&mut cx)
        .unwrap()
        .value(&mut cx);
    let rusult = checker::is_contains_sensitive_word(&rust_str, &checker::MatchType::MaxMatchType);
    Ok(cx.boolean(rusult))
}

fn init_sensitive_set(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let mut set = BTreeSet::<String>::new();
    let js_array_handle: Handle<JsArray> = cx.argument(0)?;
    let vec: Vec<Handle<JsValue>> = js_array_handle.to_vec(&mut cx)?;
    for (_i, &v) in vec.iter().enumerate() {
        if v.is_a::<JsString, _>(&mut cx) {
            let s = v.downcast::<JsString, _>(&mut cx).unwrap();
            let s = s.value(&mut cx);
            // println!("index/ {}: {}", i, s);
            set.insert(s);
        };
    }
    checker::build_sensitive_word_map(set);
    // println!("{:?}", map);
    Ok(cx.undefined())
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("hello", hello)?;
    cx.export_function("has_sensitive_word", has_sensitive_word)?;
    cx.export_function("init_sensitive_set", init_sensitive_set)?;

    Ok(())
}
