; ModuleID = 'aura_main'
target datalayout = "e-m:e-p:32:32-p10:32:32-p20:32:32-i64:64-n32:64-S128-ni:1:10:20"
target triple = "wasm32-unknown-unknown"

define i64 @add(i64 %a, i64 %b) {
entry:
  %_a = alloca i64, align 8
  store i64 %a, i64* %_a, align 8
  %_b = alloca i64, align 8
  store i64 %b, i64* %_b, align 8
  %4 = load i64, ptr %_a, align 8
  %5 = load i64, ptr %_b, align 8
  %6 = add nsw i64 %4, %5
  ret i64 %6
}

define i32 @main() {
entry:
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8

  %greeting = alloca i64, align 8
  store i64 0, i64* %greeting, align 8

  %1 = load i64, ptr %greeting, align 8
  %print = alloca i64, align 8
  store i64 %1, i64* %print, align 8

  %2 = mul nsw i64 10, 2
  %3 = add nsw i64 5, %2
  %value = alloca i64, align 8
  store i64 %3, i64* %value, align 8

  %threshold = alloca i64, align 8
  store i64 100, i64* %threshold, align 8

  %7 = call i64 @add(i64 20, i64 22)
  %result = alloca i64, align 8
  store i64 %7, i64* %result, align 8

  ret i32 0
}
