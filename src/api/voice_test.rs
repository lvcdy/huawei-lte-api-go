//! Tests for the [`Voice`] API group.

use crate::api::voice::Voice;
use crate::testsupport::conn_with;

/// `voice/featureswitch` resolves.
#[test]
fn featureswitch_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/featureswitch",
        "<response><FeatureSwitch>1</FeatureSwitch></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.featureswitch().expect("ok");
    assert_eq!(value["FeatureSwitch"], "1");
}

/// `voice/sipaccount` resolves.
#[test]
fn sipaccount_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/sipaccount",
        "<response><SipAccount>1000</SipAccount></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.sipaccount().expect("ok");
    assert_eq!(value["SipAccount"], "1000");
}

/// `voice/sipadvance` resolves.
#[test]
fn sipadvance_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/sipadvance",
        "<response><SipAdvance>1</SipAdvance></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.sipadvance().expect("ok");
    assert_eq!(value["SipAdvance"], "1");
}

/// `voice/sipserver` resolves.
#[test]
fn sipserver_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/sipserver",
        "<response><Server>sip.example.com</Server></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.sipserver().expect("ok");
    assert_eq!(value["Server"], "sip.example.com");
}

/// `voice/speeddial` resolves.
#[test]
fn speeddial_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/speeddial",
        "<response><SpeedDial>1</SpeedDial></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.speeddial().expect("ok");
    assert_eq!(value["SpeedDial"], "1");
}

/// `voice/functioncode` resolves.
#[test]
fn functioncode_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/functioncode",
        "<response><FunctionCode>1</FunctionCode></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.functioncode().expect("ok");
    assert_eq!(value["FunctionCode"], "1");
}

/// `voice/voiceadvance` resolves.
#[test]
fn voiceadvance_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/voiceadvance",
        "<response><VoiceAdvance>1</VoiceAdvance></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.voiceadvance().expect("ok");
    assert_eq!(value["VoiceAdvance"], "1");
}

/// `voice/voicebusy` resolves.
#[test]
fn voicebusy_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/voicebusy",
        "<response><VoiceBusy>0</VoiceBusy></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.voicebusy().expect("ok");
    assert_eq!(value["VoiceBusy"], "0");
}

/// `voice/voiperstatus` resolves.
#[test]
fn voiperstatus_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml(
        "api/voice/voiperstatus",
        "<response><VoipError>0</VoipError></response>",
    );

    let voice = Voice::new(&conn);
    let value = voice.voiperstatus().expect("ok");
    assert_eq!(value["VoipError"], "0");
}

/// `voice/volte` resolves.
#[test]
fn volte_returns_json() {
    let (conn, tx) = conn_with();
    tx.route_xml("api/voice/volte", "<response><Volte>1</Volte></response>");

    let voice = Voice::new(&conn);
    let value = voice.volte().expect("ok");
    assert_eq!(value["Volte"], "1");
}
