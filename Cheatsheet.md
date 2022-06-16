***Original cheatsheet by [Dorbell](https://www.youtube.com/c/DingDongDirt).***  
***Modified and updated by [Blid](https://www.youtube.com/c/BlidDev).***  



# Variables
## set
description: declares a new variable; if it already exists, the value will be overwritten.  
usage: ``set [identifier], [value] ``  
args:  
* ``idetifier``: The name of the new variable
* ``value``: The value that'll be inserted into the variable. Can be a literal (such as `5`) or  a variables value (`set var2, $var1`)  

example: ```set var1, 11```


## op
description: performs an operation on the first specified value.  
uage: `op [variable], [operator], [opvalue]`  
args:  
* `variable`: variable refernce (such as `$var`)
* `operator`: operator string, avilable operators:
    * `+`: plus  
    * `-`: minus  
    * `*`: multiply  
    * `/`: devide   

example: `op $eggs * 7`

=release=
usage: release [variable]
description: release deletes a variable from memory.
example: release $foo

---
# Conditions and navigation


### ifkey
usage: ifkey [SDL Keycode] [scope start] [scope end]
description: ifkey checks if the specified key is held down.
example: ifkey Up 2 2

### ifkey
usage: ifkey [SDL Keycode] [scope start] [scope end]
description: ifkey checks if the specified key is held down.
example: ifkey Up 2 2

---

# Printing
=print=
usage: print [string]
description: print displays text in the console.
example: print hello world!

=out=
usage: out [variable]
description: out prints the specified variable to the console.
example: out $bar

=post=
usage: post
description: post prints the stack to the console.

---

# Graphics / Input Handling
=clear=
usage: clear
description: clear clears all graphics in the window.

=handle_input=
usage: handle_input
description: handle_input handles the user's keyboard input.

=put=
usage: put [x] [y] [colour (0 black|1 white|2 red)]
description: put displays a 10x10 pixel on a 50x50 grid, in the window.
example: put 24 24 2

=display=
usage: display
description: display updates the window's graphics.

=USAGE OF GRAPHICS COMMANDS: EXAMPLE=
1 // Clear the screen
2 clear
3 // Handle input every frame
4 handle_input
5 // Display a red square in the middle
6 put 24 24 2
7 // Update the display
8 display
9 // Continue loop
10 goto 1

---

# Miscellaneous
=rng=
usage: rng [outvar] [range start] [range end] [excluded number]
description: rng sets the value of the specified integer to a random number in the specified range.
example: rng baz 1 10 0
NOTE THE FOLLOWING
- An excluded number is REQUIRED, and can be out of range
- A variable must NOT be specified with the prefix $ in rng; this is the only exception

=goto=
usage: goto [line numver]
description: goto jumps to the specified line number.
example: goto 1

There are several secret commands, bet you can't find them all