#![allow(dead_code, unused_imports)]
// when data is bound by the same name immutably, it also freezes
// frozen data can't be modified until the immutable binding goes out of scope
pub fn de_freezing() {
    let mut _mutable_integer = 7i32;

    {
        // shadowing by immutable ing `_mutable_integer`
        let _mutable_integer = _mutable_integer;

        // error `_mutable_integer` is frozen in this scope
        // _mutable_integer = 50;
        // ^uncomment to see compiler time error

        // `_mutable_integer` goes out of scope
    }
    // ok `_mutable_integer` is not frozen in this scope
    _mutable_integer = 3;
    println!("mutable integer {}", _mutable_integer)
}