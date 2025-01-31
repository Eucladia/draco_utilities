use draco_utilities::base64::{decode_base64, encode_base64};

#[test]
fn encode() {
  let expected = "b25lIHR3byB0aHJlZSBmb3VyIGZpdmU="; //"one two three four five";
  let mut encoded = Vec::new();

  encode_base64(b"one two three four five", &mut encoded);
  let result = std::str::from_utf8(&encoded).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn encode_moby_dick() {
  let expected = "Q2FsbCBtZSBJc2htYWVsLiBTb21lIHllYXJzIGFnby0tbmV2ZXIgbWluZCBob3cgbG9uZ\
                yBwcmVjaXNlbHktLWhhdmluZwpsaXR0bGUgb3Igbm8gbW9uZXkgaW4gbXkgcHVyc2UsIG\
                FuZCBub3RoaW5nIHBhcnRpY3VsYXIgdG8gaW50ZXJlc3QgbWUgb24Kc2hvcmUsIEkgdGh\
                vdWdodCBJIHdvdWxkIHNhaWwgYWJvdXQgYSBsaXR0bGUgYW5kIHNlZSB0aGUgd2F0ZXJ5\
                IHBhcnQgb2YKdGhlIHdvcmxkLiBJdCBpcyBhIHdheSBJIGhhdmUgb2YgZHJpdmluZyBvZ\
                mYgdGhlIHNwbGVlbiBhbmQgcmVndWxhdGluZwp0aGUgY2lyY3VsYXRpb24uIFdoZW5ldm\
                VyIEkgZmluZCBteXNlbGYgZ3Jvd2luZyBncmltIGFib3V0IHRoZSBtb3V0aDsKd2hlbmV\
                2ZXIgaXQgaXMgYSBkYW1wLCBkcml6emx5IE5vdmVtYmVyIGluIG15IHNvdWw7IHdoZW5l\
                dmVyIEkgZmluZApteXNlbGYgaW52b2x1bnRhcmlseSBwYXVzaW5nIGJlZm9yZSBjb2Zma\
                W4gd2FyZWhvdXNlcywgYW5kIGJyaW5naW5nIHVwCnRoZSByZWFyIG9mIGV2ZXJ5IGZ1bm\
                VyYWwgSSBtZWV0OyBhbmQgZXNwZWNpYWxseSB3aGVuZXZlciBteSBoeXBvcyBnZXQKc3V\
                jaCBhbiB1cHBlciBoYW5kIG9mIG1lLCB0aGF0IGl0IHJlcXVpcmVzIGEgc3Ryb25nIG1v\
                cmFsIHByaW5jaXBsZSB0bwpwcmV2ZW50IG1lIGZyb20gZGVsaWJlcmF0ZWx5IHN0ZXBwa\
                W5nIGludG8gdGhlIHN0cmVldCwgYW5kIG1ldGhvZGljYWxseQprbm9ja2luZyBwZW9wbG\
                UncyBoYXRzIG9mZi0tdGhlbiwgSSBhY2NvdW50IGl0IGhpZ2ggdGltZSB0byBnZXQgdG8\
                gc2VhCmFzIHNvb24gYXMgSSBjYW4uIFRoaXMgaXMgbXkgc3Vic3RpdHV0ZSBmb3IgcGlz\
                dG9sIGFuZCBiYWxsLiBXaXRoIGEKcGhpbG9zb3BoaWNhbCBmbG91cmlzaCBDYXRvIHRoc\
                m93cyBoaW1zZWxmIHVwb24gaGlzIHN3b3JkOyBJIHF1aWV0bHkKdGFrZSB0byB0aGUgc2\
                hpcC4gVGhlcmUgaXMgbm90aGluZyBzdXJwcmlzaW5nIGluIHRoaXMuIElmIHRoZXkgYnV\
                0IGtuZXcKaXQsIGFsbW9zdCBhbGwgbWVuIGluIHRoZWlyIGRlZ3JlZSwgc29tZSB0aW1l\
                IG9yIG90aGVyLCBjaGVyaXNoIHZlcnkKbmVhcmx5IHRoZSBzYW1lIGZlZWxpbmdzIHRvd\
                2FyZHMgdGhlIG9jZWFuIHdpdGggbWUuCg==";

  let mut encoded = Vec::new();

  encode_base64(
    b"Call me Ishmael. Some years ago--never mind how long precisely--having\n\
  little or no money in my purse, and nothing particular to interest me on\n\
  shore, I thought I would sail about a little and see the watery part of\n\
  the world. It is a way I have of driving off the spleen and regulating\n\
  the circulation. Whenever I find myself growing grim about the mouth;\n\
  whenever it is a damp, drizzly November in my soul; whenever I find\n\
  myself involuntarily pausing before coffin warehouses, and bringing up\n\
  the rear of every funeral I meet; and especially whenever my hypos get\n\
  such an upper hand of me, that it requires a strong moral principle to\n\
  prevent me from deliberately stepping into the street, and methodically\n\
  knocking people's hats off--then, I account it high time to get to sea\n\
  as soon as I can. This is my substitute for pistol and ball. With a\n\
  philosophical flourish Cato throws himself upon his sword; I quietly\n\
  take to the ship. There is nothing surprising in this. If they but knew\n\
  it, almost all men in their degree, some time or other, cherish very\n\
  nearly the same feelings towards the ocean with me.\n",
    &mut encoded,
  );
  let result = std::str::from_utf8(&encoded).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn decode() {
  let expected = "one two three four five";
  let mut decoded = Vec::new();

  decode_base64(b"b25lIHR3byB0aHJlZSBmb3VyIGZpdmU=", &mut decoded).unwrap();
  let result = std::str::from_utf8(&decoded).unwrap();
  assert_eq!(result, expected);
}

#[test]
fn decode_moby_dick() {
  let expected = "Call me Ishmael. Some years ago--never mind how long precisely--having\n\
  little or no money in my purse, and nothing particular to interest me on\n\
  shore, I thought I would sail about a little and see the watery part of\n\
  the world. It is a way I have of driving off the spleen and regulating\n\
  the circulation. Whenever I find myself growing grim about the mouth;\n\
  whenever it is a damp, drizzly November in my soul; whenever I find\n\
  myself involuntarily pausing before coffin warehouses, and bringing up\n\
  the rear of every funeral I meet; and especially whenever my hypos get\n\
  such an upper hand of me, that it requires a strong moral principle to\n\
  prevent me from deliberately stepping into the street, and methodically\n\
  knocking people's hats off--then, I account it high time to get to sea\n\
  as soon as I can. This is my substitute for pistol and ball. With a\n\
  philosophical flourish Cato throws himself upon his sword; I quietly\n\
  take to the ship. There is nothing surprising in this. If they but knew\n\
  it, almost all men in their degree, some time or other, cherish very\n\
  nearly the same feelings towards the ocean with me.\n";

  let mut decoded = Vec::new();

  decode_base64(
    b"Q2FsbCBtZSBJc2htYWVsLiBTb21lIHllYXJzIGFnby0tbmV2ZXIgbWluZCBob3cgbG9uZ\
                yBwcmVjaXNlbHktLWhhdmluZwpsaXR0bGUgb3Igbm8gbW9uZXkgaW4gbXkgcHVyc2UsIG\
                FuZCBub3RoaW5nIHBhcnRpY3VsYXIgdG8gaW50ZXJlc3QgbWUgb24Kc2hvcmUsIEkgdGh\
                vdWdodCBJIHdvdWxkIHNhaWwgYWJvdXQgYSBsaXR0bGUgYW5kIHNlZSB0aGUgd2F0ZXJ5\
                IHBhcnQgb2YKdGhlIHdvcmxkLiBJdCBpcyBhIHdheSBJIGhhdmUgb2YgZHJpdmluZyBvZ\
                mYgdGhlIHNwbGVlbiBhbmQgcmVndWxhdGluZwp0aGUgY2lyY3VsYXRpb24uIFdoZW5ldm\
                VyIEkgZmluZCBteXNlbGYgZ3Jvd2luZyBncmltIGFib3V0IHRoZSBtb3V0aDsKd2hlbmV\
                2ZXIgaXQgaXMgYSBkYW1wLCBkcml6emx5IE5vdmVtYmVyIGluIG15IHNvdWw7IHdoZW5l\
                dmVyIEkgZmluZApteXNlbGYgaW52b2x1bnRhcmlseSBwYXVzaW5nIGJlZm9yZSBjb2Zma\
                W4gd2FyZWhvdXNlcywgYW5kIGJyaW5naW5nIHVwCnRoZSByZWFyIG9mIGV2ZXJ5IGZ1bm\
                VyYWwgSSBtZWV0OyBhbmQgZXNwZWNpYWxseSB3aGVuZXZlciBteSBoeXBvcyBnZXQKc3V\
                jaCBhbiB1cHBlciBoYW5kIG9mIG1lLCB0aGF0IGl0IHJlcXVpcmVzIGEgc3Ryb25nIG1v\
                cmFsIHByaW5jaXBsZSB0bwpwcmV2ZW50IG1lIGZyb20gZGVsaWJlcmF0ZWx5IHN0ZXBwa\
                W5nIGludG8gdGhlIHN0cmVldCwgYW5kIG1ldGhvZGljYWxseQprbm9ja2luZyBwZW9wbG\
                UncyBoYXRzIG9mZi0tdGhlbiwgSSBhY2NvdW50IGl0IGhpZ2ggdGltZSB0byBnZXQgdG8\
                gc2VhCmFzIHNvb24gYXMgSSBjYW4uIFRoaXMgaXMgbXkgc3Vic3RpdHV0ZSBmb3IgcGlz\
                dG9sIGFuZCBiYWxsLiBXaXRoIGEKcGhpbG9zb3BoaWNhbCBmbG91cmlzaCBDYXRvIHRoc\
                m93cyBoaW1zZWxmIHVwb24gaGlzIHN3b3JkOyBJIHF1aWV0bHkKdGFrZSB0byB0aGUgc2\
                hpcC4gVGhlcmUgaXMgbm90aGluZyBzdXJwcmlzaW5nIGluIHRoaXMuIElmIHRoZXkgYnV\
                0IGtuZXcKaXQsIGFsbW9zdCBhbGwgbWVuIGluIHRoZWlyIGRlZ3JlZSwgc29tZSB0aW1l\
                IG9yIG90aGVyLCBjaGVyaXNoIHZlcnkKbmVhcmx5IHRoZSBzYW1lIGZlZWxpbmdzIHRvd\
                2FyZHMgdGhlIG9jZWFuIHdpdGggbWUuCg==",
    &mut decoded,
  )
  .unwrap();
  let result = std::str::from_utf8(&decoded).unwrap();
  assert_eq!(result, expected);
}
