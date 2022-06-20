# Glang
## Backstory
Hello and welcome to the Glang git repository. Game Slang or in short Glang is a super simple interpreted language written in Rust with the soul purpose to make pong. After making [a video](https://youtu.be/9JNUzwDLucA) about it, a lot of people requested an update so I made a new version, a little better this time.

### Disclaimer
Glang was made as an experimental side project of mine and by **no means** was meant for serious use other than playing around. It is extremely unstable and limited.



## Libraries I used
* `Unescape`
* ``Pixels``
* ``Device Query``
* ``Beryllium``
* ``Fermium``
* ``Zstring``
* ``Open``



## Installation/Use

### Installing the syntax highlighting Visual Studio Code extension:
1. Grab ``glang-extension.zip`` from the repository or from the [itch.io page](https://blid.itch.io/glang)
2. Extract the ``glang`` folder within the ``.zip`` file into your ``.vscode/extensions`` folder (located in ``C:/Users/USERNAME/.vscode/extensions`` on Windows)
3. Open Visual Studio Code, the extension should appear in your ``installed`` category in the extensions tab and every file ending with ``.glg`` will be highlighted correctly.

### Building from source:
Since Glang code base is written in Rust building it from source should be cross platform as long as you have ``cargo`` and ``cmake`` installed:

**Note**: In all operating systems you'll need the most recent version of both ``Rust/Rustup`` and ``CMake``. I addition you'll also need the ``libx11-dev`` and ``libxv-dev`` packages on **Linux**.

1. Open your OS's terminal and type in ```git git clone https://github.com/BlidDev/Glang.git```.
2. After git finishes cloning the repository ```cd``` into the Glang folder.
3. Type in ```cargo build --release``` (building for the first time will take a bit of time since Rust imports all the libraries)
4. Rust will create a new folder called ``target`` and one called ``release``. Inside release you'll find the ``glang`` executable, you can now use it as mentioned bellow.

### Windows:
1. Get ``glang.exe`` from building the project from source or from downloading and unzipping the [itch.io release](https://blid.itch.io/glang)
2. Run a script by typing in the terminal (in the same directory as the ``exe`` file) ```glang.exe path/to/script.glg```
### Linux:
1. Get ``glang`` from building the project from source or from downloading and unzipping the [itch.io release](https://blid.itch.io/glang)
2. Run a script by typing in the terminal (in the same directory as the program file) ```./glang path/to/script.glg```


--- 

## Example Code
### An example with no graphics:
```kotlin
// This code will generate and print 10 random numbers from 0 to 99

print Hello\tWorld!\n
set i, 0 
set r, 0

#LOOP:
if i, <, 10, 3
    rng r, 0, 100
    out $r
    goto LOOP

print Finished!\n
```

### An example of using graphics:

```kotlin
// This programs draws a blue diagonal line of pixels from (0,0) to (119,119)

init 848,480,212,120
set WHITE, 16777215
set BLUE, 255

set_clear $WHITE
clear

put 119, 119, $BLUE
set i, 0

#FIIL_LOOP:
    get color, $i, $i
    if color, ==, $BLUE,2
        goto GAME_LOOP

#GAME_LOOP:
handle_input
display
goto GAME_LOOP

```


## Documentation
You can find basic documentation of Glang in the attached [cheatsheet](Cheatsheet.md)


## **Thanks for trying out Glang! :)**