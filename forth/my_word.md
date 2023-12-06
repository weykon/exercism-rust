## DUP
这个命令用于复制堆栈顶部的元素。执行 DUP 后，堆栈顶部将出现两个相同的元素。例如，如果堆栈顶部的元素是 x，执行 DUP 后，堆栈顶部将有两个 x。
## DROP：
DROP 命令用于删除堆栈顶部的元素。执行此命令后，堆栈顶部的元素将被移除。例如，如果堆栈顶部的元素是 x，执行 DROP 后，x 将被从堆栈中删除。
## SWAP：
SWAP 命令用于交换堆栈顶部的两个元素。执行此命令后，堆栈顶部的两个元素将互换位置。例如，如果堆栈顶部的元素是 x 和 y（y 在顶部），执行 SWAP 后，这两个元素的位置将交换，x 将移至顶部。
## OVER：
OVER 命令用于复制堆栈中第二个元素到堆栈顶部。执行此命令后，堆栈中第二个元素的副本将被放置在堆栈顶部。例如，如果堆栈顶部的元素是 x 和 y（y 在顶部），执行 OVER 后，x 的一个副本将被放置在 y 的上面，即堆栈顶部。

1. Parse Input: You'll need to parse the input to identify numbers, commands, and word definitions.
2. Stack Operations: Implement the basic stack operations for manipulating data.
3. Arithmetic Operations: Implement the arithmetic operations that work on the stack.
4. Word Definitions: Allow defining new words with the specified syntax.
5. Execution: Execute the parsed commands in the correct order.
