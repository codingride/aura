; ModuleID = 'aura_main'
source_filename = "main.au"

@.str.print_int = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
declare i32 @printf(ptr, ...)

define i64 @add(i64 %a, i64 %b) {
entry:
  %_a = alloca i64, align 8
  store i64 %a, i64* %_a, align 8
  %_b = alloca i64, align 8
  store i64 %b, i64* %_b, align 8
  %10 = load i64, ptr %_a, align 8
  %11 = load i64, ptr %_b, align 8
  %12 = add nsw i64 %10, %11
  ret i64 %12
}

define i32 @main() {
entry:
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8

  %greeting = alloca i64, align 8
  store i64 0, i64* %greeting, align 8

  ; print variable
  %1 = load i64, ptr %greeting, align 8
  %2 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %1)

  %3 = load i64, ptr %greeting, align 8
  %print = alloca i64, align 8
  store i64 %3, i64* %print, align 8

  ; print variable
  %4 = load i64, ptr %print, align 8
  %5 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %4)

  %6 = mul nsw i64 10, 2
  %7 = add nsw i64 5, %6
  %value = alloca i64, align 8
  store i64 %7, i64* %value, align 8

  ; print variable
  %8 = load i64, ptr %value, align 8
  %9 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %8)

  %threshold = alloca i64, align 8
  store i64 100, i64* %threshold, align 8

  %13 = call i64 @add(i64 20, i64 22)
  %result = alloca i64, align 8
  store i64 %13, i64* %result, align 8

  ; print variable
  %14 = load i64, ptr %result, align 8
  %15 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %14)

  ret i32 0
}
