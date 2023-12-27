use super::super::*;
use dsntk_feel::scope;

#[test]
fn _0001() {
  te_number(false, &scope!(), "round down(5.0, 0)", 5, 0);
}

#[test]
fn _0002() {
  te_number(false, &scope!(), "round down(5.1, 0)", 5, 0);
}

#[test]
fn _0003() {
  te_number(false, &scope!(), "round down(5.5, 0)", 5, 0);
}

#[test]
fn _0004() {
  te_number(false, &scope!(), "round down(5.6, 0)", 5, 0);
}

#[test]
fn _0005() {
  te_number(false, &scope!(), "round down(5.9, 0)", 5, 0);
}

#[test]
fn _0006() {
  te_number(false, &scope!(), "round down(-5.0, 0)", -5, 0);
}

#[test]
fn _0007() {
  te_number(false, &scope!(), "round down(-5.1, 0)", -5, 0);
}

#[test]
fn _0008() {
  te_number(false, &scope!(), "round down(-5.5, 0)", -5, 0);
}

#[test]
fn _0009() {
  te_number(false, &scope!(), "round down(-5.6, 0)", -5, 0);
}

#[test]
fn _0010() {
  te_number(false, &scope!(), "round down(-5.9, 0)", -5, 0);
}

#[test]
fn _0011() {
  te_number(false, &scope!(), "round down(1.121, 2)", 112, 2);
}

#[test]
fn _0012() {
  te_number(false, &scope!(), "round down(1.126, 2)", 112, 2);
}

#[test]
fn _0013() {
  te_number(false, &scope!(), "round down(-1.121, 2)", -112, 2);
}

#[test]
fn _0014() {
  te_number(false, &scope!(), "round down(-1.126, 2)", -112, 2);
}

#[test]
#[ignore]
fn _0015() {
  //--------------------------------------------------------------------------------------------------------------------
  // Used maximum allowed scale.
  // Currently get this: (signal: 11, SIGSEGV: invalid memory reference)
  //--------------------------------------------------------------------------------------------------------------------
  te_number(false, &scope!(), "round down(5.5, 6176)", 55, 1);
}

#[test]
fn _0016() {
  //--------------------------------------------------------------------------------------------------------------------
  // Scale is out of range.
  //--------------------------------------------------------------------------------------------------------------------
  te_null(
    false,
    &scope!(),
    "round down(5.5, (6176 + 1))",
    "[core::round_down] scale is out of range -6111..6176: 6177",
  );
}