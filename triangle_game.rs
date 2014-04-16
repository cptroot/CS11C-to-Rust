use std::io::println;

use triangle_routines::triangle_print;
mod triangle_routines;

static NUM_PEGS:int = 15;
static NUM_MOVES:int = 36;

static moves:[[int, ..3], ..NUM_MOVES] = [
  [0, 1, 3],
  [0, 2, 5],
  [1, 3, 6],
  [1, 4, 8],
  [2, 4, 7],
  [2, 5, 9],
  [3, 1, 0],
  [3, 4, 5],
  [3, 6, 10],
  [3, 7, 12],
  [4, 7, 11],
  [4, 8, 13],
  [5, 2, 0],
  [5, 4, 3],
  [5, 8, 12],
  [5, 9, 14],
  [6, 3, 1],
  [6, 7, 8],
  [7, 4, 2],
  [7, 8, 9],
  [8, 4, 1],
  [8, 7, 6],
  [9, 5, 2],
  [9, 8, 7],
  [10, 6, 3],
  [10, 11, 12],
  [11, 7, 4],
  [11, 12, 13],
  [12, 7, 3],
  [12, 8, 5],
  [12, 11, 10],
  [12, 13, 14],
  [13, 8, 4],
  [13, 12, 11],
  [14, 9, 5],
  [14, 13, 12]
];


fn main() {
  let mut board = [false, ..NUM_PEGS];

  /* Parse the input, assuming valid input */
  triangle_routines::triangle_input(board);

  /* Solve the board */
  let result = solve(board);

  /* If no solution, say so */
  if result == false {
    println("There are no solutions to the initial position given.");
  }
}

/* Return the number of pegs on the board. */
fn num_pegs(board:&[bool]) -> int {
  let mut sum = 0;

  for peg in board.iter() { sum += *peg as int; }
  return sum;
}

/* Return 1 if the move is valid on this board, otherwise return 0. */
fn valid_move(board:&[bool], move:&[int]) -> bool {
  return board[move[0]] && board[move[1]] && (!board[move[2]]);
}

/* Make this move on this board. */
fn make_move(board:&mut [bool], move:&[int]) {
  board[move[0]] = false;
  board[move[1]] = false;
  board[move[2]] = true;
}

/* Unmake this move on this board. */
fn unmake_move(board:&mut [bool], move:&[int]) {
  board[move[0]] = true;
  board[move[1]] = true;
  board[move[2]] = false;
}

/*
 * Solve the game starting from this board.  Return 1 if the game can
 * be solved; otherwise return 0.  Do not permanently alter the board passed
 * in. Once a solution is found, print the boards making up the solution in
 * reverse order.
 */
fn solve(board:&mut [bool]) -> bool {
  /* Check if the board is already solved. If it is, print it. */
  if num_pegs(board) == 1 {
    triangle_print(board);
    return true;
  }


  /* For each move we could make, make it,
   * and then try to solve the resulting board.
   */
  for i in range(0, NUM_MOVES) {
    if !valid_move(board, moves[i]) { continue; }

    make_move(board, moves[i]);

    /* If we find a solution, print the board and return true. */
    if solve(board) {
      unmake_move(board, moves[i]);
      triangle_print(board);
      return true;
    }
    unmake_move(board, moves[i]);
  }

  /* Return false */
  return false;
}
