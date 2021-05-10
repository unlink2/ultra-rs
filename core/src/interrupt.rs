// $12 == $status
#[naked]
pub unsafe extern "C" fn disable_int() {
    asm!(r#"
        .set noat
        mfc0 $t0, $12
        addiu $at, $zero, 0xFFFE
        and $t1, $t0, $at
        mtc0 $t1, $12
        andi $v0, $t0, 0x1
        nop
        jr $ra
        nop
    "#,
    options(noreturn));
}

#[naked]
pub unsafe extern "C" fn enable_int() {
    asm!(r#"
        .set noat
        addiu $a0, $zero, 1
        mfc0 $t0, $12
        or $t0, $t0, $a0
        mtc0 $t0, $12
        nop
        nop
        jr $ra
        nop
    "#,
    options(noreturn));
}
