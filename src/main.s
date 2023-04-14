	.section	__TEXT,__text,regular,pure_instructions
	.build_version macos, 11, 0
	.p2align	2
__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc88e1babb8493d9dE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	bl	__ZN4core3ops8function6FnOnce9call_once17h2fca31165e6c9e2bE
	; InlineAsm Start
	; InlineAsm End
	b	LBB0_1
LBB0_1:
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.private_extern	__ZN3std2rt10lang_start17h4a203ef9f539b20aE
	.globl	__ZN3std2rt10lang_start17h4a203ef9f539b20aE
	.p2align	2
__ZN3std2rt10lang_start17h4a203ef9f539b20aE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	str	x1, [sp]
	mov	x0, x2
	ldr	x2, [sp]
	str	x0, [sp, #8]
	mov	x4, x3
	ldr	x3, [sp, #8]
	sub	x0, x29, #8
	stur	x8, [x29, #-8]
	adrp	x1, l___unnamed_1@PAGE
	add	x1, x1, l___unnamed_1@PAGEOFF
	bl	__ZN3std2rt19lang_start_internal17h9f0566e553deb11eE
	str	x0, [sp, #16]
	ldr	x0, [sp, #16]
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h0d043447c794ac8aE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN3std10sys_common9backtrace28__rust_begin_short_backtrace17hc88e1babb8493d9dE
	bl	__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17heae5b33b2ef97879E
	sturb	w0, [x29, #-1]
	ldurb	w0, [x29, #-1]
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6cd58fa43dc80064E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	mov	x2, x1
	ldr	x0, [x8]
	ldr	x1, [x8, #8]
	bl	__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17h20d2c7c2ea4e15cdE
	and	w0, w0, #0x1
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h893f704e0973e266E:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h35e563dddea1a503E
	and	w0, w0, #0x1
	.cfi_def_cfa wsp, 16
	ldp	x29, x30, [sp], #16
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN48_$LT$$u5b$T$u5d$$u20$as$u20$core..fmt..Debug$GT$3fmt17h20d2c7c2ea4e15cdE:
	.cfi_startproc
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x0, [sp]
	str	x1, [sp, #8]
	mov	x0, x2
	sub	x8, x29, #16
	str	x8, [sp, #16]
	bl	__ZN4core3fmt9Formatter10debug_list17h4545a930d4bf5d49E
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	bl	__ZN4core5slice4iter13Iter$LT$T$GT$3new17h345dd056797203afE
	mov	x2, x0
	ldr	x0, [sp, #16]
	str	x2, [sp, #24]
	mov	x2, x1
	ldr	x1, [sp, #24]
	bl	__ZN4core3fmt8builders9DebugList7entries17h70e7f52f956723edE
	bl	__ZN4core3fmt8builders9DebugList6finish17hff355a1409bd14a2E
	and	w0, w0, #0x1
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt10ArgumentV19new_debug17h97b621b92b4d083aE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	adrp	x8, __ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h31a1c10ee8d82ebcE@PAGE
	add	x8, x8, __ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h31a1c10ee8d82ebcE@PAGEOFF
	str	x8, [sp, #16]
	ldr	x8, [sp, #16]
	str	x0, [sp, #24]
	ldr	x9, [sp, #24]
	str	x9, [sp]
	str	x8, [sp, #8]
	ldr	x0, [sp]
	ldr	x1, [sp, #8]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3fmt3num49_$LT$impl$u20$core..fmt..Debug$u20$for$u20$u8$GT$3fmt17h35e563dddea1a503E:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	mov	x0, x1
	str	x0, [sp, #16]
	bl	__ZN4core3fmt9Formatter15debug_lower_hex17h048772b104e69e74E
	tbnz	w0, #0, LBB7_2
	b	LBB7_1
LBB7_1:
	ldr	x0, [sp, #16]
	bl	__ZN4core3fmt9Formatter15debug_upper_hex17hca088ccaf9ff1030E
	tbnz	w0, #0, LBB7_5
	b	LBB7_4
LBB7_2:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..LowerHex$u20$for$u20$u8$GT$3fmt17hdb0ff1e71a83285aE
	and	w8, w0, #0x1
	sturb	w8, [x29, #-1]
	b	LBB7_3
LBB7_3:
	ldurb	w8, [x29, #-1]
	and	w0, w8, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB7_4:
	.cfi_restore_state
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$u8$GT$3fmt17hf0c0261afa5b9c72E
	and	w8, w0, #0x1
	sturb	w8, [x29, #-1]
	b	LBB7_6
LBB7_5:
	ldr	x1, [sp, #16]
	ldr	x0, [sp, #8]
	bl	__ZN4core3fmt3num52_$LT$impl$u20$core..fmt..UpperHex$u20$for$u20$u8$GT$3fmt17h51ab49533878399fE
	and	w8, w0, #0x1
	sturb	w8, [x29, #-1]
	b	LBB7_6
LBB7_6:
	b	LBB7_3
	.cfi_endproc

	.p2align	2
__ZN4core3fmt8builders9DebugList7entries17h70e7f52f956723edE:
Lfunc_begin0:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception0
	sub	sp, sp, #80
	.cfi_def_cfa_offset 80
	stp	x29, x30, [sp, #64]
	add	x29, sp, #64
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x0, [sp, #8]
	mov	x0, x1
	mov	x1, x2
	bl	__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6ae7cc2b390ddf0dE
	str	x0, [sp, #16]
	str	x1, [sp, #24]
	b	LBB8_1
LBB8_1:
Ltmp1:
	add	x0, sp, #16
	bl	__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc68930e85e432b78E
	str	x0, [sp]
Ltmp2:
	b	LBB8_4
LBB8_2:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB8_3:
Ltmp3:
	mov	x8, x1
	stur	x0, [x29, #-16]
	stur	w8, [x29, #-8]
	b	LBB8_2
LBB8_4:
	ldr	x8, [sp]
	str	x8, [sp, #32]
	ldr	x8, [sp, #32]
	subs	x8, x8, #0
	cset	w8, eq
	and	w8, w8, #0x1
	ands	w8, w8, #0x1
	cset	x8, eq
	subs	x8, x8, #0
	cset	w8, ne
	tbnz	w8, #0, LBB8_6
	b	LBB8_5
LBB8_5:
	ldr	x0, [sp, #8]
	.cfi_def_cfa wsp, 80
	ldp	x29, x30, [sp, #64]
	add	sp, sp, #80
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB8_6:
	.cfi_restore_state
	ldr	x0, [sp, #8]
	ldr	x8, [sp, #32]
	sub	x1, x29, #24
	stur	x8, [x29, #-24]
Ltmp4:
	adrp	x2, l___unnamed_2@PAGE
	add	x2, x2, l___unnamed_2@PAGEOFF
	bl	__ZN4core3fmt8builders9DebugList5entry17h8599d6df117285a4E
Ltmp5:
	b	LBB8_9
LBB8_7:
	b	LBB8_2
LBB8_8:
Ltmp6:
	mov	x8, x1
	stur	x0, [x29, #-16]
	stur	w8, [x29, #-8]
	b	LBB8_7
LBB8_9:
	b	LBB8_10
LBB8_10:
	b	LBB8_1
Lfunc_end0:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table8:
Lexception0:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end0-Lcst_begin0
Lcst_begin0:
	.uleb128 Lfunc_begin0-Lfunc_begin0
	.uleb128 Ltmp1-Lfunc_begin0
	.byte	0
	.byte	0
	.uleb128 Ltmp1-Lfunc_begin0
	.uleb128 Ltmp2-Ltmp1
	.uleb128 Ltmp3-Lfunc_begin0
	.byte	0
	.uleb128 Ltmp2-Lfunc_begin0
	.uleb128 Ltmp4-Ltmp2
	.byte	0
	.byte	0
	.uleb128 Ltmp4-Lfunc_begin0
	.uleb128 Ltmp5-Ltmp4
	.uleb128 Ltmp6-Lfunc_begin0
	.byte	0
Lcst_end0:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3fmt9Arguments6new_v117h2c6ceec4b15501efE:
	.cfi_startproc
	sub	sp, sp, #144
	.cfi_def_cfa_offset 144
	stp	x29, x30, [sp, #128]
	add	x29, sp, #128
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	.cfi_remember_state
	str	x8, [sp, #16]
	str	x0, [sp, #24]
	str	x1, [sp, #32]
	str	x2, [sp, #40]
	str	x3, [sp, #48]
	subs	x8, x1, x3
	cset	w8, lo
	tbnz	w8, #0, LBB9_2
	b	LBB9_1
LBB9_1:
	ldr	x8, [sp, #32]
	ldr	x9, [sp, #48]
	add	x9, x9, #1
	subs	x8, x8, x9
	cset	w8, hi
	and	w8, w8, #0x1
	strb	w8, [sp, #63]
	b	LBB9_3
LBB9_2:
	mov	w8, #1
	strb	w8, [sp, #63]
	b	LBB9_3
LBB9_3:
	ldrb	w8, [sp, #63]
	tbnz	w8, #0, LBB9_5
	b	LBB9_4
LBB9_4:
	ldr	x8, [sp, #48]
	ldr	x9, [sp, #16]
	ldr	x10, [sp, #40]
	ldr	x11, [sp, #32]
	ldr	x12, [sp, #24]
	stur	xzr, [x29, #-16]
	str	x12, [x9, #16]
	str	x11, [x9, #24]
	ldur	x12, [x29, #-16]
	ldur	x11, [x29, #-8]
	str	x12, [x9]
	str	x11, [x9, #8]
	str	x10, [x9, #32]
	str	x8, [x9, #40]
	.cfi_def_cfa wsp, 144
	ldp	x29, x30, [sp, #128]
	add	sp, sp, #144
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
LBB9_5:
	.cfi_restore_state
	add	x8, sp, #64
	str	x8, [sp, #8]
	adrp	x0, l___unnamed_3@PAGE
	add	x0, x0, l___unnamed_3@PAGEOFF
	mov	w9, #1
	mov	x1, x9
	adrp	x2, l___unnamed_4@PAGE
	add	x2, x2, l___unnamed_4@PAGEOFF
	mov	x3, #0
	bl	__ZN4core3fmt9Arguments6new_v117h2c6ceec4b15501efE
	ldr	x0, [sp, #8]
	adrp	x1, l___unnamed_5@PAGE
	add	x1, x1, l___unnamed_5@PAGEOFF
	bl	__ZN4core9panicking9panic_fmt17hc1e7b11add95109dE
	.cfi_endproc

	.p2align	2
__ZN4core3num21_$LT$impl$u20$u64$GT$11to_be_bytes17hc989148b57e9db55E:
	.cfi_startproc
	sub	sp, sp, #16
	.cfi_def_cfa_offset 16
	rev	x8, x0
	str	x8, [sp, #8]
	ldr	x8, [sp, #8]
	str	x8, [sp]
	ldr	x0, [sp]
	add	sp, sp, #16
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hcbce53e150ddaf25E:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	ldr	x0, [x0]
	bl	__ZN4core3ops8function6FnOnce9call_once17hcb1d02ec49282d5dE
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17h2fca31165e6c9e2bE:
	.cfi_startproc
	sub	sp, sp, #32
	.cfi_def_cfa_offset 32
	stp	x29, x30, [sp, #16]
	add	x29, sp, #16
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	blr	x0
	.cfi_def_cfa wsp, 32
	ldp	x29, x30, [sp, #16]
	add	sp, sp, #32
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ops8function6FnOnce9call_once17hcb1d02ec49282d5dE:
Lfunc_begin1:
	.cfi_startproc
	.cfi_personality 155, _rust_eh_personality
	.cfi_lsda 16, Lexception1
	sub	sp, sp, #64
	.cfi_def_cfa_offset 64
	stp	x29, x30, [sp, #48]
	add	x29, sp, #48
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x8, x0
	add	x0, sp, #16
	str	x8, [sp, #16]
Ltmp8:
	bl	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h0d043447c794ac8aE
	str	w0, [sp, #12]
Ltmp9:
	b	LBB13_3
LBB13_1:
	ldur	x0, [x29, #-16]
	bl	__Unwind_Resume
LBB13_2:
Ltmp10:
	mov	x8, x1
	stur	x0, [x29, #-16]
	stur	w8, [x29, #-8]
	b	LBB13_1
LBB13_3:
	ldr	w0, [sp, #12]
	.cfi_def_cfa wsp, 64
	ldp	x29, x30, [sp, #48]
	add	sp, sp, #64
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
Lfunc_end1:
	.cfi_endproc
	.section	__TEXT,__gcc_except_tab
	.p2align	2
GCC_except_table13:
Lexception1:
	.byte	255
	.byte	255
	.byte	1
	.uleb128 Lcst_end1-Lcst_begin1
Lcst_begin1:
	.uleb128 Ltmp8-Lfunc_begin1
	.uleb128 Ltmp9-Ltmp8
	.uleb128 Ltmp10-Lfunc_begin1
	.byte	0
	.uleb128 Ltmp9-Lfunc_begin1
	.uleb128 Lfunc_end1-Ltmp9
	.byte	0
	.byte	0
Lcst_end1:
	.p2align	2

	.section	__TEXT,__text,regular,pure_instructions
	.p2align	2
__ZN4core3ptr27drop_in_place$LT$$RF$u8$GT$17hb491c0b8636fbe73E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hbce990d7c3914740E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5array69_$LT$impl$u20$core..fmt..Debug$u20$for$u20$$u5b$T$u3b$$u20$N$u5d$$GT$3fmt17h31a1c10ee8d82ebcE:
	.cfi_startproc
	sub	sp, sp, #48
	.cfi_def_cfa_offset 48
	stp	x29, x30, [sp, #32]
	add	x29, sp, #32
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	str	x1, [sp, #8]
	mov	w8, #8
	mov	x1, x8
	adrp	x2, l___unnamed_6@PAGE
	add	x2, x2, l___unnamed_6@PAGEOFF
	bl	__ZN97_$LT$core..ops..range..RangeFull$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h25c028a863345f79E
	mov	x9, x0
	mov	x8, x1
	ldr	x1, [sp, #8]
	add	x0, sp, #16
	str	x9, [sp, #16]
	str	x8, [sp, #24]
	bl	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h6cd58fa43dc80064E
	and	w0, w0, #0x1
	.cfi_def_cfa wsp, 48
	ldp	x29, x30, [sp, #32]
	add	sp, sp, #48
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.p2align	2
__ZN4core5slice4iter13Iter$LT$T$GT$3new17h345dd056797203afE:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	str	x0, [sp, #8]
	str	x1, [sp, #16]
	str	x0, [sp, #56]
	ldr	x8, [sp, #56]
	str	x8, [sp, #80]
	mov	w8, #0
	tbnz	w8, #0, LBB17_2
	b	LBB17_1
LBB17_1:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	add	x8, x8, x9
	str	x8, [sp, #40]
	b	LBB17_3
LBB17_2:
	ldr	x8, [sp, #8]
	ldr	x9, [sp, #16]
	add	x8, x8, x9
	str	x8, [sp, #88]
	ldr	x8, [sp, #88]
	str	x8, [sp, #72]
	ldr	x8, [sp, #72]
	str	x8, [sp, #64]
	ldr	x8, [sp, #64]
	str	x8, [sp, #40]
	b	LBB17_3
LBB17_3:
	ldr	x8, [sp, #8]
	str	x8, [sp, #48]
	ldr	x8, [sp, #40]
	ldr	x9, [sp, #48]
	str	x9, [sp, #32]
	str	x8, [sp, #24]
	ldr	x0, [sp, #24]
	ldr	x1, [sp, #32]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc

	.p2align	2
__ZN54_$LT$$LP$$RP$$u20$as$u20$std..process..Termination$GT$6report17heae5b33b2ef97879E:
	.cfi_startproc
	mov	w0, #0
	ret
	.cfi_endproc

	.p2align	2
__ZN63_$LT$I$u20$as$u20$core..iter..traits..collect..IntoIterator$GT$9into_iter17h6ae7cc2b390ddf0dE:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN91_$LT$core..slice..iter..Iter$LT$T$GT$$u20$as$u20$core..iter..traits..iterator..Iterator$GT$4next17hc68930e85e432b78E:
	.cfi_startproc
	sub	sp, sp, #96
	.cfi_def_cfa_offset 96
	.cfi_remember_state
	str	x0, [sp]
	ldr	x8, [x0, #8]
	str	x8, [sp, #24]
	ldr	x8, [sp, #24]
	str	x8, [sp, #64]
	mov	w8, #1
	tbnz	w8, #0, LBB20_2
	b	LBB20_1
LBB20_1:
	ldr	x9, [sp]
	ldr	x8, [x9, #8]
	ldr	x9, [x9]
	subs	x8, x8, x9
	cset	w8, eq
	tbnz	w8, #0, LBB20_4
	b	LBB20_3
LBB20_2:
	ldr	x8, [sp]
	ldr	x8, [x8]
	str	x8, [sp, #32]
	ldr	x8, [sp, #32]
	str	x8, [sp, #72]
	b	LBB20_1
LBB20_3:
	mov	w8, #0
	tbnz	w8, #0, LBB20_6
	b	LBB20_7
LBB20_4:
	str	xzr, [sp, #8]
	b	LBB20_5
LBB20_5:
	ldr	x0, [sp, #8]
	add	sp, sp, #96
	.cfi_def_cfa_offset 0
	ret
LBB20_6:
	.cfi_restore_state
	ldr	x8, [sp]
	ldr	x9, [x8]
	subs	x9, x9, #1
	str	x9, [sp, #88]
	ldr	x9, [sp, #88]
	str	x9, [sp, #56]
	ldr	x9, [sp, #56]
	str	x9, [sp, #48]
	ldr	x9, [sp, #48]
	str	x9, [x8]
	ldr	x8, [x8, #8]
	str	x8, [sp, #16]
	b	LBB20_8
LBB20_7:
	ldr	x10, [sp]
	ldr	x8, [x10, #8]
	ldr	x9, [x10, #8]
	add	x9, x9, #1
	str	x9, [sp, #80]
	ldr	x9, [sp, #80]
	str	x9, [sp, #40]
	ldr	x9, [sp, #40]
	str	x9, [x10, #8]
	str	x8, [sp, #16]
	b	LBB20_8
LBB20_8:
	ldr	x8, [sp, #16]
	str	x8, [sp, #8]
	b	LBB20_5
	.cfi_endproc

	.p2align	2
__ZN97_$LT$core..ops..range..RangeFull$u20$as$u20$core..slice..index..SliceIndex$LT$$u5b$T$u5d$$GT$$GT$5index17h25c028a863345f79E:
	.cfi_startproc
	ret
	.cfi_endproc

	.p2align	2
__ZN4main4main17hd6b17885bda823f5E:
	.cfi_startproc
	sub	sp, sp, #112
	.cfi_def_cfa_offset 112
	stp	x29, x30, [sp, #96]
	add	x29, sp, #96
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	stur	xzr, [x29, #-8]
	ldur	x0, [x29, #-8]
	bl	__ZN4core3num21_$LT$impl$u20$u64$GT$11to_be_bytes17hc989148b57e9db55E
	stur	x0, [x29, #-16]
	ldur	x8, [x29, #-16]
	sub	x0, x29, #24
	stur	x8, [x29, #-24]
	bl	__ZN4core3fmt10ArgumentV19new_debug17h97b621b92b4d083aE
	sub	x2, x29, #40
	stur	x0, [x29, #-40]
	stur	x1, [x29, #-32]
	add	x8, sp, #8
	str	x8, [sp]
	adrp	x0, l___unnamed_7@PAGE
	add	x0, x0, l___unnamed_7@PAGEOFF
	mov	w9, #2
	mov	x1, x9
	mov	w9, #1
	mov	x3, x9
	bl	__ZN4core3fmt9Arguments6new_v117h2c6ceec4b15501efE
	ldr	x0, [sp]
	bl	__ZN3std2io5stdio6_print17hf5189a9887145206E
	.cfi_def_cfa wsp, 112
	ldp	x29, x30, [sp, #96]
	add	sp, sp, #112
	.cfi_def_cfa_offset 0
	.cfi_restore w30
	.cfi_restore w29
	ret
	.cfi_endproc

	.globl	_main
	.p2align	2
_main:
	.cfi_startproc
	stp	x29, x30, [sp, #-16]!
	.cfi_def_cfa_offset 16
	mov	x29, sp
	.cfi_def_cfa w29, 16
	.cfi_offset w30, -8
	.cfi_offset w29, -16
	mov	x2, x1
	mov	x8, x0
	sxtw	x1, w8
	adrp	x0, __ZN4main4main17hd6b17885bda823f5E@PAGE
	add	x0, x0, __ZN4main4main17hd6b17885bda823f5E@PAGEOFF
	mov	w3, #0
	bl	__ZN3std2rt10lang_start17h4a203ef9f539b20aE
	ldp	x29, x30, [sp], #16
	ret
	.cfi_endproc

	.section	__DATA,__const
	.p2align	3
l___unnamed_1:
	.quad	__ZN4core3ptr85drop_in_place$LT$std..rt..lang_start$LT$$LP$$RP$$GT$..$u7b$$u7b$closure$u7d$$u7d$$GT$17hbce990d7c3914740E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN4core3ops8function6FnOnce40call_once$u7b$$u7b$vtable.shim$u7d$$u7d$17hcbce53e150ddaf25E
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h0d043447c794ac8aE
	.quad	__ZN3std2rt10lang_start28_$u7b$$u7b$closure$u7d$$u7d$17h0d043447c794ac8aE

	.p2align	3
l___unnamed_2:
	.quad	__ZN4core3ptr27drop_in_place$LT$$RF$u8$GT$17hb491c0b8636fbe73E
	.asciz	"\b\000\000\000\000\000\000\000\b\000\000\000\000\000\000"
	.quad	__ZN42_$LT$$RF$T$u20$as$u20$core..fmt..Debug$GT$3fmt17h893f704e0973e266E

	.section	__TEXT,__const
l___unnamed_8:
	.ascii	"invalid args"

	.section	__DATA,__const
	.p2align	3
l___unnamed_3:
	.quad	l___unnamed_8
	.asciz	"\f\000\000\000\000\000\000"

	.section	__TEXT,__const
	.p2align	3
l___unnamed_4:
	.byte	0

l___unnamed_9:
	.ascii	"/rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/core/src/fmt/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_5:
	.quad	l___unnamed_9
	.asciz	"K\000\000\000\000\000\000\000\221\001\000\000\r\000\000"

	.section	__TEXT,__const
l___unnamed_10:
	.ascii	"/rustc/2c8cc343237b8f7d5a3c3703e3a87f2eb2c54a74/library/core/src/array/mod.rs"

	.section	__DATA,__const
	.p2align	3
l___unnamed_6:
	.quad	l___unnamed_10
	.asciz	"M\000\000\000\000\000\000\000D\001\000\000\033\000\000"

	.section	__TEXT,__const
l___unnamed_11:
	.byte	10

	.section	__DATA,__const
	.p2align	3
l___unnamed_7:
	.quad	l___unnamed_4
	.space	8
	.quad	l___unnamed_11
	.asciz	"\001\000\000\000\000\000\000"

.subsections_via_symbols
