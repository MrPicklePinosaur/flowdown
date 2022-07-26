# Control Flow

Simple control flow can be modified using the choice construct. You can think
of each choice line as an if-statement. If the condition before the `:` is
true, the statement will be executed. Note that only one statement is supported
after a condition. If you need a more involved branch, use a dialog and jump to
it, as shown in the 'red' case.
```
What's your favorite color?
[capture $color]
* $color == "red": -> @red
* $color == "blue": [audio blue_da_ba_dee.wav]
* $color == "green": i guess green is an ok color...

@ red
yay, my favorite color is red too!
```
