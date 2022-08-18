use std::rc::Rc;

use wasm_bindgen::UnwrapThrowExt;
use web_sys::{HtmlInputElement, HtmlSelectElement, InputEvent};
use yew::{Callback, TargetCast};

pub fn convert_string_callback(callback: Callback<String>) -> Callback<InputEvent> {
    convert_callback(
        callback,
        Box::new(|input: InputEvent| {
            let element: HtmlInputElement = input.target_dyn_into().unwrap_throw();
            element.value()
        }),
    )
}

/// # Overview
/// This function converts the [Callback] fired by the change of an HTML ```<select>``` into a [Callback] containing any ```Enum``` that implements ```try_from<&str>```.
/// The try_from has to have a match for the  ```value``` of any ```<option>``` inside of the ```<select>```.
/// It returns the try_from's ```Error``` if the try_from fails, (because a value wasnt matched).
///
/// Note, that the function is backwards - it takes the output callback (with the enum value) as parameter,
/// and returns a ```Callback<InputEvent>``` that you then can add to the ```onchange``` of a ```<select>``` like so:
/// ```rust
/// enum E {
///     V1,
///     V2
/// }
///
/// impl TryFrom<&str> for E {
///     type Error = ();
///     
///     fn try_from(s: &str) -> Result<Self, Self::Error> {
///         let result = match s.to_lowercase().as_str() {
///             "1" => Self::V1,
///             "2" => Self::V2,
///             _ => return Err(()),
///         };
///
///         Ok(result)
///     }
/// }
///
/// let outer_callback: Callback<Result<String>> = Callback::noop();
/// html!{
///     <select onchange={convert_select_input_to_enum_callback(outer_callback)}>
///         <option value="1">1</option>
///         <option value="2">2</option>
///     </select>
/// };
///
/// ```
///
/// # Panic
/// Unwrap_throw()s when invoked on any other than an HTML ```<select>``` element.
pub fn convert_select_input_to_enum_callback<Enum, Error>(
    callback: Callback<Result<Enum, Error>>,
) -> Callback<InputEvent>
where
    Enum: for<'a> TryFrom<&'a str, Error = Error> + 'static,
    Error: 'static,
{
    let transform: fn(InputEvent) -> String = |event| {
        event
            .target_dyn_into::<HtmlSelectElement>()
            .unwrap_throw()
            .value()
    };

    convert_enum_callback::<InputEvent, Enum, Error>(callback, transform)
}

/// This function converts a [Callback] containing InputEvent into an enum (or struct) that implements [TryFrom] for &str.
/// The input transform function provided is used to convert the InputEvent into a String, which is then used to constuct the enum.
pub fn convert_enum_callback<Input, Output, Error>(
    callback: Callback<Result<Output, Error>>,
    input_transform: fn(Input) -> String,
) -> Callback<Input>
where
    Input: 'static,
    Output: for<'a> TryFrom<&'a str, Error = Error> + 'static,
    Error: 'static,
{
    convert_callback(
        callback,
        Box::new(move |input| {
            let intermediate = input_transform(input);
            intermediate.as_str().try_into()
        }),
    )
}

pub fn convert_callback<Input, Output>(
    callback: Callback<Output>,
    function: Box<dyn Fn(Input) -> Output>,
) -> Callback<Input>
where
    Input: 'static,
    Output: 'static,
{
    let passive = if let Callback::Callback { cb: _, passive } = callback {
        passive
    } else {
        None
    };
    Callback::Callback {
        cb: Rc::from(move |input: Input| {
            callback.emit(function(input));
        }),
        passive,
    }
}
