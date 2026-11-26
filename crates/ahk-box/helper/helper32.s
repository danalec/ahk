/* The call of the thirty-two bits.
 *
 * The arguments of a Windows function of thirty-two bits go on the
 * stack, and the last one is pushed first: the first argument is at
 * the lowest place of the stack. The answer of an integer is in EAX,
 * and the answer of a float is in ST0, the stack of the x87.
 *
 * The assembly is in a file of its own, and not in the text of the
 * compiler: the machine of thirty-two bits has six registers to spend,
 * and the compiler of the C has none left for the operands of a block
 * of assembly that wants three of them and takes three more.
 *
 * The caller of _ahk_invoke32 pushes the image of the arguments: the
 * words in the order they are pushed, the last argument first.
 *
 * The answer of an integer of eight bytes is in EDX and EAX, and the two
 * are the one answer: the EDX of the callee is left alone.
 */

    .text
    .globl  _ahk_invoke32
_ahk_invoke32:
    pushl   %ebp
    movl    %esp, %ebp
    pushl   %ebx
    pushl   %esi
    pushl   %edi
    movl    8(%ebp), %eax          /* the address of the function */
    movl    12(%ebp), %ecx         /* the image of the arguments */
    movl    16(%ebp), %edx         /* how many words there are */
    testl   %edx, %edx
    jz      2f
1:  pushl   (%ecx)                 /* the words, in the order of the image */
    addl    $4, %ecx
    decl    %edx
    jnz     1b
2:  call    *%eax
    /* The callee takes the arguments back, and the stack of the caller
       is where it was. */
    leal    -12(%ebp), %esp
    popl    %edi
    popl    %esi
    popl    %ebx
    popl    %ebp
    ret

/* The answer of a float: ST0, the stack of the x87.
 *
 * It is stored as a double, and the value of a float is exact in a
 * double. The bits of the double come back in EDX and EAX, the way a
 * whole number of eight bytes comes back on this machine.
 */
    .globl  _ahk_float32
_ahk_float32:
    subl    $8, %esp
    fstpl   (%esp)
    movl    (%esp), %eax          /* the low half of the double */
    movl    4(%esp), %edx         /* the high half of it */
    addl    $8, %esp
    ret
