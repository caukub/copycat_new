use boa_engine::{
    Context, JsValue, JsResult, NativeFunction, 
    object::ObjectInitializer, property::Attribute, js_string
};

fn my_log(_this: &JsValue, args: &[JsValue], context: &mut Context) -> JsResult<JsValue> {
    let message = args.get(0).cloned().unwrap_or_default().to_string(context)?;
    println!("Log: {}", message.to_std_string_escaped());
    Ok(JsValue::undefined())
}

pub fn run_js() {
    let mut context = Context::default();

    // Create the object and attach the method using the initializer
    let custom_obj = ObjectInitializer::new(&mut context)
        .function(NativeFunction::from_fn_ptr(my_log), js_string!("log"), 0)
        .build();

    // Register it globally
    context.register_global_property(
        js_string!("myConsole"),
        custom_obj,
        Attribute::all(),
    );

    context.eval(boa_engine::Source::from_bytes("myConsole.log('It works!');")).unwrap();
}