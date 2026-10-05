; ModuleID = 'aura_main'
source_filename = "main.au"

@.str.print_int = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1
declare i32 @printf(ptr, ...)

define i64 @calculate(i64 %a, i64 %b) {
entry:
  %_a = alloca i64, align 8
  store i64 %a, i64* %_a, align 8
  %_b = alloca i64, align 8
  store i64 %b, i64* %_b, align 8

; let score = ...
  %score = alloca i64, align 8
  store i64 100, i64* %score, align 8

  ; print the variable out to screen
  %1 = load i64, ptr %score, align 8
  %2 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %1)

  ret i64 0
}

define i32 @main() {
entry:
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8

; if condition branching
  %3 = load i64, ptr %age, align 8
  %4 = icmp sgt i64 %3, 18
  br i1 %4, label %then_1, label %merge_1

then_1:
; let access = ...
  %access = alloca i64, align 8
  store i64 1, i64* %access, align 8

  ; print the variable out to screen
  %5 = load i64, ptr %access, align 8
  %6 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %5)

  br label %merge_1

merge_1:
  ret i32 0
}
