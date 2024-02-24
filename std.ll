; ModuleID = 'std.47a7d92d4824152e-cgu.0'
source_filename = "std.47a7d92d4824152e-cgu.0"
target datalayout = "e-m:e-p:32:32-Fi8-i64:64-v128:64:128-a:0:32-n32-S64"
target triple = "armv7-unknown-linux-gnueabihf"

%"core::fmt::Arguments<'_>" = type { { ptr, i32 }, { ptr, i32 }, { ptr, i32 } }
%Obj = type { [4 x i8], i8, [3 x i8], ptr }

@alloc_91c7fa63c3cfeaa3c795652d5cf060e4 = private unnamed_addr constant <{ [12 x i8] }> <{ [12 x i8] c"invalid args" }>, align 1
@alloc_e90401c92a6af8b32765b1534130c461 = private unnamed_addr constant <{ ptr, [4 x i8] }> <{ ptr @alloc_91c7fa63c3cfeaa3c795652d5cf060e4, [4 x i8] c"\0C\00\00\00" }>, align 4
@alloc_c06a172a08ac35a48b6ad59116e021fc = private unnamed_addr constant <{}> zeroinitializer, align 4
@alloc_b5970474149acb40bd55b9b54ff0a4d7 = private unnamed_addr constant <{ [75 x i8] }> <{ [75 x i8] c"/rustc/82e1608dfa6e0b5569232559e3d385fea5a93112/library/core/src/fmt/mod.rs" }>, align 1
@alloc_e2692cde95cee8eff404aaf56d326227 = private unnamed_addr constant <{ ptr, [12 x i8] }> <{ ptr @alloc_b5970474149acb40bd55b9b54ff0a4d7, [12 x i8] c"K\00\00\00M\01\00\00\0D\00\00\00" }>, align 4
@alloc_49a1e817e911805af64bbc7efb390101 = private unnamed_addr constant <{ [1 x i8] }> <{ [1 x i8] c"\0A" }>, align 1
@alloc_d996c3117e6c6cf00322adc8c0464a6e = private unnamed_addr constant <{ ptr, [4 x i8], ptr, [4 x i8] }> <{ ptr @alloc_c06a172a08ac35a48b6ad59116e021fc, [4 x i8] zeroinitializer, ptr @alloc_49a1e817e911805af64bbc7efb390101, [4 x i8] c"\01\00\00\00" }>, align 4

; core::fmt::Arguments::new_v1
; Function Attrs: inlinehint uwtable
define internal void @_ZN4core3fmt9Arguments6new_v117hc1fc00fff85c066aE(ptr sret(%"core::fmt::Arguments<'_>") align 4 %_0, ptr align 4 %pieces.0, i32 %pieces.1, ptr align 4 %args.0, i32 %args.1) unnamed_addr #0 {
start:
  %_15 = alloca { ptr, i32 }, align 4
  %_13 = alloca { ptr, i32 }, align 4
  %_11 = alloca %"core::fmt::Arguments<'_>", align 4
  %_3 = icmp ult i32 %pieces.1, %args.1
  br i1 %_3, label %bb1, label %bb2

bb2:                                              ; preds = %start
  %_8 = add i32 %args.1, 1
  %_6 = icmp ugt i32 %pieces.1, %_8
  br i1 %_6, label %bb3, label %bb4

bb1:                                              ; preds = %start
  br label %bb3

bb4:                                              ; preds = %bb2
  store ptr null, ptr %_13, align 4
  %0 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 0
  store ptr %pieces.0, ptr %0, align 4
  %1 = getelementptr inbounds { ptr, i32 }, ptr %_0, i32 0, i32 1
  store i32 %pieces.1, ptr %1, align 4
  %2 = getelementptr inbounds { ptr, i32 }, ptr %_13, i32 0, i32 0
  %3 = load ptr, ptr %2, align 4, !align !2, !noundef !3
  %4 = getelementptr inbounds { ptr, i32 }, ptr %_13, i32 0, i32 1
  %5 = load i32, ptr %4, align 4
  %6 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 2
  %7 = getelementptr inbounds { ptr, i32 }, ptr %6, i32 0, i32 0
  store ptr %3, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, i32 }, ptr %6, i32 0, i32 1
  store i32 %5, ptr %8, align 4
  %9 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_0, i32 0, i32 1
  %10 = getelementptr inbounds { ptr, i32 }, ptr %9, i32 0, i32 0
  store ptr %args.0, ptr %10, align 4
  %11 = getelementptr inbounds { ptr, i32 }, ptr %9, i32 0, i32 1
  store i32 %args.1, ptr %11, align 4
  ret void

bb3:                                              ; preds = %bb1, %bb2
  store ptr null, ptr %_15, align 4
  %12 = getelementptr inbounds { ptr, i32 }, ptr %_11, i32 0, i32 0
  store ptr @alloc_e90401c92a6af8b32765b1534130c461, ptr %12, align 4
  %13 = getelementptr inbounds { ptr, i32 }, ptr %_11, i32 0, i32 1
  store i32 1, ptr %13, align 4
  %14 = getelementptr inbounds { ptr, i32 }, ptr %_15, i32 0, i32 0
  %15 = load ptr, ptr %14, align 4, !align !2, !noundef !3
  %16 = getelementptr inbounds { ptr, i32 }, ptr %_15, i32 0, i32 1
  %17 = load i32, ptr %16, align 4
  %18 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_11, i32 0, i32 2
  %19 = getelementptr inbounds { ptr, i32 }, ptr %18, i32 0, i32 0
  store ptr %15, ptr %19, align 4
  %20 = getelementptr inbounds { ptr, i32 }, ptr %18, i32 0, i32 1
  store i32 %17, ptr %20, align 4
  %21 = getelementptr inbounds %"core::fmt::Arguments<'_>", ptr %_11, i32 0, i32 1
  %22 = getelementptr inbounds { ptr, i32 }, ptr %21, i32 0, i32 0
  store ptr @alloc_c06a172a08ac35a48b6ad59116e021fc, ptr %22, align 4
  %23 = getelementptr inbounds { ptr, i32 }, ptr %21, i32 0, i32 1
  store i32 0, ptr %23, align 4
; call core::panicking::panic_fmt
  call void @_ZN4core9panicking9panic_fmt17h8911d7f11480f1bdE(ptr align 4 %_11, ptr align 4 @alloc_e2692cde95cee8eff404aaf56d326227) #3
  unreachable
}

; Function Attrs: uwtable
define ptr @test(ptr %ob) unnamed_addr #1 {
start:
  %_0.i = alloca { ptr, ptr }, align 4
  %_7 = alloca [1 x { ptr, ptr }], align 4
  %_3 = alloca %"core::fmt::Arguments<'_>", align 4
  %_9 = getelementptr inbounds %Obj, ptr %ob, i32 0, i32 1
  store ptr %_9, ptr %_0.i, align 4
  %0 = getelementptr inbounds { ptr, ptr }, ptr %_0.i, i32 0, i32 1
  store ptr @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$i8$GT$3fmt17hd4a6609f40fb10eeE", ptr %0, align 4
  %1 = load ptr, ptr %_0.i, align 4, !nonnull !3, !align !4, !noundef !3
  %2 = getelementptr inbounds { ptr, ptr }, ptr %_0.i, i32 0, i32 1
  %3 = load ptr, ptr %2, align 4, !nonnull !3, !noundef !3
  %4 = insertvalue { ptr, ptr } poison, ptr %1, 0
  %5 = insertvalue { ptr, ptr } %4, ptr %3, 1
  %_8.0 = extractvalue { ptr, ptr } %5, 0
  %_8.1 = extractvalue { ptr, ptr } %5, 1
  %6 = getelementptr inbounds [1 x { ptr, ptr }], ptr %_7, i32 0, i32 0
  %7 = getelementptr inbounds { ptr, ptr }, ptr %6, i32 0, i32 0
  store ptr %_8.0, ptr %7, align 4
  %8 = getelementptr inbounds { ptr, ptr }, ptr %6, i32 0, i32 1
  store ptr %_8.1, ptr %8, align 4
; call core::fmt::Arguments::new_v1
  call void @_ZN4core3fmt9Arguments6new_v117hc1fc00fff85c066aE(ptr sret(%"core::fmt::Arguments<'_>") align 4 %_3, ptr align 4 @alloc_d996c3117e6c6cf00322adc8c0464a6e, i32 2, ptr align 4 %_7, i32 1)
; call std::io::stdio::_print
  call void @_ZN3std2io5stdio6_print17h135615577e490158E(ptr align 4 %_3)
  ret ptr %ob
}

; core::fmt::num::imp::<impl core::fmt::Display for i8>::fmt
; Function Attrs: uwtable
declare zeroext i1 @"_ZN4core3fmt3num3imp51_$LT$impl$u20$core..fmt..Display$u20$for$u20$i8$GT$3fmt17hd4a6609f40fb10eeE"(ptr align 1, ptr align 4) unnamed_addr #1

; core::panicking::panic_fmt
; Function Attrs: cold noinline noreturn uwtable
declare void @_ZN4core9panicking9panic_fmt17h8911d7f11480f1bdE(ptr align 4, ptr align 4) unnamed_addr #2

; std::io::stdio::_print
; Function Attrs: uwtable
declare void @_ZN3std2io5stdio6_print17h135615577e490158E(ptr align 4) unnamed_addr #1

attributes #0 = { inlinehint uwtable "target-cpu"="generic" "target-features"="+v7,+vfp3,-d32,+thumb2,-neon" }
attributes #1 = { uwtable "target-cpu"="generic" "target-features"="+v7,+vfp3,-d32,+thumb2,-neon" }
attributes #2 = { cold noinline noreturn uwtable "target-cpu"="generic" "target-features"="+v7,+vfp3,-d32,+thumb2,-neon" }
attributes #3 = { noreturn }

!llvm.module.flags = !{!0}
!llvm.ident = !{!1}

!0 = !{i32 4, !"PIC Level", i32 2}
!1 = !{!"rustc version 1.75.0 (82e1608df 2023-12-21)"}
!2 = !{i64 4}
!3 = !{}
!4 = !{i64 1}
