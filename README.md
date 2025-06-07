
# Rust Embedded CS510

## Homework 1: Game of Life

## Student John Eley

## Program Assignment

You will write a program that interactively plays Game of Life on your MB2.

## Specs:

- The program runs the game at 10 frames per second (updates once per 100ms).
- The program starts with a random board.
- While the A button is held, the board is re-randomized every frame.
- Otherwise, when the B button is not ignored and is pressed, the board is "complemented": every "on" cell is turned "off" and every "off" cell is turned "on". The B button is then ignored for 5 frames (0.5s).
- Otherwise, if the program reaches a state where all cells on the board are off, the program waits 5 frames (0.5s). If it has not received a button press, it then starts with a new random board.

Otherwise, normal Life steps are taken.

### Results:

I initially program initially used Vec arrays and struct to store cell data. It was complicated, but I got it to work OK. I had to learn how to use allocators to create the memory for the vectors. It was a complex way to implement it, but I was initially influlenced by implementations that were designed to work with world of arbitrary sizes. It eventually abandoned this approach. It was getting very complex and hard to understand. I then switched to using Bart's life code. It was a much more elegant way to do it. 

Buttons were pretty straight forward. I found the ignore logic a bit annoying, but is works. 

I embelished the code and added a serial display. To see it, connect to the board with a terminal program set to 115200 8,n,1. The serial display will mirror the LED display. It was a fun extension that helped me learn how to use the serial functions. 

