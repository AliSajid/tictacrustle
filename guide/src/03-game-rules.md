# Game Rules

This document explains the rules of Tic-Tac-Toe as implemented in Tic-Tac-Rustle.

## Basic Rules

1. **Players**: Two players, X and O
2. **Goal**: Get three of your marks in a row (horizontal, vertical, or diagonal)
3. **Turns**: X goes first. Players alternate turns.
4. **Win**: First to get 3 marks in a row wins
5. **Draw**: If the board fills without a winner, it's a draw

## Board Representation

```
┌───┬───┬───┐
│ X │ O │ - │
├───┼───┼───┤
│ - │ - │ X │
├───┼───┼───┤
│ O │ X │ - │
└───┴───┴───┘
```

## State Encoding

Each board state can be encoded as a ternary (base-3) number:

- `0` = Empty
- `1` = X
- `2` = O

Example:

```
Board:
X O -
- - X
O X -

Encoding: 1 2 0 0 1 2 0 1 2
State: 120012012 (ternary)
        4257 (decimal)
```

## Legal States

Out of 19,683 possible 3x3 grids, only 5,478 are legally reachable:

- A player can't make a move if the board is full
- X always has the same or one more mark than O
- Impossible states are pruned at compile time

## Win Detection

A win occurs when:

```
X X X - - - - - -  # Horizontal
- - X - - X - - X  # Vertical
- - - X X X - - -  # Diagonal
```

The win condition is checked after each move.

## MENACE's Moves

MENACE chooses moves based on **decision weights**:

1. Each possible move has a weight
2. Choose the move with highest weight
3. If weights are equal, pick randomly
4. After the game, adjust weights based on outcome

## Special Rules

### Symmetry (MENACE-C)

The classic MENACE treats rotationally equivalent states as identical. This reduces the state space from 19,683 to 5,478.

### Distinct States (MENACE-S)

MENACE-S treats all states as distinct, without symmetry assumptions. This uses more memory but is more explicit.

## Draw Conditions

A draw occurs when:

1. The board is full
2. No player has won
3. The last player made their final move

## Game Over

A game ends when:

- Someone wins (3 in a row)
- The board is full (draw)
- A player makes an illegal move

## Strategy Tips

1. **For X**: Control the center if possible
2. **For O**: Block X's winning moves
3. **Defense first**: Don't give the opponent an easy win
4. **Create forks**: Set up two winning threats simultaneously
