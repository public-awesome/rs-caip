// asset_name = asset_namespace : asset_reference

use regex::Regex;
use std::str::FromStr;

struct Caip19AssetNameSpec<'a> {
  asset_name_regex: &'a str,
  asset_namespace_regex: &'a str,
  asset_reference_regex: &'a str,
}

const CAIP19_ASSET_NAME_SPEC: Caip19AssetNameSpec<'static> = Caip19AssetNameSpec {
  asset_name_regex: r"[-:a-zA-Z0-9]{5,73}",
  asset_namespace_regex: r"[-a-z0-9]{3,8}",
  asset_reference_regex: r"[-a-zA-Z0-9]{1,64}",
};

#[derive(Debug, Clone, PartialEq)]
pub enum AssetNameError {
  InvalidName,
  InvalidNamespace,
  InvalidReference,
}

#[derive(Debug, PartialEq)]
pub struct AssetName {
  namespace: String,
  reference: String,
}

impl FromStr for AssetName {
  type Err = AssetNameError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !Regex::new(CAIP19_ASSET_NAME_SPEC.asset_name_regex)
      .unwrap()
      .is_match(s)
    {
      return Err(AssetNameError::InvalidName);
    }
    let c: Vec<&str> = s.split(":").collect();
    if c.len() != 2 {
      return Err(AssetNameError::InvalidName);
    }
    let namespace = c[0];
    if !Regex::new(CAIP19_ASSET_NAME_SPEC.asset_namespace_regex)
      .unwrap()
      .is_match(namespace)
    {
      return Err(AssetNameError::InvalidNamespace);
    }
    let reference = c[1];
    if !Regex::new(CAIP19_ASSET_NAME_SPEC.asset_reference_regex)
      .unwrap()
      .is_match(reference)
    {
      return Err(AssetNameError::InvalidReference);
    }

    Ok(AssetName {
      namespace: namespace.to_string(),
      reference: reference.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::asset_name::{AssetName, FromStr};

  #[test]
  fn valid_asset_name() {
    let name = AssetName::from_str("slip44:563");
    assert_eq!(
      name.unwrap(),
      AssetName {
        namespace: "slip44".to_string(),
        reference: "563".to_string(),
      }
    )
  }
}
