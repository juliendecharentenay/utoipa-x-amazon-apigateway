use super::*;

/// Utility function to merge extensions. utoipa provides a method which only look at the extension name level. So specify 
/// 2 extensions with the name will overwrite each other even if the extension is meant to be an array or object. This
/// prevent extension chaining.
pub fn merge_extensions(extensions: &mut utoipa::openapi::extensions::Extensions, other: &utoipa::openapi::extensions::Extensions) {
  for (key, value) in other.iter() {
    match extensions.get_mut(key) {
      Some(serde_json::value::Value::Null) 
      | Some(serde_json::value::Value::Bool(_))
      | Some(serde_json::value::Value::Number(_))
      | Some(serde_json::value::Value::String(_))
      | None 
      => { extensions.insert(key.clone(), value.clone()); },
      Some(serde_json::value::Value::Array(arr)) 
      => {
        if let serde_json::value::Value::Array(my_arr) = value {
          arr.append(&mut my_arr.clone());
        } else {
          log::warn!("Unable to merge Array with incompatible type");
        }
      },
      Some(serde_json::value::Value::Object(map)) 
      => {
        if let serde_json::value::Value::Object(my_map) = value {
          // Possible need to merge value here in place of overwriting.
          for (k,v) in my_map.iter() { map.insert(k.clone(), v.clone()); }
        } else {
          log::warn!("Unable to merge Object with incompatible type");
        }
      },
    }
  }
}

/// Utility function to serialize a ref
pub fn serialize_ref<S>(serializer: S, v: String) -> std::result::Result<S::Ok, S::Error>
where S: serde::Serializer
{
  use serde::ser::SerializeMap;
  let mut s = serializer.serialize_map(Some(1))?;
  s.serialize_entry("$ref", &v)?;
  s.end()
}
