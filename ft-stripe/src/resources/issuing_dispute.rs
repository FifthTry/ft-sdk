// Code taken from https://github.com/wyyerd/stripe-rs/tree/c2f03f8dec41e20b66f9bbe902b8384096ac653c
// ======================================
// This file was automatically generated.
// ======================================

use crate::ids::IssuingDisputeId;
use crate::params::Object;
use serde::{Deserialize, Serialize};

/// The resource representing a Stripe "IssuingDispute".
#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct IssuingDispute {
    /// Unique identifier for the object.
    pub id: IssuingDisputeId,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
}

impl Object for IssuingDispute {
    type Id = IssuingDisputeId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "issuing.dispute"
    }
}
