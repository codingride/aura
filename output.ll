; ModuleID = 'aura_main'
target datalayout = "e-m:e-p:32:32-p10:32:32-p20:32:32-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

define i64 @add(i64 %a, i64 %b) {
entry:
  %_a = alloca i64, align 8
  store i64 %a, i64* %_a, align 8
  %_b = alloca i64, align 8
  store i64 %b, i64* %_b, align 8
  %1 = load i64, ptr %_a, align 8
  %2 = load i64, ptr %_b, align 8
  %3 = add nsw i64 %1, %2
  ret i64 %3
}

define double @scale(double %factor) {
entry:
  %_factor = alloca double, align 8
  store double %factor, double* %_factor, align 8
  %4 = load double, ptr %_factor, align 8
  %5 = fadd double %4, 1.5
  ret double %5
}

define i64 @add_bonus(i64 %val) {
entry:
  %_val = alloca i64, align 8
  store i64 %val, i64* %_val, align 8
  %6 = load double, ptr %_val, align 8
  %7 = fadd double %6, 10.0
  ret double %7
}

