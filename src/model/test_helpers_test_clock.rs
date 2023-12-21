use serde::{Serialize, Deserialize};
/**A test clock enables deterministic control over objects in testmode. With a test clock, you can create
objects at a frozen time in the past or future, and advance to a specific future time to observe webhooks and state changes. After the clock advances,
you can either validate the current state of your scenario (and test your assumptions), change the current state of your scenario (and test more complex scenarios), or keep advancing forward in time.*/
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct TestHelpersTestClock {
    ///Time at which the object was created. Measured in seconds since the Unix epoch.
    pub created: i64,
    ///Time at which this clock is scheduled to auto delete.
    pub deletes_after: i64,
    ///Time at which all objects belonging to this clock are frozen.
    pub frozen_time: i64,
    ///Unique identifier for the object.
    pub id: String,
    ///Has the value `true` if the object exists in live mode or the value `false` if the object exists in test mode.
    pub livemode: bool,
    ///The custom name supplied at creation.
    #[serde(skip_serializing_if = "Option::is_none")]
    pub name: Option<String>,
    ///String representing the object's type. Objects of the same type share the same value.
    pub object: String,
    ///The status of the Test Clock.
    pub status: String,
}
impl std::fmt::Display for TestHelpersTestClock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "{}", serde_json::to_string(self).unwrap())
    }
}