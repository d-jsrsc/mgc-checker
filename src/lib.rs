mod checker;

use neon::prelude::*;
use std::collections::BTreeSet;

use lazy_static::lazy_static;

lazy_static! {
    static ref THREAD_POOL: rayon::ThreadPool = init_pool();
}

const RAYON_THREAD_NUM: usize = 5;
fn init_pool() -> rayon::ThreadPool {
    rayon::ThreadPoolBuilder::new()
        .num_threads(RAYON_THREAD_NUM)
        .build()
        .unwrap()
}

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

fn has_sensitive_word_async(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let content = cx.argument::<JsString>(0)?;
    let callback = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let queue = cx.channel();

    // TODO unwrap
    let rust_str = content
        .downcast::<JsString, _>(&mut cx)
        .unwrap()
        .value(&mut cx);

    THREAD_POOL.spawn(move || {
        queue.send(move |mut cx| {
            let result =
                checker::is_contains_sensitive_word(&rust_str, &checker::MatchType::MaxMatchType);
            let callback = callback.into_inner(&mut cx);
            let this = cx.undefined();
            let args = vec![
                cx.null().upcast::<JsValue>(),
                JsBoolean::new(&mut cx, result).upcast::<JsValue>(),
            ];
            callback.call(&mut cx, this, args)?;
            Ok(())
        })
    });
    Ok(cx.undefined())
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
    cx.export_function("initSensitiveSet", init_sensitive_set)?;
    cx.export_function("hasSensitiveWordSync", has_sensitive_word)?;
    cx.export_function("hasSensitiveWord", has_sensitive_word_async)?;

    Ok(())
}
