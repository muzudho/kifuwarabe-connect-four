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

Let's proceed with development while testing.  

* [ ] Step 2. Create the `test.rs` file.
  * Add little by little as you progress through the steps.  

The first thing you have to create is your motive.  
It is important to start with the appearance.  

* [x] Step 3. Create the 'look_and_model.rs'.
  * [x] Piece - "O", "X".
  * [x] Game result - Win/Draw/Lose.
  * [x] Position - It's the board.
  * [x] Search - Computer player search info.

If you want to play immediately, you have the talent of a game creator.  
Being able to control your position means being able to play.  

* [x] Step 4. 'position.rs'
  * [x] do_move
  * [x] undo_move
  * [x] opponent

Let's enter commands into the computer. Create a command line parser.  

* [x] Step 5. 'command_line_seek.rs'
  * [x] Starts with.
  * [x] Go next to.
  * [x] Rest.

People who are looking for something 10 minutes a day are looking for something for a week in a year.  
Before creating the game itself, let's first create the replay function. Let's get it for a week.  

* [ ] Step 6. 'uxi_protocol.rs'
  * [ ] Do. (Before 'From XFEN') Excludes legal moves and winning/losing decisions.
  * [ ] To XFEN.
  * [ ] From XFEN.
  * [ ] Undo.

Let's make a principal command.  

* [ ] Step 7. 'examples/main.rs' command line.
  * [ ] position.
  * [ ] pos.
  * [ ] do.
  * [ ] undo.
  * [ ] uxi.
  * [ ] xfen.

Before you make a computer player, let's judge the outcome. And let's test.  

* [ ] Step 8. 'win_lose_judgment.rs'
  * [ ] Win.
  * [ ] Draw - Not win, not lose, can not play.
  * [ ] Lose. - Not win is lose.

Before creating a computer player, let's create a mechanism to measure performance.  

* [ ] Step 9. 'performance_measurement.rs'
  * [ ] Seconds. - Stopwatch.
  * [ ] Node per second.

Finally, let's make a computer player.  

* [ ] Step 10. 'computer_player.rs'
  * [ ] Search.
  * [ ] Evaluation - None.
* [ ] 'main.py'
  * [ ] Create "go" command.
* [ ] Remeve all 'TODO' tasks. Examples: '// TODO Write a code here.'
