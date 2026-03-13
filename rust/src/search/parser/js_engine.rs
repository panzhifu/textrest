use std::cell::RefCell;

use base64::Engine;
use boa_engine::object::ObjectInitializer;
use boa_engine::property::Attribute;
use boa_engine::js_string;
use boa_engine::{Context, JsResult, JsValue, NativeFunction, Source};
use url::form_urlencoded;

thread_local! {
    static JS_CONTEXT: RefCell<Context> = RefCell::new(Context::default());
}

pub fn eval_js(code: &str) -> Option<String> {
    eval_js_with_context(code, None, None, None)
}

pub fn eval_js_with_context(
    code: &str,
    result: Option<&str>,
    base_url: Option<&str>,
    book_json: Option<&str>,
) -> Option<String> {
    let code = code.trim();
    if code.is_empty() {
        return None;
    }

    JS_CONTEXT.with(|ctx| {
        let mut ctx = ctx.borrow_mut();
        if let Err(err) = prepare_context(&mut ctx, result, base_url, book_json) {
            return Some(err.to_string());
        }
        match ctx.eval(Source::from_bytes(code)) {
            Ok(value) => js_value_to_string(&value, &mut ctx),
            Err(err) => {
                let message = err.to_string();
                if message.is_empty() {
                    None
                } else {
                    Some(message)
                }
            }
        }
    })
}

fn prepare_context(
    ctx: &mut Context,
    result: Option<&str>,
    base_url: Option<&str>,
    book_json: Option<&str>,
) -> JsResult<()> {
    let global = ctx.global_object();

    if let Some(result) = result {
        global.set(
            js_string!("result"),
            JsValue::from(js_string!(result)),
            true,
            ctx,
        )?;
    }
    if let Some(base_url) = base_url {
        global.set(
            js_string!("baseUrl"),
            JsValue::from(js_string!(base_url)),
            true,
            ctx,
        )?;
    }
    if let Some(book_json) = book_json {
        if let Ok(value) = ctx.eval(Source::from_bytes(book_json)) {
            global.set(js_string!("book"), value, true, ctx)?;
        }
    }

    let java_obj = ObjectInitializer::new(ctx)
        .function(NativeFunction::from_fn_ptr(js_encode_uri), js_string!("encodeURI"), 1)
        .function(
            NativeFunction::from_fn_ptr(js_base64_encode),
            js_string!("base64Encode"),
            1,
        )
        .function(
            NativeFunction::from_fn_ptr(js_base64_decode),
            js_string!("base64Decode"),
            1,
        )
        .build();
    global.set(js_string!("java"), java_obj, true, ctx)?;

    let mut window_obj = ObjectInitializer::new(ctx);
    if let Some(base_url) = base_url {
        window_obj.property(
            js_string!("location"),
            JsValue::from(js_string!(base_url)),
            Attribute::all(),
        );
        window_obj.property(
            js_string!("href"),
            JsValue::from(js_string!(base_url)),
            Attribute::all(),
        );
    }
    let window_obj = window_obj.build();
    global.set(js_string!("window"), window_obj, true, ctx)?;

    Ok(())
}

fn js_encode_uri(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let input = args
        .get(0)
        .cloned()
        .unwrap_or_default()
        .to_string(ctx)?
        .to_std_string_escaped();
    let encoded: String = form_urlencoded::byte_serialize(input.as_bytes()).collect();
    Ok(JsValue::from(js_string!(encoded)))
}

fn js_base64_encode(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let input = args
        .get(0)
        .cloned()
        .unwrap_or_default()
        .to_string(ctx)?
        .to_std_string_escaped();
    let encoded = base64::engine::general_purpose::STANDARD.encode(input.as_bytes());
    Ok(JsValue::from(js_string!(encoded)))
}

fn js_base64_decode(_this: &JsValue, args: &[JsValue], ctx: &mut Context) -> JsResult<JsValue> {
    let input = args
        .get(0)
        .cloned()
        .unwrap_or_default()
        .to_string(ctx)?
        .to_std_string_escaped();
    let decoded = base64::engine::general_purpose::STANDARD
        .decode(input.as_bytes())
        .unwrap_or_default();
    let output = String::from_utf8(decoded).unwrap_or_default();
    Ok(JsValue::from(js_string!(output)))
}

fn js_value_to_string(value: &JsValue, ctx: &mut Context) -> Option<String> {
    if value.is_null() || value.is_undefined() {
        return None;
    }

    match value.to_string(ctx) {
        Ok(js_str) => {
            let out = js_str.to_std_string_escaped();
            if out.trim().is_empty() {
                None
            } else {
                Some(out)
            }
        }
        Err(_) => None,
    }
}
