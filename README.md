# kifuwarabe-connect-four

WIP

## Run

```shell
cargo run
```

## How to program a connect-four game?

During development, you may need to reproduce the behavior of your computer.  
It is difficult to compare the behavior. Instead, it is useful to get the logs and compare the logs.  
**But logger's difficult to make, so use library.**  

* [x] Step 1. Use logger library.
  * [x] Use casual_logger library at 'Cargo.toml', 'main.rs'.
  * [x] Create the 'log.rs' file.
    * [x] Extend the logger.

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [ ] Step 2. Create the 'look_and_model.rs'.
  * [ ] Piece - "O", "X".
  * [ ] Game result - Win/Draw/Lose.
  * [ ] Position - It's the board.
  * [ ] Search - Computer player search info.

If you want to play immediately, you have the talent of a game creator.  
Being able to control your position means being able to play.  

* [ ] Step 3. 'position.rs'
  * [ ] do_move
  * [ ] undo_move
  * [ ] opponent

Let's enter commands into the computer. Create a command line parser.  

* [ ] Step 4. 'command_line_seek.rs'
  * [ ] Input.
  * [ ] Starts with.
  * [ ] Go next to.
  * [ ] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [ ] Step 5. 'uxi_protocol.rs'
  * [ ] Do. (Before 'From XFEN') Excludes legal moves and winning/losing decisions.
  * [ ] To XFEN.
  * [ ] From XFEN.
  * [ ] Undo.

Let's make a principal command.  

* [ ] Step 6. 'examples/main.rs' command line.
  * [ ] position.
  * [ ] pos.
  * [ ] do.
  * [ ] undo.
  * [ ] uxi.
  * [ ] xfen.

Before you make a computer player, let's judge the outcome. And let's test.  

* [ ] Step 7. 'win_lose_judgment.rs'
  * [ ] Win.
  * [ ] Draw - Not win, not lose, can not play.
  * [ ] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [ ] Step 8. 'performance_measurement.rs'
  * [ ] Seconds. - Stopwatch.
  * [ ] Node per second.

Finally, let's make a computer player.  

* [ ] Step 9. 'computer_player.rs'
  * [ ] Search.
  * [ ] Evaluation - None.
* [ ] 'main.py'
  * [ ] Create "go" command.
* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'
