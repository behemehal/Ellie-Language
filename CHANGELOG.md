# Changelog

# v0.5.5

- Parser v0.3.1

  - Fix unresolved return type [05a1697137d8f1eed30b833cc86149646c6b344b](https://github.com/behemehal/Ellie-Language/commit/05a1697137d8f1eed30b833cc86149646c6b344b)
  - Implement getter & setter [48ff89ad3e159f175bf7f4435caf4e47605719bc](https://github.com/behemehal/Ellie-Language/commit/48ff89ad3e159f175bf7f4435caf4e47605719bc)
  - Fix faulty compiling sequence [931c9ffa930a19c42ad5186554ffafe18bdf2425](https://github.com/behemehal/Ellie-Language/commit/931c9ffa930a19c42ad5186554ffafe18bdf2425)

- Tokenizer v0.2.2
  - Fix cursor issues [3ea4d25726beae6fcecb00c477358cc6989055a3](https://github.com/behemehal/Ellie-Language/commit/3ea4d25726beae6fcecb00c477358cc6989055a3)
  - [TECHNICAL] Save function's parameter name and type cursor positions separately [5ab7849dd09407baea6a39a47803a3c6e953314b](https://github.com/behemehal/Ellie-Language/commit/5ab7849dd09407baea6a39a47803a3c6e953314b)
  - Fix the situation that ret cant be void. [97e877e5d08ca800e1c3c0394e09a158276a2d50](https://github.com/behemehal/Ellie-Language/commit/97e877e5d08ca800e1c3c0394e09a158276a2d50)
  - [Major] Fix cursor issues with cli. [3ea4d25726beae6fcecb00c477358cc6989055a3](https://github.com/behemehal/Ellie-Language/commit/3ea4d25726beae6fcecb00c477358cc6989055a3)

# v0.6.0

- Parser v0.4.0

  - Implement conditions [0253d6898ed0a1447392652b422cec2759358413](https://github.com/behemehal/Ellie-Language/commit/0253d6898ed0a1447392652b422cec2759358413)
  - Implement for loop [bfdb7d90e5a5cec14d42164634c546b78a804493](https://github.com/behemehal/Ellie-Language/commit/bfdb7d90e5a5cec14d42164634c546b78a804493)

# v0.6.2

- Parser v0.4.2

  - Fix performance issues [c003952cef5df0c111cd33fcf395e92f1d0528d6](https://github.com/behemehal/Ellie-Language/commit/c003952cef5df0c111cd33fcf395e92f1d0528d6)

- Tokenizer v0.3.2
  - Fix performance issues [c003952cef5df0c111cd33fcf395e92f1d0528d6](https://github.com/behemehal/Ellie-Language/commit/c003952cef5df0c111cd33fcf395e92f1d0528d6)

# v0.6.3

- Parser v0.4.3

  - Implement getter setter property resolver [4053165a6dc1a41e2c54cd1cb53eb12449b14e78](https://github.com/behemehal/Ellie-Language/commit/4053165a6dc1a41e2c54cd1cb53eb12449b14e78)

- Tokenizer v0.3.3
  - Fix comment bug [2723381f7f0f32e549c56a4e29a14ac3667c44a0](https://github.com/behemehal/Ellie-Language/commit/2723381f7f0f32e549c56a4e29a14ac3667c44a0)

# v0.7.0

- Parser v0.5.0
  - Implement Enum

- Tokenizer v0.4.0
  - Implement Enum

- ByteCode v0.1.0
  - Implement Transpiler

- VM v0.1.0
  - Implement VM

- Core v0.3.0
  - Implement VM

# v0.7.1

- Parser v0.5.1
  Fix operator priority bugs
- Core v0.4.0
  Add raw types
- VM 0.1.1
  Implement more instructions and raw types
- Tokenizer v0.4.1
  Fix null resolver & not operator conflict

# v0.8.1

- Core v0.5.0
  Split features [5fa7fdf92edf61caedd8bf987c984394ca7e0216](https://github.com/behemehal/Ellie-Language/commit/5fa7fdf92edf61caedd8bf987c984394ca7e0216)
- VM 0.1.1
  Implement heap safety [301f9f91d86be9874e9b06144e1e66a85c958619](https://github.com/behemehal/Ellie-Language/commit/301f9f91d86be9874e9b06144e1e66a85c958619)
- Tokenizer v0.4.4
  Fix null resolver & not operator conflict
- ByteCode v0.1.1
  Fix code order problems [e4af101b8f51d297b23550826559a9c9ceebd516](https://github.com/behemehal/Ellie-Language/commit/e4af101b8f51d297b23550826559a9c9ceebd516)

# v0.8.2

- EllieEngine
  Add tools && local registers [fb7a2797afa9d6b88b781d3120f0b7b0df9fdadb](https://github.com/behemehal/Ellie-Language/commit/fb7a2797afa9d6b88b781d3120f0b7b0df9fdadb)
- Tokenizer v0.4.6
  Fix operator errors [38415a241962103f05ee47090b0a3946ee65bcac](https://github.com/behemehal/Ellie-Language/commit/38415a241962103f05ee47090b0a3946ee65bcac)

# v1.4.13-alpha

- ## ellie_tokenizer v0.5.2

- Add generic usage safety [201c53d0602f4ddcb4348dca1553689f135645a1](https://github.com/behemehal/Ellie-Language/commit/201c53d0602f4ddcb4348dca1553689f135645a1)

- ## ellie_parser v0.7.2

- Fix generics construction on class [5830b180388f5b4ed776c2281bbba0ef3ba17351](https://github.com/behemehal/Ellie-Language/commit/5830b180388f5b4ed776c2281bbba0ef3ba17351)
- Fix function parameter errors due to self calculation [c38269891fa2a6b9295167e820a26d68041c8893](https://github.com/behemehal/Ellie-Language/commit/c38269891fa2a6b9295167e820a26d68041c8893)
- Fix unnecessary calls [4d5a2389b6779a19f2c606fe8b0baa2b6e235c9d](https://github.com/behemehal/Ellie-Language/commit/4d5a2389b6779a19f2c606fe8b0baa2b6e235c9d)
- Fix loop's dependency errors [9d1f7a34c114055fdedc89d10c166604ace6c6a0](https://github.com/behemehal/Ellie-Language/commit/9d1f7a34c114055fdedc89d10c166604ace6c6a0)
- Add generic usage safety [201c53d0602f4ddcb4348dca1553689f135645a1](https://github.com/behemehal/Ellie-Language/commit/201c53d0602f4ddcb4348dca1553689f135645a1)

- ## ellie_fmt v0.5.2

- Implement cloak formatter & fix ret errors [6266f5d7464c1c9d82c78890a6c7901e837268ec](https://github.com/behemehal/Ellie-Language/commit/6266f5d7464c1c9d82c78890a6c7901e837268ec)

- ## ellie_vm v0.5.2

- Fix unnecessary calls & Add clean vm error show [4d5a2389b6779a19f2c606fe8b0baa2b6e235c9d](https://github.com/behemehal/Ellie-Language/commit/4d5a2389b6779a19f2c606fe8b0baa2b6e235c9d)
- Fix static array frame errors [746ee75c7fd74982e4054b158a478704c1a97b96](https://github.com/behemehal/Ellie-Language/commit/746ee75c7fd74982e4054b158a478704c1a97b96)
- Add more info to internal functions [2006dde05bc935b77ba83994317ac865ce8426c7](https://github.com/behemehal/Ellie-Language/commit/2006dde05bc935b77ba83994317ac865ce8426c7)

- ## ellie_bytecode v0.4.2

- Fix loop's dependency errors [9d1f7a34c114055fdedc89d10c166604ace6c6a0](https://github.com/behemehal/Ellie-Language/commit/9d1f7a34c114055fdedc89d10c166604ace6c6a0)

- ## elliec v0.3.1

- Add multiple output to elliec [68e3fb967af1f82b017af492da70f352b558a262](https://github.com/behemehal/Ellie-Language/commit/68e3fb967af1f82b017af492da70f352b558a262)

- Add more info to internal functions [2006dde05bc935b77ba83994317ac865ce8426c7](https://github.com/behemehal/Ellie-Language/commit/2006dde05bc935b77ba83994317ac865ce8426c7)
