use crate::*;

/// Tests that false to int results in 0.
#[test]
fn false_to_int_works() {
    let locals = [];
    let blocks = [
        block!(switch_int(bool_to_int::<u8>(const_bool(false)), &[(0u8, 1)], 2)),
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// Tests that true to int results in 1.
#[test]
fn true_to_int_works() {
    let locals = [];
    let blocks = [
        block!(switch_int(bool_to_int::<u8>(const_bool(true)), &[(1u8, 1)], 2)),
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// Tests that boolean negation works.
#[test]
fn not_works_both_ways() {
    let locals = [];
    let blocks = [
        block!(if_(not(const_bool(false)), 1, 3)), // go to next block if !false
        block!(if_(not(const_bool(true)), 3, 2)),  // go to unreachable if !true
        block!(exit()),
        block!(unreachable()),
    ];

    let program = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(program);
}

/// Tests that boolean not requires a boolean operand
#[test]
fn boolean_not_requires_boolean_op() {
    let locals = &[<bool>::get_type()];
    let statements = &[storage_live(0), assign(local(0), not(const_int(0u8))), storage_dead(0)];
    let program = small_program(locals, statements);
    assert_ill_formed(program);
}

/// Tests that bool2int requires a boolean operand
#[test]
fn bool2int_requires_boolean_op() {
    let locals = &[<u8>::get_type()];
    let statements =
        &[storage_live(0), assign(local(0), bool_to_int::<u8>(const_int(0u8))), storage_dead(0)];
    let program = small_program(locals, statements);
    assert_ill_formed(program);
}

/// Test that BinOpBool::BitAnd works
#[test]
fn bit_and_bool_works() {
    let locals = [];
    let unreach_block = 5;
    let blocks = [
        // if false go to next block
        block!(if_(bool_and(const_bool(false), const_bool(false)), unreach_block, 1)),
        block!(if_(bool_and(const_bool(false), const_bool(true)), unreach_block, 2)),
        block!(if_(bool_and(const_bool(true), const_bool(false)), unreach_block, 3)),
        // if true go to next block
        block!(if_(bool_and(const_bool(true), const_bool(true)), 4, unreach_block)),
        block!(exit()),
        block!(unreachable()),
    ];
    let prog = program(&[function(Ret::No, 0, &locals, &blocks)]);
    assert_stop(prog);
}

// Test that BinOpBool::BitAnd fails with non-int/non-bool
#[test]
fn bit_and_requires_bool() {
    let locals = [<bool>::get_type()];
    let const_arr = array(&[const_int::<u8>(0); 3], <u8>::get_type());
    let b0 = block!(
        storage_live(0),
        assign(local(0), bool_and(const_arr, const_arr)),
        storage_dead(0),
        exit(),
    );
    let prog = program(&[function(Ret::No, 0, &locals, &[b0])]);
    assert_ill_formed(prog);
}

// Test that BinOpBool::BitAnd fails with int
#[test]
fn bit_and_no_bool_int_mixing() {
    let locals = [<bool>::get_type()];
    let b0 = block!(
        storage_live(0),
        assign(local(0), bool_and(const_int::<i32>(1), const_int::<i32>(0))),
        storage_dead(0),
        exit(),
    );
    let prog = program(&[function(Ret::No, 0, &locals, &[b0])]);
    assert_ill_formed(prog);
}
