// This code is taken from [async-stripe](https://github.com/arlyon/async-stripe/tree/0a00d31894191ee0c6b4bda31e0d52d59e8e93b7)
// Author: Alexander Lyon
// License under either of:
//      - Apache License, Version 2.0, (LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0)
//      - MIT License (LICENSE-MIT or https://opensource.org/licenses/MIT)

// ======================================
// This file was automatically generated.
// ======================================

use serde::{Deserialize, Serialize};

use crate::ids::UsageRecordId;
use crate::params::{Object, Timestamp};

/// The resource representing a Stripe "UsageRecord".
///
/// For more details see <https://stripe.com/docs/api/usage_records/object>
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct UsageRecord {
    /// Unique identifier for the object.
    pub id: UsageRecordId,

    /// Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,

    /// The usage quantity for the specified date.
    pub quantity: u64,

    /// The ID of the subscription item this usage record contains data for.
    pub subscription_item: String,

    /// The timestamp when this usage occurred.
    pub timestamp: Timestamp,
}

impl Object for UsageRecord {
    type Id = UsageRecordId;
    fn id(&self) -> Self::Id {
        self.id.clone()
    }
    fn object(&self) -> &'static str {
        "usage_record"
    }
}
