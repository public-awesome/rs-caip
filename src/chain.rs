use regex::Regex;
use std::fmt;
use std::str::FromStr;

struct Caip2Spec<'a> {
  delimiter: &'a str,
  chain_id_regex: &'a str,
  namespace_regex: &'a str,
  reference_regex: &'a str,
}

const CAIP2_SPEC: Caip2Spec<'static> = Caip2Spec {
  delimiter: ":",
  chain_id_regex: r"[-:a-zA-Z0-9]{5,41}",
  namespace_regex: r"[-a-z0-9]{3,8}",
  reference_regex: r"[-a-zA-Z0-9]{1,32}",
};

#[derive(Debug, Clone, PartialEq)]
pub enum ChainIdError {
  InvalidChainId,
  InvalidNamespace,
  InvalidReference,
}

#[derive(Debug, PartialEq)]
struct ChainId {
  namespace: String,
  reference: String,
}

impl fmt::Display for ChainIdError {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    match self {
      ChainIdError::InvalidChainId => write!(f, "invalid chain id"),
      ChainIdError::InvalidNamespace => write!(f, "invalid chain id namespace"),
      ChainIdError::InvalidReference => write!(f, "invalid chain id reference"),
    }
  }
}

impl FromStr for ChainId {
  type Err = ChainIdError;

  fn from_str(s: &str) -> Result<Self, Self::Err> {
    if !Regex::new(CAIP2_SPEC.chain_id_regex).unwrap().is_match(s) {
      return Err(ChainIdError::InvalidChainId);
    }
    let params: Vec<&str> = s.split(CAIP2_SPEC.delimiter).collect();
    if params.len() != 2 {
      return Err(ChainIdError::InvalidChainId);
    }

    let namespace = params[0];
    if !Regex::new(CAIP2_SPEC.namespace_regex)
      .unwrap()
      .is_match(namespace)
    {
      return Err(ChainIdError::InvalidNamespace);
    }

    let reference = params[1];
    if !Regex::new(CAIP2_SPEC.reference_regex)
      .unwrap()
      .is_match(reference)
    {
      return Err(ChainIdError::InvalidReference);
    }

    Ok(ChainId {
      namespace: namespace.to_string(),
      reference: reference.to_string(),
    })
  }
}

#[cfg(test)]
mod tests {
  use crate::chain::{ChainId, ChainIdError, FromStr};

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
  fn invalid_chain_id_too_short() {
    let id = ChainId::from_str("cos");
    assert_eq!(id.unwrap_err(), ChainIdError::InvalidChainId)
  }

  #[test]
  fn invalid_chain_id_too_many_colons() {
    let id = ChainId::from_str("cosmos:stargaze-3:invalid");
    assert_eq!(id.unwrap_err(), ChainIdError::InvalidChainId)
  }

  #[test]
  fn invalid_chain_id_namespace_too_short() {
    let id = ChainId::from_str("c:stargaze-3");
    assert_eq!(id.unwrap_err(), ChainIdError::InvalidNamespace)
  }

  // #[test]
  // fn invalid_chain_id_namespace_too_long() {
  //   let id = ChainId::from_str("ckasfasd;lfkjas;flkjasd;flkj:stargaze-3");
  //   assert_eq!(id.unwrap_err(), ChainIdError)
  // }

  #[test]
  fn invalid_chain_id_bad_reference_too_short() {
    let id = ChainId::from_str("cosmos:");
    assert_eq!(id.unwrap_err(), ChainIdError::InvalidReference)
  }

  // #[test]
  // fn invalid_chain_id_bad_reference_too_long() {
  //   let id = ChainId::from_str("cosmos:adkslfjalkdfhaksfja;sdkfjasdlkfjhasdlfkjhaslfkjsadhflkasjdhflasdkfjhadlskfjahdslfkjadhfladjkfhadlkfjhaljfhalkfjhadlfkjh");
  //   assert_eq!(id.unwrap_err(), ChainIdError)
  // }
}
