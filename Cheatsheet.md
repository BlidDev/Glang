***Original cheatsheet by [Dorbell](https://www.youtube.com/c/DingDongDirt).***  
***Modified and updated by [Blid](https://www.youtube.com/c/BlidDev).***  

## Before reading 
I highly suggest to also check out some of examples in the ``examples`` in order to see the language used in practice 


# Variables

### Glang has 4 variable types
* I32 (discrimination index = 0)
* F32 (discrimination index = 1)
* BOOL (discrimination index = 2)
* STR (discrimination index = 3)

## set
**Description**: Declares a new variable; if it already exists, the value will be overwritten.  
**Usage**: ``set [identifier], [value] ``  
**Args**:  
* ``identifier``: The name of the new variable (literal)
* ``value``: The value that'll be inserted into the variable. Can be a literal (such as `5`) or  a variable's value (`set var2, $var1`)  

**Example**: ```set var1, 11```

## rng
**Description**: Sets a random number from a range to a given variable  
**Usage**: `rng [destination], [start], [end]`  
**Args**:
* `destination`: A string value (literal or variable's value) that point's to the destination variable's name
* `start`: Integer or float (literal or variable's value)
* `end`: Integer or float (literal or variable's value)

**NOTE**: The destination variable's name can be change dynamically.
**Examples**:   

1. `rng var, 1, 10`
2. ```kotlin
    set var1, 0
    set var2, 0
    set name, var1

    //Assign var1 a random number
    rng $name, 0, 10

    //Now lets change it to var2 run the same thing again
    set name, var2
    rng $name, 0, 10

    ```


## op
**Description**: Performs a math operation on the first specified value.  
**Usage**: `op [variable], [operator], [opvalue]`  
**Args**:  
* `variable`: Variable reference (such as `$var`)
* `operator`: Operator string, available operators:
    * `+`: Plus  
    * `-`: Minus  
    * `*`: Multiply  
    * `/`: Divide   
* `opvalue`: Second operation argument (could be a literal or a variable's value)

**Example**: `op $var, *, 7`

## release
**Description**: Deletes a variable from the memory stack.  
**Usage**: `release [variable]`  
**Args**:
* `variable`: Variable reference (such as `$var`)  

**Example**: `release $foo`  

## reset
**Description**: Clears the entire memory stack.  
**Example**: `reset`  

---
# Conditions and navigation

## Labels

**Description**: Creates a reference to a place in a file.  
**Usage**: `#[label_name]:`  
**Args**:
* `label_name`: String literal (cannot be a variable's value) 

**Example**: `#MYLABEL:`  

## goto
**Description**: Makes the program jump to the specified Label.  
**Usage**: `goto [label_name]`  
**Args**:
* `label_name`: String value (literal or a variable's value) 

**Examples**: 
1. `goto MYLABEL`
2. `goto $VarThanContainsTheLabelsName`

## if
**Description**: Checks if the given condition is true.  
**Usage**: ``if [num1], [condition], [num2], [scope range]``  
**NOTE**: Both `num1` and `num2` have to be the same type!  
**Args**:
* `num1`: Left value (literal or variable)
* `condition`: One of:
    * `>`:  Greater than
    * `<`:  Smaller than
    * `>=`: Greater or equal to
    * `<=`: Smaller or equal to
    * `==`: Equals to
    * `!=`: Not equal to
* `num2`: Right value (literal or variable)
* `scope range`: Specifies the length of the condition's scope  (integer literal or variable's value)

**Example**: `if $var, <=, 10, 4`

## ifkey
**Description**: Checks if the specified key is held down.  
**Usage**: ``ifkey [keycode], [scope range]``  
**Args**:
* `keycode`: String value (literal or variable) of a [Device Query keycode](https://docs.rs/device_query/latest/device_query/keymap/enum.Keycode.html)
* `scope range`: Specifies the length of the condition's scope  (integer literal or variable's value)

**Example**: `ifkey Up, 2  `

---

# Printing
## print
**Description**: Displays text in the console.  
**Usage**: `print [string]`
**Args**:
* `string`: String literal (cannot be a variable's value)  

**Example**: `print hello world!\n`

## out
**Description**: Prints the specified value to the console.  
**Usage**: ``out [variable]``  
**Args**: 
* `variable`: Any value (literal or variable's value)  

**Example**: `out $bar`

## post 
**Description**: Prints the whole stack to the console.  
**Usage**: `post`  

---

# Graphics / Input Handling
## init
**Description**: Initializes a new canvas window with the specified arguments.  
**Usage**: ``init [window_width], [window_height], [canvas_width], [canvas_height]``   
**Args**: 
* `[window_width] `: Integer value (literal or variable's value)
* `[window_height]`: Integer value (literal or variable's value)
* `[canvas_width]`: Integer value (literal or variable's value)
* `[canvas_height]`: Integer value (literal or variable's value)

**Example**: `init 848, 480, $sizex, $sizey`

## resize
**Description**: Resizes the canvas.
**Usage**: ``resize [canvas_width], [canvas_height]``   
**Args**: 
* `[canvas_width]`: Integer value (literal or variable's value)
* `[canvas_height]`: Integer value (literal or variable's value)

**Example**: `resize 1, $sizey`


# clear
**Description**: Clears the canvas with the current `clear color`.  
**Usage**: `clear`

**Example**: `clear`

# set_clear
**Description**: Sets the `clear color`.  
**Usage**: `set_clear [color]`
**Args**:
* `color`: [Integer RGB value](https://www.checkyourmath.com/convert/color/rgb_decimal.php) (literal or variable's value)

**Example**: `set_color 3315350`

## handle_input 
**Description**: Handles the graphics window and keyboard input.  
**Usage**: `handle_input`  

**Example**: `handle_input`

## put 
**Description**: Places in the canvas a colored pixel at the given coordinates.
**Usage**: `put [x], [y], [color]`
**Args**:
* `x`: Integer value (literal or variable's value)
* `y`: Integer value (literal or variable's value)
* `color`: [Integer RGB value](https://www.checkyourmath.com/convert/color/rgb_decimal.php) (literal or variable's value)

**Example**: `put 24, $var, 3315350`

## get
**Description**: The reverse of `put`, gets the color from the pixel at the given coordinates and inserts it into a given variable.  
**Usage**: `get [var_name], [x], [y]`  
**Args**:
* `var_name`: The variable to get the [integer RGB value](https://www.checkyourmath.com/convert/color/rgb_decimal.php) (literal)
* `x`: Integer value (literal or variable's value)
* `y`: Integer value (literal or variable's value)

**Example**: `get var1, 2, 2`

## area 
**Description**: Fills a given area in the canvas with colored pixels.  
**Usage**: `area [pos_x], [pos_y], [width], [height], [color]`  
**Args**:
* `pos_x`: Integer value (literal or variable's value)
* `pos_y`: Integer value (literal or variable's value)
* `width`: Integer value (literal or variable's value)
* `height`: Integer value (literal or variable's value)
* `color`: [Integer RGB value](https://www.checkyourmath.com/convert/color/rgb_decimal.php) (literal or variable's value)

**Example**: `area 24, $var, 5, 5 , 3315350`

## display
**Description**: Display updates the window's graphics.  
**Usage**: `display`

**Example**: `display`

## A summery use of most of the graphical functions
```  kotlin
// Initializes a 848x480 window with a 51x51 canvas
init 848, 480, 51, 51

// Set a nice color setup
set NiceBlue, 3315350
set NiceBlack, 3289650

set_clear $NiceBlack

#LOOP:

// Clear the screen
clear

// Handle input every frame
handle_input

// Display a red square in the middle
put 25, 25, $NiceBlue

// Update the display
display

// Continue loop
goto LOOP
```
---


# Miscellaneous

## sleep
**Description**: Makes the thread sleep for a give amount of milliseconds.  
**Usage**: `sleep [millis]`  
**Args**:  
* `millis`: Integer value (literal or variable's value)

**Example**: `sleep 100`

## exit
**Description**: Quits the program.  
**Usage**: `exit [exit_code]`  
**Args**:  
* `exit_code`: Integer value (literal or variable's value)

**Example**: `exit 0`


***There are several secret commands, bet you can't find them all***