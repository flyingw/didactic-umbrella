//! Script commands

// --------------------------------------------------------------------------------------------
// Constants
// --------------------------------------------------------------------------------------------

/// Pushes 0 onto the stack
pub const OP_0: u8 = 0;
/// Pushes 0 onto the stack
pub const OP_FALSE: u8 = 0;

/// Offset by n to push n bytes onto the stack, where n: [1-75]
pub const OP_PUSH: u8 = 0;

pub const OP_PUSH_20: u8 = 20;

/// The next byte sets the number of bytes to push onto the stack
pub const OP_PUSHDATA1: u8 = 76;
/// The next two bytes sets the number of bytes to push onto the stack
pub const OP_PUSHDATA2: u8 = 77;
/// The next four bytes sets the number of bytes to push onto the stack
pub const OP_PUSHDATA4: u8 = 78;

// --------------------------------------------------------------------------------------------
// Flow Control
// --------------------------------------------------------------------------------------------

/// Does nothing
pub const OP_NOP: u8 = 97;
/// If the top stack is true, statements are executed. Top stack value is removed.
pub const OP_IF: u8 = 99;
/// If the top stack is false, statements are executed. Top stack value is removed.
pub const OP_NOTIF: u8 = 100;
/// If the preceding OP_IF or OP_NOTIF statemetns were not executed, then statements are executed.
pub const OP_ELSE: u8 = 103;
/// Ends an if-else block
pub const OP_ENDIF: u8 = 104;
/// Marks a statement as invalid if the top stack value is false. Top stack value is removed.
pub const OP_VERIFY: u8 = 105;
/// Marks a statements as invalid
pub const OP_RETURN: u8 = 106;

// --------------------------------------------------------------------------------------------
// Stack
// --------------------------------------------------------------------------------------------

/// Moves the top item on the main stack to the alt stack
pub const OP_TOALTSTACK: u8 = 107;
/// Moves the top item on the alt stack to the main stack
pub const OP_FROMALTSTACK: u8 = 108;
/// Duplicates the top stack value if it is not zero
pub const OP_IFDUP: u8 = 115;
/// Puts the number of stack items onto the stack
pub const OP_DEPTH: u8 = 116;
/// Drops the top stack value
pub const OP_DROP: u8 = 117;
/// Duplicates the top stack item
pub const OP_DUP: u8 = 118;
/// Removes the second-to-top stack item
pub const OP_NIP: u8 = 119;
/// Copies the second-to-top stack item to the top
pub const OP_OVER: u8 = 120;
/// The item n back in the stack is copied to the top
pub const OP_PICK: u8 = 121;
/// The item n back in the stack is moved to the top
pub const OP_ROLL: u8 = 122;
/// The top three items on the stack are rotated to the left
pub const OP_ROT: u8 = 123;
/// The top two items on the stack are swapped
pub const OP_SWAP: u8 = 124;
/// The item at the top of the stack is copied and inserted before the second-to-top item
pub const OP_TUCK: u8 = 125;
/// Removes the top two items from the stack
pub const OP_2DROP: u8 = 109;
/// Duplicates the top two stack items
pub const OP_2DUP: u8 = 110;
/// Duplicates the top three stack items
pub const OP_3DUP: u8 = 111;
/// Copies the pair of items two spaces back to the front
pub const OP_2OVER: u8 = 112;
/// The fifth and sixth items back are moved to the top of the stack
pub const OP_2ROT: u8 = 113;
/// Swaps the top two pairs of items
pub const OP_2SWAP: u8 = 114;

// --------------------------------------------------------------------------------------------
// Splice
// --------------------------------------------------------------------------------------------

/// Concatenates two byte sequences
pub const OP_CAT: u8 = 126;
/// Splits the byte sequence at position n
pub const OP_SPLIT: u8 = 127;
/// Pushes the byte sequence length of the top stack item without popping it
pub const OP_SIZE: u8 = 130;

// --------------------------------------------------------------------------------------------
// Bitwise Logic
// --------------------------------------------------------------------------------------------

/// Boolean and between each bit in the inputs
pub const OP_AND: u8 = 132;
/// Boolean or between each bit in the inputs
pub const OP_OR: u8 = 133;
/// Boolean exclusive or between each bit in the inputs
pub const OP_XOR: u8 = 134;
/// Returns 1 if the inputs are exactly equal, 0 otherwise
pub const OP_EQUAL: u8 = 135;
/// Same as OP_EQUAL, but runs OP_VERIFY afterward
pub const OP_EQUALVERIFY: u8 = 136;

// --------------------------------------------------------------------------------------------
// Arithmetic
// --------------------------------------------------------------------------------------------

/// Adds 1 to the input
pub const OP_1ADD: u8 = 139;
/// Subtracts 1 from the input
pub const OP_1SUB: u8 = 140;
/// The sign of the input is flipped
pub const OP_NEGATE: u8 = 143;
/// The input is made positive
pub const OP_ABS: u8 = 144;
/// If the input is 0 or 1, it is flipped. Otherwise, the output will be 0.
pub const OP_NOT: u8 = 145;
/// Returns 0 if the input is 0. 1 otherwise.
pub const OP_0NOTEQUAL: u8 = 146;
/// Adds a to b
pub const OP_ADD: u8 = 147;
/// Subtracts b from a
pub const OP_SUB: u8 = 148;
/// Divides a by b
pub const OP_DIV: u8 = 150;
/// Returns the remainder after dividing a by b
pub const OP_MOD: u8 = 151;
/// If both a and b are not empty, the output is 1. Otherwise, 0.
pub const OP_BOOLAND: u8 = 154;
/// If a or b is not empty, the output is 1. Otherwise, 0.
pub const OP_BOOLOR: u8 = 155;
/// Returns 1 if the numbers are equal. Otherwise, 0.
pub const OP_NUMEQUAL: u8 = 156;
/// Same as OP_NUMEQUAL, but runs OP_VERIFY afterward
pub const OP_NUMEQUALVERIFY: u8 = 157;
/// Returns 1 if the numbers are not equal. Otherwise, 0.
pub const OP_NUMNOTEQUAL: u8 = 158;
/// Returns 1 if a is less than b. Otherwise, 0.
pub const OP_LESSTHAN: u8 = 159;
/// Returns 1 if a is greater than b. Otherwise, 0.
pub const OP_GREATERTHAN: u8 = 160;
/// Returns 1 if a is less than or equal to b. Otherwise, 0.
pub const OP_LESSTHANOREQUAL: u8 = 161;
/// Returns 1 if a is greater than or equal to b. Otherwise, 0.
pub const OP_GREATERTHANOREQUAL: u8 = 162;
/// Returns the smaller of a and b
pub const OP_MIN: u8 = 163;
/// Returns the larger of a and b
pub const OP_MAX: u8 = 164;
/// Returns 1 if x is within the specified range, left inclusive. Otherwise, 0.
pub const OP_WITHIN: u8 = 165;
/// Converts numeric value a into a byte sequence of length b
pub const OP_NUM2BIN: u8 = 128;
/// Converts byte sequence x into a numeric value
pub const OP_BIN2NUM: u8 = 129;

// --------------------------------------------------------------------------------------------
// Cryptography
// --------------------------------------------------------------------------------------------

/// The input is hashed using RIPEMD-160
pub const OP_RIPEMD160: u8 = 166;
/// The input is hashed using SHA-1
pub const OP_SHA1: u8 = 167;
/// The input is hashed using SHA-256
pub const OP_SHA256: u8 = 168;
/// The input is hashed twice: first with SHA-256 and then with RIPEMD-160
pub const OP_HASH160: u8 = 169;
/// The input is hashed two times with SHA-256
pub const OP_HASH256: u8 = 170;
/// Marks the part of the script after which the signature will begin matching
pub const OP_CODESEPARATOR: u8 = 171;
/// Puts 1 on the stack if the signature authorizes the public key and transaction hash. Otherwise 0.
pub const OP_CHECKSIG: u8 = 172;
/// Same as OP_CHECKSIG, but OP_VERIFY is executed afterward
pub const OP_CHECKSIGVERIFY: u8 = 173;
/// Puts 1 on the stack if m of n signatures authorize the public key and transaction hash. Otherwise 0.
pub const OP_CHECKMULTISIG: u8 = 174;
/// Same as OP_CHECKMULTISIG, but OP_VERIFY is executed afterward
pub const OP_CHECKMULTISIGVERIFY: u8 = 175;

// --------------------------------------------------------------------------------------------
// Locktime
// --------------------------------------------------------------------------------------------

/// Marks transaction as invalid if the top stack item is greater than the transaction's lock_time
pub const OP_CHECKLOCKTIMEVERIFY: u8 = 177;
/// Marks transaction as invalid if the top stack item is less than the transaction's sequence used for relative lock time
pub const OP_CHECKSEQUENCEVERIFY: u8 = 178;
