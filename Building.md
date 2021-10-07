# Building And Running Ellie

Ellie's runtime is not ready but you can see [parsed elements as objects](#example-parser-dump) or you can see [runtime dump](#example-runtime-dump)

```shell
$ cargo build --release
```

Ellie cli should be in ./target/release/ellie

## Info
If you find dump is too complicated use `-dstd` argument to remove standard libraries. But you cannot remove std in runtime dump.

---

## Example parser dump

```ellie
v test = "test string";
```

```shell
$ ellie ./index.ei -rf
```
You can find parser dump as `./ei.json` in terminal path

Output: 

```json
[
    {
        "Variable": {
            "name": "test",
            "dynamic": true,
            "constant": false,
            "public": false,
            "value": {
                "String": {
                    "value": "test string",
                    "comma_start_pos": {
                        "range_start": [
                            0,
                            8
                        ],
                        "range_end": [
                            0,
                            9
                        ]
                    },
                    "comma_end_pos": {
                        "range_start": [
                            0,
                            20
                        ],
                        "range_end": [
                            0,
                            21
                        ]
                    },
                    "value_pos": {
                        "range_start": [
                            0,
                            10
                        ],
                        "range_end": [
                            0,
                            20
                        ]
                    }
                }
            },
            "pos": {
                "range_start": [
                    0,
                    0
                ],
                "range_end": [
                    0,
                    22
                ]
            },
            "name_pos": {
                "range_start": [
                    0,
                    2
                ],
                "range_end": [
                    0,
                    6
                ]
            },
            "value_pos": {
                "range_start": [
                    0,
                    9
                ],
                "range_end": [
                    0,
                    22
                ]
            },
            "type_pos": {
                "range_start": [
                    0,
                    0
                ],
                "range_end": [
                    0,
                    0
                ]
            },
            "rtype": {
                "Generic": {
                    "rtype": "string"
                }
            },
            "hash": "6cdf3a7eb08f5349b1c8c84e23766ca10ad4456aa6c0ca27"
        }
    }
]
```

## Example runtime dump

```ellie
v test = "test string";
```

```shell
$ ellie ./index.ei
```
You can find runtime dump as `./ei.dmp` in terminal path

Output:
```dmp
DUMP:
---

Pages:
	---
	Page 0x01:
		Headers:
			0x0d : test

		Stack:
			5 = 0x70c17ebee755ca13>(0x00)
			5 = 0x153e054bfd17b158>(0x00)
			5 = 0xbc0cae655eee2a5>(0x00)
			5 = 0xeb2e928d6ccaac12>(0x00)
			5 = 0x1f71a7002faf86c4>(0x00)
			5 = 0xebf41986e3ad108d>(0x00)
			5 = 0x2deed82de63dd021>(0x00)
			5 = 0x2407408a9fa9336d>(0x00)
			5 = 0x9c733038d5cbdb19>(0x00)
			5 = 0x8a77507eb262720a>(0x00)
			5 = 0x129361bb0df6393a>(0x00)
			5 = 0x9a434b7cfac85f5b>(0x02, 0x03)
			5 = 0x8ed9b1428c713836>(0x03)
			2 = 0x0d : t(0x00, 0x153e054bfd17b158) : 0x00
		HEAP:
			0x00 : String(0x1fddd751e90)
	---
	Page 0xbc0cae655eee2a5:
		Headers:
			0x00 : int

		Stack:
			1 = 0x00 : 0xbb7392fb7ec6d506 : *
		HEAP:
			EMPTY
	---
	Page 0xcc7959dddba8362:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0x129361bb0df6393a:
		Headers:
			0x00 : nullAble

		Stack:
			1 = 0x00 : 0x67eb77e00bc70639 : 0x00
		HEAP:
			EMPTY
	---
	Page 0x153e054bfd17b158:
		Headers:
			0x00 : string

		Stack:
			1 = 0x00 : 0x6d867e94a1a359a4 : *
		HEAP:
			EMPTY
	---
	Page 0x1f71a7002faf86c4:
		Headers:
			0x00 : collective

		Stack:
			1 = 0x00 : 0x6554f5b5b488bdba : 0x00, 0x01
		HEAP:
			EMPTY
	---
	Page 0x2407408a9fa9336d:
		Headers:
			0x00 : cloak

		Stack:
			1 = 0x00 : 0x7ab6f5c4015258dd : *
		HEAP:
			EMPTY
	---
	Page 0x2deed82de63dd021:
		Headers:
			0x00 : bool

		Stack:
			1 = 0x00 : 0x6cbc544016c2f28b : *
		HEAP:
			EMPTY
	---
	Page 0x460a97d3da539116:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0x482dc7ac524a9c3b:
		Headers:
			0x00 : millisecond
			0x01 : callback

		Stack:
			4 = 0x00 : t(0x00, 0xbc0cae655eee2a5)
			4 = 0x01 : t(0x00, 0xbc0cae655eee2a5)
			5 = 0xbc0cae655eee2a5>?
			5 = 0x70c17ebee755ca13>?
			5 = 0x9a434b7cfac85f5b>?
		HEAP:
			EMPTY
	---
	Page 0x6554f5b5b488bdba:
		Headers:
			0x00 : Key
			0x01 : Value

		Stack:
			6 = 0x00 : 0x00
			6 = 0x01 : 0x01
		HEAP:
			EMPTY
	---
	Page 0x67eb77e00bc70639:
		Headers:
			0x00 : Type

		Stack:
			6 = 0x00 : 0x00
		HEAP:
			EMPTY
	---
	Page 0x6cbc544016c2f28b:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0x6d867e94a1a359a4:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0x70c17ebee755ca13:
		Headers:
			0x00 : void

		Stack:
			1 = 0x00 : 0xb34b2163af078657 : *
		HEAP:
			EMPTY
	---
	Page 0x7ab6f5c4015258dd:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0x8a77507eb262720a:
		Headers:
			0x00 : function

		Stack:
			1 = 0x00 : 0xcc7959dddba8362 : *
		HEAP:
			EMPTY
	---
	Page 0x8ed9b1428c713836:
		Headers:
			0x03 : delay

		Stack:
			5 = 0xbc0cae655eee2a5>?
			5 = 0x70c17ebee755ca13>?
			5 = 0x9a434b7cfac85f5b>?
			0 = 0x03 : <(0x00, t(0x00, 0xbc0cae655eee2a5)), (0x01, t(0x00, 0xbc0cae655eee2a5))> : t(0x00, 0x70c17ebee755ca13) > 5201032687611714619
		HEAP:
			EMPTY
	---
	Page 0x9a434b7cfac85f5b:
		Headers:
			0x00 : seconds
			0x02 : sleep
			0x03 : sleep

		Stack:
			5 = 0xbc0cae655eee2a5>?
			5 = 0x70c17ebee755ca13>?
			7 = 0x02 : <(0x00, t(0x00, 0xbc0cae655eee2a5))> : t(0x00, 0x70c17ebee755ca13)
			7 = 0x03 : <(0x00, t(0x00, 0xbc0cae655eee2a5))> : t(0x00, 0x70c17ebee755ca13)
		HEAP:
			EMPTY
	---
	Page 0x9c733038d5cbdb19:
		Headers:
			0x00 : array

		Stack:
			1 = 0x00 : 0x9dc8b7f4c6e7861a : 0x00
		HEAP:
			EMPTY
	---
	Page 0x9dc8b7f4c6e7861a:
		Headers:
			0x00 : Type

		Stack:
			6 = 0x00 : 0x00
		HEAP:
			EMPTY
	---
	Page 0xb34b2163af078657:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0xbb7392fb7ec6d506:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0xd5c362845e82d9bc:
		Headers:
			EMPTY
		Stack:
			EMPTY
		HEAP:
			EMPTY
	---
	Page 0xeb2e928d6ccaac12:
		Headers:
			0x00 : char

		Stack:
			1 = 0x00 : 0xd5c362845e82d9bc : *
		HEAP:
			EMPTY
	---
	Page 0xebf41986e3ad108d:
		Headers:
			0x00 : float

		Stack:
			1 = 0x00 : 0x460a97d3da539116 : *
		HEAP:
			EMPTY

	---
```