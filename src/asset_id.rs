// asset_id = asset_type / token_id
// asset_type = chain_id / asset_name
// CryptoKitties NFT ID
// eip155:1/erc721:0x06012c8cf97BEaD5deAe237070F9587f8E7A266d/771769
// Stargaze NFT ID
// cosmos:stargaze-1/sg721:stars14x8psqx3xzhmqtdv4qw6v43uf5305sfe0nt4u4/771769

use crate::asset_type::AssetType;
use regex::Regex;
use std::str::FromStr;

struct Caip19Spec<'a> {
  asset_id_regex: &'a str,
  token_id_regex: &'a str,
}

const CAIP19_SPEC: Caip19Spec<'static> = Caip19Spec {
  asset_id_regex: r"[-:a-zA-Z0-9]{13,148}",
  token_id_regex: r"[-a-zA-Z0-9]{1,32}",
};

#[derive(Debug, Clone, PartialEq)]
pub enum AssetIdError {
  InvalidAssetId,
  InvalidAssetIdNumComponents,
  InvalidTokenId,
}

#[derive(Debug, PartialEq)]
struct AssetId {
  asset_type: AssetType,
  token_id: String,
}

impl FromStr for AssetId {
  type Err = AssetIdError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !Regex::new(CAIP19_SPEC.asset_id_regex).unwrap().is_match(s) {
      return Err(AssetIdError::InvalidAssetId);
    }
    let c: Vec<&str> = s.rsplitn(2, "/").collect();
    if c.len() != 2 {
      return Err(AssetIdError::InvalidAssetIdNumComponents);
    }

    let asset_type = AssetType::from_str(c[1]);

    let token_id = c[0];
    if !Regex::new(CAIP19_SPEC.token_id_regex)
      .unwrap()
      .is_match(token_id)
    {
      return Err(AssetIdError::InvalidTokenId);
    }

    Ok(AssetId {
      asset_type: asset_type.unwrap(),
      token_id: token_id.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::asset_id::{AssetId, AssetType, FromStr};

  #[test]
  fn valid_asset_id_eth() {
    let asset_id =
      AssetId::from_str("eip155:1/erc721:0x06012c8cf97BEaD5deAe237070F9587f8E7A266d/771769");
    assert_eq!(
      asset_id.unwrap(),
      AssetId {
        asset_type: AssetType::from_str(
          "eip155:1/erc721:0x06012c8cf97BEaD5deAe237070F9587f8E7A266d"
        )
        .unwrap(),
        token_id: "771769".to_string()
      }
    )
  }

  #[test]
  fn valid_asset_id_stargaze() {
    let asset_id = AssetId::from_str(
      "cosmos:stargaze-1/sg721:stars14x8psqx3xzhmqtdv4qw6v43uf5305sfe0nt4u4/771769",
    );
    assert_eq!(
      asset_id.unwrap(),
      AssetId {
        asset_type: AssetType::from_str(
          "cosmos:stargaze-1/sg721:stars14x8psqx3xzhmqtdv4qw6v43uf5305sfe0nt4u4"
        )
        .unwrap(),
        token_id: "771769".to_string()
      }
    )
  }
}
