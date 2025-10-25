# CalcVM
basic calculator that implements a compiler and virtual runtime in order to demonstrate the compilation pipeline

# VM
the virtual machine implemented to run this code is stack based. and higher order than NISVC. 

# Compiler
the compiler is implemented to parse expressions into stack vm bytecode
given the expression `2(1+3)` 
an output would be
```asm
	push 1
	push 3
	add
	push 2
	mult
	out
```

an example of a more implemented version
```
	x = 1
	f(x) = x^2
	set = [ (x,f(x)) | x <- [0..10] ]
```
would be compiled to
```
	# label f(x)
	push 2
	exp
	# end f(x)
	
	
```

this architecture allows for extensive additions to the fundemental expression parsing, such as variables and control logic.
the vm will (likely) evolve into MagnoliaVM. 

## Instructions
### StackOp
	* `push x -> x`: pushes x to the stack
	* `pop`: destroys a value from top of stack
	* `mov x -> x`: moves a value from offset of x to top of stack
	* `cpy x -> x`: copies a value from offset of x to top of stack
### Math
	* `add x y -> a`: adds x + y
	* `sub x y -> a`: subtracts x - y
	* `mult x y -> a`: multiplies x * y
	* `div x y -> a b`: divides x / y
	* `exp x y -> a`: exponentiates x^2
### IO
	* `out`: print the value at top of stack to stdout
	* `in`:  reads a value from stdin
### Arrays
	an array is used for random access of a series of values. and cannot change in size. (are they actually needed?)
	* `alc x -> rA`: allocates an array of x elements
	* `dalc rA`: deallocates array by array reference handle
	* `acc rA i -> a`: accesses value from array index
	* `ins rA i a`: inserts value at array index
### stacks
	a stack is used for sequential access of a series of values, it can dyanmically change in size 
	(
		destroyed when popping last value? or maybe just internally destroyed and if you reference a stack reference that is not allocated it just builds a new one?
		thus stk is just a record keeper of the next available descriptor? but then that leaks the counter. 
	)
	* `stk -> rS`: initializes a new stack
	* `dstk`: destroys stack
	* `spush rS x`: pushes x to rS stack
	* `spop rS`: destroys value at top of rS stack
	* `smov rS x -> x`: moves a value from offset of x to top of rS stack
	* `scpy rS x -> x`: copies a value from offset of x to top of rS stack
