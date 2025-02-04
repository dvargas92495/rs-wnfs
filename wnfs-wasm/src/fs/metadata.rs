use crate::value;
use js_sys::{Object, Reflect};
use libipld::Ipld;
use wasm_bindgen::JsValue;
use wnfs::Metadata;

use super::utils::error;

pub(crate) struct JsMetadata<'a>(pub(crate) &'a Metadata);

impl TryFrom<JsMetadata<'_>> for JsValue {
    type Error = js_sys::Error;

    fn try_from(value: JsMetadata<'_>) -> Result<Self, Self::Error> {
        let metadata = Object::new();

        if let Some(Ipld::Integer(i)) = value.0 .0.get("created") {
            Reflect::set(
                &metadata,
                &value!("created"),
                &value!(i64::try_from(*i).map_err(error("Cannot convert created value"))?),
            )?;
        }

        if let Some(Ipld::Integer(i)) = value.0 .0.get("modified") {
            Reflect::set(
                &metadata,
                &value!("modified"),
                &value!(i64::try_from(*i).map_err(error("Cannot convert modified value"))?),
            )?;
        }

        Ok(value!(metadata))
    }
}
