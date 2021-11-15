// asset_type = chain_id / asset_name
// chain_id = chain_namespace : chain_reference
// asset_name = asset_namespace : asset_reference
// Stargaze Token
// cosmos:stargaze-1/slip44:563

use crate::asset_name::AssetName;
use crate::chain::ChainId;
use regex::Regex;
use std::str::FromStr;

struct Caip19AssetTypeSpec<'a> {
  asset_type_regex: &'a str,
}

const CAIP19_ASSET_TYPE_SPEC: Caip19AssetTypeSpec<'static> = Caip19AssetTypeSpec {
  asset_type_regex: r"[-:a-zA-Z0-9]{11,115}",
};

#[derive(Debug, Clone, PartialEq)]
pub enum AssetTypeError {
  InvalidType,
}

#[derive(Debug, PartialEq)]
pub struct AssetType {
  chain_id: ChainId,
  asset_name: AssetName,
}

impl FromStr for AssetType {
  type Err = AssetTypeError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !Regex::new(CAIP19_ASSET_TYPE_SPEC.asset_type_regex)
      .unwrap()
      .is_match(s)
    {
      return Err(AssetTypeError::InvalidType);
    }
    let c: Vec<&str> = s.split("/").collect();
    if c.len() != 2 {
      return Err(AssetTypeError::InvalidType);
    }

    let chain_id = ChainId::from_str(c[0]);
    let asset_name = AssetName::from_str(c[1]);

    Ok(AssetType {
      chain_id: chain_id.unwrap(),
      asset_name: asset_name.unwrap(),
    })
  }
}

#[derive(Debug, PartialEq)]
struct AssetId {
  asset_type: AssetType,
  token_id: String,
}

#[cfg(test)]
mod tests {
  use crate::asset_type::{AssetName, AssetType, FromStr};
  use crate::chain::ChainId;

  #[test]
  fn valid_asset_type() {
    let asset_type = AssetType::from_str("cosmos:stargaze-1/slip44:563");
    assert_eq!(
      asset_type.unwrap(),
      AssetType {
        chain_id: ChainId::from_str("cosmos:stargaze-1").unwrap(),
        asset_name: AssetName::from_str("slip44:563").unwrap(),
      }
    )
  }
}
