; ModuleID = 'aura_main'
source_filename = "main.au"

@.str.print_int = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8

; let greeting = ...
  %greeting = alloca i64, align 8
  store i64 0, i64* %greeting, align 8

  ; print the variable out to screen
  %1 = load i64, ptr %greeting, align 8
  %2 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %1)

; let print = ...
  %3 = load i64, ptr %greeting, align 8
  %print = alloca i64, align 8
  store i64 %3, i64* %print, align 8

  ; print the variable out to screen
  %4 = load i64, ptr %print, align 8
  %5 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %4)

; let value = ...
  %6 = mul nsw i64 10, 2
  %7 = add nsw i64 5, %6
  %value = alloca i64, align 8
  store i64 %7, i64* %value, align 8

  ; print the variable out to screen
  %8 = load i64, ptr %value, align 8
  %9 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %8)

; const threshold = ...
  %threshold = alloca i64, align 8
  store i64 100, i64* %threshold, align 8

  ret i32 0
}
