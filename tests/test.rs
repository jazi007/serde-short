use cdr::{CdrBe, Infinite};
use serde_repr::{Deserialize_repr, Serialize_repr};
use serde_short::{Deserialize_short, Serialize_short};

#[test]
fn test_short() {
    #[derive(Serialize_short, Deserialize_short, PartialEq, Debug)]
    #[repr(C)]
    enum Test {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(u8)]
    enum Testu8 {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
    #[derive(Serialize_repr, Deserialize_repr, PartialEq, Debug)]
    #[repr(u16)]
    enum Testu16 {
        Two = 2,
        Three = 3,
        Five = 5,
        Seven = 7,
    }
    let v1 = cdr::serialize::<_, _, CdrBe>(&Testu8::Two, Infinite).unwrap();
    let v2 = cdr::serialize::<_, _, CdrBe>(&Test::Two, Infinite).unwrap();
    let v3 = cdr::serialize::<_, _, CdrBe>(&Testu16::Two, Infinite).unwrap();
    assert_eq!(v1, v2);
    assert_ne!(v1, v3);
    let decoded = cdr::deserialize::<Test>(&v2).unwrap();
    assert_eq!(decoded, Test::Two);
}
