.section ".text.boot"
.global _start
_start:
    // 设置栈指针
    ldr x30, =LD_STACK_PTR
    mov sp, x30
    // 跳转到C主函数
    bl rust_main
    .equ PSCI_SYSTEM_OFF, 0x84000002
    .globl system_off
    system_off:
            ldr     x0, =PSCI_SYSTEM_OFF
            hvc     #0
