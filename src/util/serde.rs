use serde::{Deserialize, Deserializer};

pub fn bool_from_int<'de, D>(deserializer: D) -> Result<bool, D::Error>
where
  D: Deserializer<'de>,
{
  let num = <u8 as Deserialize<'de>>::deserialize(deserializer)?;
  Ok(num != 0)
}