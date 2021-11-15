# Rust CAIP Standard Utilities

Chain Agnostic Improvement Proposals ([CAIP](https://github.com/ChainAgnostic/CAIPs)) is a set of standards for multi-chain interoperability.

## CAIP-2: Chain ID

```rs
use caip::chain::{ChainId, FromStr};

let id = ChainId::from_str("cosmos:stargaze-3");
```

## CAIP-19: Asset Type

```rs
use crate::asset_type::{AssetType, FromStr};

let asset_type = AssetType::from_str("cosmos:stargaze-1/slip44:563");
```

## CAIP-19: Asset ID

Ethereum assets:

```rs
use crate::asset_id::{AssetId, FromStr};

let asset_id =
  AssetId::from_str("eip155:1/erc721:0x06012c8cf97BEaD5deAe237070F9587f8E7A266d/771769");
```

Stargaze assets:

```rs
use crate::asset_id::{AssetId, FromStr};

let asset_id = AssetId::from_str(
  "cosmos:stargaze-1/sg721:stars14x8psqx3xzhmqtdv4qw6v43uf5305sfe0nt4u4/771769",
);
```
