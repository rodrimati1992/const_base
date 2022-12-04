use crate::ArrayStr;

#[test]
fn from_utf8_test() {
    ArrayStr::from_utf8([0xFF; 4]).unwrap_err();

    assert_eq!(ArrayStr::from_utf8(*b"hello").unwrap(), "hello")
}

#[test]
fn from_utf8_unwrap_ok_test() {
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"hello"), "hello")
}

#[test]
#[should_panic]
fn from_utf8_unwrap_err_test() {
    ArrayStr::from_utf8_unwrap([0xFF; 4]);
}

#[test]
fn from_utf8_unchecked_test() {
    unsafe { assert_eq!(ArrayStr::from_utf8_unchecked(*b"hello"), "hello") }
}

#[test]
fn len_test() {
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"").len(), 0);
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"w").len(), 1);
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"wo").len(), 2);
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"wor").len(), 3);
    assert_eq!(ArrayStr::from_utf8_unwrap(*b"worl").len(), 4);
}

#[test]
fn accessors_test() {
    static AS: ArrayStr<5> = ArrayStr::from_utf8_unwrap(*b"world");

    {
        let str: &'static str = AS.as_str();
        assert_eq!(str, "world");
    }
    {
        let arr: &'static [u8; 5] = AS.as_array();
        assert_eq!(arr, b"world");
    }
    {
        let arr: [u8; 5] = AS.into_array();
        assert_eq!(arr, *b"world");
    }
}
