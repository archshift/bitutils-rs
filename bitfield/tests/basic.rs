extern crate bf_impl;
use bf_impl::bf;

bf!(Hello[u8] {
    foo: 1:4,
    bar: 0:3,
    foobar: 3:7
});

#[test]
fn get_set_upd() {
    let mut _hello = Hello::new(0b10101100);
    assert_eq!(_hello.foo(), 0b0110);
    assert_eq!(_hello.bar(), 0b1100);
    assert_eq!(_hello.foobar(), 0b10101);

    _hello.set_foo(0)
        .set_bar(0);
    assert_eq!(_hello.bar(), 0);

    _hello.upd_foo(|x| x + 2)
        .upd_bar(|x| x * 3);
    assert_eq!(_hello.foo(), 3 << 1);
}

bf!(TestField[u8] {
    bottom: 0:5,
    top: 6:7,
});

#[test]
fn alias() {
    let mut val = 0b10100000;
    {
        let bf = TestField::alias(&val);
        assert_eq!(bf.top(), 0b10);
    }
    let bf = TestField::alias_mut(&mut val);
    bf.set_top(0b11);
    assert_eq!(bf.val, 0b11100000);
}

#[test]
fn formatting() {
    let out = format!("{:x?}", TestField::new(!0));
    assert_eq!(out, "TestField { bottom: 3f, top: 3 }");
}
