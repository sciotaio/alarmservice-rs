extern crate serde;
extern crate serde_derive;
extern crate serde_json;
extern crate url;
extern crate reqwest;
extern crate tsify_next;
extern crate wasm_bindgen;

// previously, we imported macros from 'serde_derive' via 'use_macro' directive. 
// this, however, leads to a warning about unused import whenever the 'model' crate is compiled from an importing crate.
// the simple workaround is to directly import the macros in our dtos like this:
// use serde_derive::Serialize;
// use serde_derive::Deserialize;

pub mod models;