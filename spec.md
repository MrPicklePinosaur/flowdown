
# flowdown syntax

**principles**
- writing a conversation should be 'text' first, meaning that any functionality
  should be built as syntax around the dialogue.
- a conversation should read as it flows - control flow should be obvious to
  follow and syntax should not be cryptic
- conversations should be fun to write and not at all tedious!

## syntax

### text markup
the most basic 'program' is just text
```
hello world!
```

each spoken utterance continues across new lines - utterances are broken up by
an empty line
```
hello
world
this is still
the same utterance

but this is
a different utterance
```

curly braces represent special text actions, we will break down a couple:

variables can be incorporated by prefixing an identifier with a dollar sign
```
hello {$name}, nice to meet you!
```

we can introduce variations on the text by using the pipe `|` character. this
will generate all the appropriate utterance variations for you.
```
{what can I get for|how can i help} you today?
```

### comments

single line comments supported with (`//`) and multi line comments with (`/* */`)

### commands
commands are actions that we wish to take that are external to the
conversation. these should read similar to stage directions in a script.

here are some voiceflow blocks implemented as commands:

**audio**
```
please wait a moment...
[sound elevator.wav]
```

**code**
```
got it! placing an order for you right now!
[run placeOrder.js]
```

**exit**
```
goodbye!
[end]
```

**capture**
```
what's your name?
[query first_name]
```

**intent** (listen for intent)
```
what would you like to do today?
[listen]
```

### topics and gotos

a fundamental part of writing readable conversations is the ability to break
apart conversations into reusable and contained pieces. in the canvas, this is
accomplished by topics and flows.

topics allow us to move intent triggers into it's own section (no functional
difference other than organization), whereas flows resemble function calls,
allowing us to package up reusable logic.

we will combine these two concepts together.

first we introduce blocks of conversation (name pending) - which are quite
analogous to flows - they allow us to package up conversations into reusable
blocks. a block is indicated with a single equal sign (`=`) followed by the
identifier for the block, a block ends whenever the next one starts.
```
= main
starting the conversation...

= welcome
welcome to my shop!

= goodbye
thanks for visiting!
```

we can also nest blocks like so (whitespace indent optional?)
```
= main
starting the conversation...

  == section 1
  welcome to section 1!

  == section 2
  welcome to section 2!

= welcome
welcome to my shop!

= goodbye
thanks for visiting!
```

however, this may not behave how you think, running this program will yield:
```
starting the conversation...
```
that's because blocks are only declared, and not called (think function
declaration). the use of nested blocks allow scoping for what we will support
next, function calls!
```
= main
starting the conversation...
-> welcome

= welcome
welcome to my shop!
```

these behave how you would expect, upon reaching end of the block, we will jump
back to the caller.

### intents

intents are almost to how topics work, we simply mark the topic with `intent:`
and that block can be jumped to from any `[listen]` command (they behave like
normal topics otherwise)
```
hello, how can i help you today?
[listen]

= intent: check balance
...

= intent: deposit money
...

= intent: withdraw money
...

```

### choice

simple button like responses can be modeled by:
```
what's your favorite color?
* red: -> red
* blue: blue is mid
* green: i guess green is an ok color... -> green

= red
yay, my favorite color is red too!

= green
but is it really?
```

### if conditions

when and unless clauses
```
// only execute if condition is true
[when pizza_type == pineapple] i love pineapple pizza!

// only execute if condition if false
[unless pizza_type == pineapple] you should really try pineapple pizza!
```

otherwise/else clause - executes if the previously evaluated condition was false
```
[when pizza_type == pineapple] i love pineapple pizza!
[otherwise] you should really try pineapple pizza!

// alternatively
[unless pizza_type == pineapple] you should really try pineapple pizza!
[otherwise pizza_type == pineapple] i love pineapple pizza!
```

## challenges with design

local testing - code blocks need to be run in our own environment, so it may prove difficult to dev locally

