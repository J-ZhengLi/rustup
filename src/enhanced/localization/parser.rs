// TODO: Remove this allow when implementing this feature
#![allow(dead_code)]

use serde::Deserialize;

#[derive(Deserialize)]
struct Locale {
    help: Help,
    errors: Errors,
}

#[derive(Deserialize)]
struct Help {

}

#[derive(Deserialize)]
struct Errors {

}