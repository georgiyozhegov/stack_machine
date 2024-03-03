# Concept 
Pure stack machine. Small number of instructions. There are currently 20 instructions.

# instructions
|Code|Instruction|Pop|Top|Push|Description|
|-|-|-|-|-|-|
|0|`load`|-|-|`value`|loads literal `value`|
|1|`add`|`a`, `b`|-|`result`|`result` = `a` + `b`|
|2|`sub`|`b`, `a`|-|`result`|`result` = `a` - `b`|
|3|`mul`|`a`, `b`|-|`result`|`result` = `a` * `b`|
|4|`div`|`b`, `a`|-|`result`|`result` = `a` / `b`|
|5|`read`|`mode`|-|`input`|reads char (`mode` = `0`) or string (`mode` = `1`) from stdin|
|6|`put`|`format`|`value`|-|prints `value` to stdout as a number (`format` = `0`), char (`format` = `1`) or bits (`format` = `2`)|
|7|`and`|`a`, `b`|-|`result`|`result` = `a` & `b`|
|8|`or`|`a`, `b`|-|`result`|`result` = `a` \| `b`|
|9|`xor`|`a`, `b`|-|`result`|`result` = `a` ^ `b`|
|10|`not`|`value`|-|`result`|`result` = !`value`|
|11|`cmp`|`b`|`a`|`result`|if `a` > `b`: `result` = `1` else if `a` < `b`: `result` = `2` else `result` = `0`|
|12|`jmp`|-|-|-|jumps to `#label`|
|13|`jeq`|`value`|-|-|jumps to `#label` if `value` = `0`|
|14|`jne`|`value`|-|-|jumps to `#label` if `value` != `0`|
|15|`jgr`|`value`|-|-|jumps to `#label` if `value` = `1`|
|16|`jle`|`value`|-|-|jumps to `#label` if `value` = `2`|
|17|`copy`|-|`value`|`value`|makes copy of `value`|
|18|`drop`|`value`|-|-|just drops `value`|
|19|`halt`|-|-|-|stops executing|

# Examples
**2 + 2**
```
#start  ; '#start' label is required
load 2  ; stack = [2]
load 2  ; stack = [2, 2]
add     ; stack = [4]
```

**2 + 2 + 2**
```
#start
load 2  ; stack = [2]
load 2  ; stack = [2, 2]
add     ; stack = [4]
load 2  ; stack = [4, 2]
add     ; stack = [6]
```

**infinite loop**
```
#start
jmp #start
```

**condition**
```
#equal
load "e"
load 1
put
halt

#notequal
load "n"
load 1
put
halt

#start  ; program starts here!
load 2
load 1
cmp 
jeq #equal
jmp #notequal  ; jumps to '#notequal' because 2 != 1
```
# TODO
Add more instructions.
