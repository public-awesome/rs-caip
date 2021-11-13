use std::fmt;
use std::str::FromStr;

#[derive(Debug, Clone, PartialEq)]
struct ChainIdError;

#[derive(Debug, PartialEq)]
struct ChainId {
  namespace: String,
  reference: String,
}

impl fmt::Display for ChainIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "invalid chain id")
  }
}

impl FromStr for ChainId {
  type Err = ChainIdError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    let params: Vec<&str> = s.split(":").collect();
    if params.len() != 2 {
      return Err(ChainIdError);
    }
    Ok(ChainId {
      namespace: params[0].to_string(),
      reference: params[1].to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::{ChainId, ChainIdError, FromStr};

  #[test]
  fn valid_chain_id() {
    let id = ChainId::from_str("cosmos:stargaze-3");
    assert_eq!(
      id.unwrap(),
      ChainId {
        namespace: "cosmos".to_string(),
        reference: "stargaze-3".to_string()
      }
    )
  }

  #[test]
  fn invalid_chain_id() {
    let id = ChainId::from_str("cosmos:stargaze-3:invalid");
    assert_eq!(id.unwrap_err(), ChainIdError)
  }
}
