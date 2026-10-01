; ModuleID = 'aura_main'
source_filename = "main.au"

@.str.print_int = private unnamed_addr constant [4 x i8] c"%d\0A\00", align 1

declare i32 @printf(ptr, ...)

define i32 @main() {
entry:
  %age = alloca i64, align 8
  store i64 25, i64* %age, align 8

; if condition branching
  %1 = load i64, ptr %age, align 8
  %2 = icmp sgt i64 %1, 18
  br i1 %2, label %then_1, label %merge_1

then_1:
; let access = ...
  %access = alloca i64, align 8
  store i64 1, i64* %access, align 8

  ; print the variable out to screen
  %3 = load i64, ptr %access, align 8
  %4 = call i32 (ptr, ...) @printf(ptr @.str.print_int, i64 %3)

  br label %merge_1

merge_1:
  ret i32 0
}
