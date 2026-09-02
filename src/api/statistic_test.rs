//! Tests for the [`Statistic`] API group.

use crate::api::statistic::Statistic;
use crate::testsupport::conn_with;

/// `statistic/feature-roam-statistic` resolves.
#[test]
fn feature_roam_statistic_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/statistic/feature-roam-statistic",
        "<response><FeatureSwitch>1</FeatureSwitch></response>",
    );

    let stat = Statistic::new(&conn);
    let value = stat.feature_roam_statistic().expect("ok");
    assert_eq!(value["FeatureSwitch"], "1");
}
