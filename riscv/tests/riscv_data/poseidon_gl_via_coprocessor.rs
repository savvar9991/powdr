#![no_std]

use runtime::coprocessors::{poseidon_gl, poseidon_gl_unsafe};

#[no_mangle]
fn main() {
    let i: [u64; 12] = [0; 12];
    let h = poseidon_gl(i);
    assert_eq!(h[0], 4330397376401421145);
    assert_eq!(h[1], 14124799381142128323);
    assert_eq!(h[2], 8742572140681234676);
    assert_eq!(h[3], 14345658006221440202);

    let i: [u64; 12] = [1; 12];
    let h = poseidon_gl(i);
    assert_eq!(h[0], 16428316519797902711);
    assert_eq!(h[1], 13351830238340666928);
    assert_eq!(h[2], 682362844289978626);
    assert_eq!(h[3], 12150588177266359240);

    let minus_one = 0xffffffff00000001 - 1;
    let i: [u64; 12] = [minus_one; 12];
    let h = poseidon_gl(i);
    assert_eq!(h[0], 13691089994624172887);
    assert_eq!(h[1], 15662102337790434313);
    assert_eq!(h[2], 14940024623104903507);
    assert_eq!(h[3], 10772674582659927682);

    let i: [u64; 12] = [
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        18446744069414584321,
        0,
        0,
        0,
        0,
    ];
    let h = poseidon_gl_unsafe(i);
    assert_eq!(h[0], 4330397376401421145);
    assert_eq!(h[1], 14124799381142128323);
    assert_eq!(h[2], 8742572140681234676);
    assert_eq!(h[3], 14345658006221440202);

    let i: [u64; 12] = [
        923978,
        235763497586,
        9827635653498,
        112870,
        289273673480943876,
        230295874986745876,
        6254867324987,
        2087,
        0,
        0,
        0,
        0,
    ];
    let h = poseidon_gl(i);
    assert_eq!(h[0], 1892171027578617759);
    assert_eq!(h[1], 984732815927439256);
    assert_eq!(h[2], 7866041765487844082);
    assert_eq!(h[3], 8161503938059336191);
}
