package session

import (
	"bytes"
	"testing"
)

// 对应 tests/test_session_cesu8.py 的 test_cesu8_encode。
func TestCesu8Encode(t *testing.T) {
	cases := []struct {
		text     string
		expected []byte
	}{
		{"", []byte{}},
		{"ASCII", []byte("ASCII")},
		{"\u00e9\u20ac", []byte("\xc3\xa9\xe2\x82\xac")},
		{"\U00010000", []byte("\xed\xa0\x80\xed\xb0\x80")},
		{"\U0001f600", []byte("\xed\xa0\xbd\xed\xb8\x80")},
		{"\U0010ffff", []byte("\xed\xaf\xbf\xed\xbf\xbf")},
		{"a\U0001f600b\U00010000c", []byte("a\xed\xa0\xbd\xed\xb8\x80b\xed\xa0\x80\xed\xb0\x80c")},
	}
	for _, c := range cases {
		got, err := Cesu8Encode(c.text)
		if err != nil {
			t.Errorf("Cesu8Encode(%q) error: %v", c.text, err)
			continue
		}
		if !bytes.Equal(got, c.expected) {
			t.Errorf("Cesu8Encode(%q) = % x, want % x", c.text, got, c.expected)
		}
	}
}

// 对应 tests/test_session_cesu8.py 的 test_cesu8_fix。
func TestCesu8Fix(t *testing.T) {
	cases := []struct {
		blob     []byte
		expected []byte
	}{
		{[]byte(""), []byte("")},
		{[]byte("ASCII"), []byte("ASCII")},
		{[]byte("\xc3\xa9\xe2\x82\xac"), []byte("\xc3\xa9\xe2\x82\xac")},
		{[]byte("\xed\xa0\x80\xed\xb0\x80"), []byte("\xf0\x90\x80\x80")},
		{[]byte("\xed\xa0\xbd\xed\xb8\x80"), []byte("\xf0\x9f\x98\x80")},
		{[]byte("\xed\xaf\xbf\xed\xbf\xbf"), []byte("\xf4\x8f\xbf\xbf")},
		{[]byte("a\xed\xa0\xbd\xed\xb8\x80b\xed\xa0\x80\xed\xb0\x80c"), []byte("a\xf0\x9f\x98\x80b\xf0\x90\x80\x80c")},
	}
	for _, c := range cases {
		got := Cesu8Fix(c.blob)
		if !bytes.Equal(got, c.expected) {
			t.Errorf("Cesu8Fix(% x) = % x, want % x", c.blob, got, c.expected)
		}
	}
}

// 对应 tests/test_session_cesu8.py 的 test_cesu8_fix_leaves_non_surrogate_pairs_unchanged。
func TestCesu8FixLeavesNonSurrogatePairsUnchanged(t *testing.T) {
	blobs := [][]byte{
		[]byte("\xed\xa0\x80"),
		[]byte("\xed\xb0\x80\xed\xa0\x80"),
		[]byte("\xed\x9f\xbf\xed\xb0\x80"),
		[]byte("\xed\xa0\x80\xed\xc0\x80"),
	}
	for _, blob := range blobs {
		if got := Cesu8Fix(blob); !bytes.Equal(got, blob) {
			t.Errorf("Cesu8Fix(% x) = % x, want unchanged", blob, got)
		}
	}
}

// 对应 tests/test_session_cesu8.py 的 test_cesu8_encode_and_fix_round_trip。
func TestCesu8EncodeAndFixRoundTrip(t *testing.T) {
	texts := []string{
		"plain text",
		"Za\u017c\u00f3\u0142\u0107 g\u0119\u015bl\u0105 ja\u017a\u0144",
		"minimum: \U00010000",
		"emoji: \U0001f600 \U0001f4f6",
		"maximum: \U0010ffff",
	}
	for _, text := range texts {
		encoded, err := Cesu8Encode(text)
		if err != nil {
			t.Errorf("Cesu8Encode(%q) error: %v", text, err)
			continue
		}
		if fixed := Cesu8Fix(encoded); string(fixed) != text {
			t.Errorf("round trip: Cesu8Fix(Cesu8Encode(%q)) = %q", text, fixed)
		}
	}
}