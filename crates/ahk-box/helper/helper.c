/* The helper: a Windows program that the raven runs.
 *
 * The tree's DllCall calls a function of a library of the machine. On the
 * box, the library is a .so and the call is made by the trampoline of the
 * box. When the library is a Windows one, the call is made here: this
 * program runs inside the raven, loads the library of the Windows that
 * the raven emulates, and calls the function.
 *
 * It is built the way the raven builds its own programs: no C runtime,
 * the entry point of the PE called directly, and the kernel32 of the
 * raven for the services. The only import is kernel32.
 *
 * The protocol is one line per call, and every value is hexadecimal:
 *
 *   request:  library \t function \t return-type \t count \t type \t value ...
 *   answer:   ok \t value \t block ...      or      error \t reason
 *
 * The type is the name the tree spells it with, and a star at the end
 * says that the argument is the address of a value. The value of an
 * integer is the bits of the whole number, the value of a float is the
 * bits of the number, and the value of a text is the bytes of the text.
 */

typedef unsigned int u32;
typedef unsigned long long u64;
typedef unsigned short u16;
typedef int BOOL;

/* The services of the kernel32 that the helper asks for. They are
   declared here and not through a header: the helper has no C runtime,
   and a header of the Windows is a runtime in disguise. */
typedef void *HANDLE;
typedef void *HMODULE;
typedef void *FARPROC;

#define STD_INPUT_HANDLE ((u32) - 10)
#define STD_OUTPUT_HANDLE ((u32) - 11)

__declspec(dllimport) HMODULE __stdcall LoadLibraryW(const u16 *path);
__declspec(dllimport) FARPROC __stdcall GetProcAddress(HMODULE module, const char *name);
__declspec(dllimport) HANDLE __stdcall GetStdHandle(u32 which);
__declspec(dllimport) BOOL __stdcall ReadFile(HANDLE handle, void *buffer, u32 count,
                                             u32 *read, void *overlapped);
__declspec(dllimport) BOOL __stdcall WriteFile(HANDLE handle, const void *buffer, u32 count,
                                              u32 *written, void *overlapped);

/* The types of the arguments, as the tree spells them. */
struct kind {
    const char *name;
    int width;             /* the bytes of the value */
    int is_float;          /* the answer comes in the float register */
    int is_text;           /* the value is the address of bytes */
    int character_width;   /* the bytes of a character of the text */
    int is_unsigned;       /* the number does not go negative */
};

static const struct kind kinds[] = {
    {"int", 4, 0, 0, 0, 0},     {"int64", 8, 0, 0, 0, 0},   {"short", 2, 0, 0, 0, 0},
    {"char", 1, 0, 0, 0, 0},    {"uchar", 1, 0, 0, 0, 1},   {"ushort", 2, 0, 0, 0, 1},
    {"uint", 4, 0, 0, 0, 1},    {"uint64", 8, 0, 0, 0, 1},  {"float", 4, 1, 0, 0, 0},
    {"double", 8, 1, 0, 0, 0},  {"ptr", 8, 0, 0, 0, 0},    {"uptr", 8, 0, 0, 0, 1},
    {"str", 2, 0, 1, 2, 0},     {"wstr", 2, 0, 1, 2, 0},   {"astr", 1, 0, 1, 1, 0},
};

#define KINDS ((int) (sizeof(kinds) / sizeof(kinds[0])))
#define MAX_ARGUMENTS 32
#define TEXT_AREA 65536
#define BLOCK_AREA 4096

/* The room of the texts that go out, and of the blocks that come back. */
static unsigned char text_area[TEXT_AREA];
static unsigned char block_area[MAX_ARGUMENTS][BLOCK_AREA];

/* The text that comes back, and the room of the read. Both are out of
   the stack: a frame of many kilobytes asks for a stack probe, and the
   helper has no C runtime to give it. */
static unsigned char answer_bytes[TEXT_AREA];
static char chunk[4096];

/* ------------------------------------------------------------------ text */

static int text_length(const char *text)
{
    int length = 0;
    while (text[length])
        length++;
    return length;
}

static int same_text(const char *left, const char *right, int length)
{
    int at;
    for (at = 0; at < length; at++)
        if (left[at] != right[at])
            return 0;
    return 1;
}

static void say(HANDLE handle, const char *text)
{
    u32 written = 0;
    WriteFile(handle, text, (u32) text_length(text), &written, 0);
}

static void say_hex(HANDLE handle, u64 number)
{
    static const char digits[] = "0123456789abcdef";
    char text[16];
    int at = 16;
    int index;
    for (index = 0; index < 16; index++) {
        text[--at] = digits[number & 15];
        number >>= 4;
    }
    say(handle, &text[at]);
}

static void say_bytes(HANDLE handle, const unsigned char *bytes, int length)
{
    static const char digits[] = "0123456789abcdef";
    int at;
    for (at = 0; at < length; at++) {
        char pair[3];
        pair[0] = digits[bytes[at] >> 4];
        pair[1] = digits[bytes[at] & 15];
        pair[2] = 0;
        say(handle, pair);
    }
}

/* ------------------------------------------------------------------ hex */

static int digit(char symbol)
{
    if (symbol >= '0' && symbol <= '9')
        return symbol - '0';
    if (symbol >= 'a' && symbol <= 'f')
        return symbol - 'a' + 10;
    if (symbol >= 'A' && symbol <= 'F')
        return symbol - 'A' + 10;
    return -1;
}

/* The number of the hexadecimal text. The answer is how many were read. */
static u64 from_hex(const char *text, int length)
{
    u64 number = 0;
    int index;
    for (index = 0; index < length; index++) {
        int value = digit(text[index]);
        if (value < 0)
            break;
        number = (number << 4) | (u64) value;
    }
    return number;
}

/* The number of the decimal text. */
static int from_decimal(const char *text)
{
    int number = 0;
    int at;
    for (at = 0; text[at]; at++) {
        if (text[at] < '0' || text[at] > '9')
            break;
        number = number * 10 + (text[at] - '0');
    }
    return number;
}

/* ----------------------------------------------------------------- kinds */

static const struct kind *kind_of(const char *name)
{
    int index;
    int length = text_length(name);
    for (index = 0; index < KINDS; index++)
        if (text_length(kinds[index].name) == length && same_text(kinds[index].name, name, length))
            return &kinds[index];
    return 0;
}

/* The number, in the width its type asks for.
 *
 * The tree sends the bits of the whole number, and the type says how
 * many of them the callee reads: the number is cut to the width and
 * stretched again, with the sign the type says. */
static u64 narrow(u64 number, const struct kind *kind)
{
    int bits = kind->width * 8;
    if (bits >= 64)
        return number;
    number &= ((u64) 1 << bits) - 1;
    if (!kind->is_unsigned && (number & ((u64) 1 << (bits - 1))))
        number |= ~(((u64) 1 << bits) - 1);
    return number;
}

/* ------------------------------------------------------------------ call */

/* The call of a function whose shape is only known now.
 *
 * The register of an argument is the one of its place: the integers go
 * in RCX, RDX, R8 and R9, and the float of a place goes in the XMM of
 * that place and in the register of the integers of it. The bytes of an
 * argument are the same in the two, and the callee reads the one its
 * signature names.
 *
 * words: the eight bytes of every argument.
 * count: how many there are.
 * function: the address of the function.
 */
static u64 integer_answer;
static u64 float_answer;

static void invoke(u64 function, const u64 *words, int count)
{
    u64 gpr[4];
    u64 xmm[4];
    u64 stack[MAX_ARGUMENTS];
    int stack_size = 0;
    int index;

    for (index = 0; index < 4; index++) {
        gpr[index] = 0;
        xmm[index] = 0;
    }
    for (index = 0; index < count; index++) {
        u64 word = words[index];
        if (index < 4) {
            gpr[index] = word;
            xmm[index] = word;
        } else
            stack[stack_size++] = word;
    }

    __asm__ volatile(
        /* The stack of the call: the shadow of the four registers, and
           the arguments that do not fit in a register. The old stack is
           kept in RBX, which the callee has to give back: R11 and the
           other registers of the ABI are volatile, and a callee that
           takes one of them leaves nothing to come back to. */
        "push %%rbx\n\t"
        "mov %%rsp, %%rbx\n\t"
        "mov %[size], %%r10\n\t"
        "sub %%r10, %%rsp\n\t"
        "and $-16, %%rsp\n\t"
        "xor %%eax, %%eax\n\t"
        "1:\n\t"
        "cmp %%rax, %[size]\n\t"
        "jae 2f\n\t"
        "mov (%[stack],%%rax,8), %%rdx\n\t"
        "mov %%rdx, 32(%%rsp,%%rax,8)\n\t"
        "inc %%eax\n\t"
        "jmp 1b\n\t"
        "2:\n\t"
        /* The floats, by position. They go first: the register of the
           pointers is free after this. */
        "movq (%[xmm]), %%xmm0\n\t"
        "movq 8(%[xmm]), %%xmm1\n\t"
        "movq 16(%[xmm]), %%xmm2\n\t"
        "movq 24(%[xmm]), %%xmm3\n\t"
        /* The integers, by position. */
        "mov (%[gpr]), %%rcx\n\t"
        "mov 8(%[gpr]), %%rdx\n\t"
        "mov 16(%[gpr]), %%r8\n\t"
        "mov 24(%[gpr]), %%r9\n\t"
        /* The call. */
        "call *%[function]\n\t"
        /* The answers come first: the registers are volatile. */
        "movq %%xmm0, %[floating]\n\t"
        "movq %%rax, %[integer]\n\t"
        /* The stack goes back where it was. */
        "mov %%rbx, %%rsp\n\t"
        "pop %%rbx\n\t"
        : [integer] "=m"(integer_answer), [floating] "=m"(float_answer)
        : [gpr] "r"(gpr), [xmm] "r"(xmm), [stack] "r"(stack),
          [size] "r"((u64) (48 + stack_size * 8)), [function] "r"(function)
        : "rax", "rcx", "rdx", "r8", "r9", "r10", "r11", "rbx", "memory", "cc");
}

/* -------------------------------------------------------------- the call */

/* The answer of the call, and the blocks that came back. */
static char answer[64];

static void answer_error(HANDLE out, const char *reason)
{
    say(out, "error\t");
    say(out, reason);
    say(out, "\n");
}

/* The text at an address, in the encoding its type asks for. */
static int read_text(u64 address, int width, unsigned char *out)
{
    int length = 0;
    if (!address)
        return 0;
    for (;;) {
        int at;
        int nul = 1;
        for (at = 0; at < width; at++)
            if (((unsigned char *) address)[at])
                nul = 0;
        if (nul)
            break;
        for (at = 0; at < width; at++)
            out[length++] = ((unsigned char *) address)[at];
        address += (u64) width;
        if (length > TEXT_AREA - 16)
            break;
    }
    out[length] = 0;
    return length;
}

/* The one call the tree asks for.
 *
 * The fields of the request are the library, the function, the type of
 * the answer, the number of the arguments, and then the type and the
 * value of every argument, one after the other. */
static void serve(HANDLE out, char **fields, int count_fields)
{
    const char *library = fields[0];
    const char *function = fields[1];
    const char *return_type = fields[2];
    int count = from_decimal(fields[3]);
    const struct kind *returns = kind_of(return_type);
    u64 words[MAX_ARGUMENTS];
    unsigned char *texts[MAX_ARGUMENTS];
    int is_address[MAX_ARGUMENTS];
    int block_length[MAX_ARGUMENTS];
    int at_text = 0;
    int index;

    if (!returns) {
        answer_error(out, "Invalid return type.");
        return;
    }
    if (count > MAX_ARGUMENTS) {
        answer_error(out, "Too many arguments.");
        return;
    }
    if (count_fields < 4 + count * 2) {
        answer_error(out, "The request is not a call.");
        return;
    }
    /* The library, and the function in it. */
    {
        u16 wide[512];
        int at;
        for (at = 0; at < 511 && library[at]; at++)
            wide[at] = (u16) (unsigned char) library[at];
        wide[at] = 0;
        HMODULE module = LoadLibraryW(wide);
        if (!module) {
            answer_error(out, "Failed to load DLL.");
            return;
        }
        {
            FARPROC address = GetProcAddress(module, function);
            if (!address) {
                answer_error(out, "Call to nonexistent function.");
                return;
            }
            /* The arguments: the text of one goes out by its address,
               and the address of a value is a block of its own. */
            for (index = 0; index < count; index++) {
                char base[32];
                const struct kind *kind;
                const char *type = fields[4 + index * 2];
                const char *value_text = fields[5 + index * 2];
                int length = text_length(type);
                int address_argument = 0;
                u64 value = 0;
                int at;

                /* A star at the end of the type says that the argument
                   is the address of a value. */
                if (length && (type[length - 1] == '*' || type[length - 1] == 'P')) {
                    address_argument = 1;
                    length--;
                }
                if (length > 31)
                    length = 31;
                for (at = 0; at < length; at++)
                    base[at] = type[at];
                base[length] = 0;
                kind = kind_of(base);
                if (!kind) {
                    answer_error(out, "Invalid arg type.");
                    return;
                }
                is_address[index] = address_argument;
                texts[index] = 0;
                block_length[index] = 0;

                if (kind->is_text) {
                    /* The bytes of the text, and their place. */
                    int bytes;
                    int at;
                    unsigned char *place;
                    /* The length of the text is the number of the hex
                       digits, half of them the bytes of the text. */
                    int digits = 0;
                    while (value_text[digits] && digit(value_text[digits]) >= 0)
                        digits++;
                    bytes = digits / 2;
                    place = text_area + at_text;
                    for (at = 0; at < bytes; at++)
                        place[at] = (unsigned char) from_hex(value_text + at * 2, 2);
                    place[bytes] = 0;
                    place[bytes + 1] = 0;
                    at_text += bytes + 2;
                    texts[index] = place;
                    value = (u64) place;
                    if (address_argument) {
                        /* The block of the address is the text itself:
                           the callee may write into it. */
                        block_length[index] = bytes;
                    }
                } else if (kind->is_float) {
                    if (kind->width == 4)
                        value = from_hex(value_text, 8);
                    else
                        value = from_hex(value_text, 16);
                    if (address_argument) {
                        unsigned char *block = block_area[index];
                        int at;
                        for (at = 0; at < kind->width; at++)
                            block[at] = (unsigned char) ((value >> (at * 8)) & 255);
                        block_length[index] = kind->width;
                        value = (u64) block;
                    }
                } else {
                    value = narrow(from_hex(value_text, 16), kind);
                    if (address_argument) {
                        unsigned char *block = block_area[index];
                        int at;
                        for (at = 0; at < kind->width; at++)
                            block[at] = (unsigned char) ((value >> (at * 8)) & 255);
                        block_length[index] = kind->width;
                        value = (u64) block;
                    }
                }
                words[index] = value;
            }
            invoke((u64) address, words, count);
        }
    }
    /* The answer. */
    say(out, "ok\t");
    if (returns->is_float)
        say_hex(out, float_answer);
    else if (returns->is_text) {
        int length = read_text(integer_answer, returns->character_width, answer_bytes);
        say_bytes(out, answer_bytes, length);
    } else
        say_hex(out, integer_answer);
    say(out, "\t");
    /* The blocks that came back. */
    for (index = 0; index < count; index++) {
        if (!is_address[index])
            continue;
        if (texts[index])
            say_bytes(out, texts[index], block_length[index]);
        else
            say_bytes(out, block_area[index], block_length[index]);
        say(out, "\t");
    }
    say(out, "\n");
}

int raven_entry(void)
{
    HANDLE out = GetStdHandle(STD_OUTPUT_HANDLE);
    HANDLE in = GetStdHandle(STD_INPUT_HANDLE);
    static char line[TEXT_AREA];
    int length = 0;
    for (;;) {
        u32 read = 0;
        int at;
        if (!ReadFile(in, chunk, (u32) sizeof(chunk), &read, 0) || !read)
            break;
        for (at = 0; at < (int) read; at++) {
            if (chunk[at] == '\n') {
                line[length] = 0;
                /* The fields of the request. */
                {
                    static char *fields[MAX_ARGUMENTS * 2 + 8];
                    int count_fields = 0;
                    char *cursor = line;
                    int field;
                    fields[count_fields++] = cursor;
                    for (field = 0; cursor[field]; field++)
                        if (cursor[field] == '\t') {
                            cursor[field] = 0;
                            if (count_fields < (int) (sizeof(fields) / sizeof(fields[0])))
                                fields[count_fields++] = cursor + field + 1;
                        }
                    if (count_fields >= 4)
                        serve(out, fields, count_fields);
                    else
                        answer_error(out, "The request is not a call.");
                }
                length = 0;
            } else if (length < TEXT_AREA - 1)
                line[length++] = chunk[at];
        }
    }
    return 0;
}
