
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

a good way to think of commands are as 'side effects'

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

**set** (update or define variable)
```
[set $name "daniel"]
```

// TODO not sure if should include this feature (force commands to be on its own line)
Commands can be embedded anywhere inside an utterance. Do note that this
however will break the utterance up into two
```
Actually I wanted to tell you [sound dumroll.wav] that I am your father!
```
will yield
```
> Actually I wanted to tell you
[ sound plays ]
> that I am your father!
```

### escapes and block quotes

since square brackets and curly braces have special uses, to output the
character literally, you must escape them
```
here are square bracket \[\] and here are curly braces \{\}
```

If you find it tiring to have to keep escaping characters, you can use a block
quote to output the text literally:
```
> hey look i can finally use [] {}
> so fun!
> and im on a new line!
```

### tones (WIP)

you can control how the voice assistant outputs the utterance with tone indicators
```
hey! did you eat my cake?!?! # angry
oh my bad, i ate it earlier  # apologetic
```

### conversations and bookmarks

a fundamental part of writing readable conversations is the ability to break
apart conversations into reusable and contained pieces. in the canvas, this is
accomplished by topics and flows.

topics allow us to move intent triggers into it's own section (no functional
difference other than organization), whereas flows resemble function calls,
allowing us to package up reusable logic.

our implementation is as follows.

conversations are analogous to function calls, they allow us to define reusable
blocks of conversation that we can jump to from anywhere.

```
@ welcome

hello welcome to my store!

@ about

my store sells a lot of things

@ contact

contact me!
```
the output is as follows
```
> hello welcome to my store!
```
notice how the other conversations aren't executed. when a conversation ends,
we entire terminate (if top level conversation), or we return control to the
caller.
```
@ layer1

    enter layer1
    -> @layer2
    exit layer1

@ layer2

    enter layer2
    -> @layer3
    exit layer2

@ layer 3

    enter layer3
    exit layer3

```
will output
```
> enter layer1
> enter layer2
> enter layer3
> exit layer3
> exit layer2
> exit layer1
```

bookmarks are quite like html header links. they give us an anchor to jump to,
but don't offer any containment, when the next bookmark starts, we will start
executing it. they are local to a conversation.
```
@ self intro

    my name is pinosaur

    ok we don't actually care about backstory lol
    -> present day

    = backstory
    i have been making stores for over 20 years

    = present day
    i am currently making a store

    = future

    i will continue making stores

```
output:
```
> my name is pinosaur
> ok we don't actually care about backstory lol
> i am currently making a store
> i will continue making stores
```
TODO: nested bookmarks with (==, ===, etc)

and of course, bookmarks are scoped
```
@ conversation 1

    jump to:
    * bookmark 1 -> bookmark 1
    * bookmark 2 -> bookmark 2
    * bookmark 3 -> bookmark 3 // error
    * bookmark 4 -> bookmark 4 // error

    = bookmark 1

    = bookmark 2

@ conversation 2

    = bookmark 2

    = bookmark 3

```

### intents

intents are almost to how topics work, we simply mark the topic with `intent:`
and that block can be jumped to from any `[listen]` command (they behave like
normal topics otherwise)
```
hello, how can i help you today?
[listen]

@ intent: check balance
...

@ intent: deposit money
...

@ intent: withdraw money
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

### if conditions (clunky - need redesign)

when and unless clauses
```
// only execute if condition is true
[when $pizza_type == "pineapple"] i love pineapple pizza!

// only execute if condition if false
[unless $pizza_type == "pineapple"] you should really try pineapple pizza!
```

otherwise/else clause - executes if the previously evaluated condition was false
```
[when $pizza_type == "pineapple"] i love pineapple pizza!
[otherwise] you should really try pineapple pizza!

// alternatively
[unless $pizza_type == "pineapple"] you should really try pineapple pizza!
[otherwise $pizza_type == "pineapple"] i love pineapple pizza!
```

## challenges with design

local testing - code blocks need to be run in our own environment, so it may prove difficult to dev locally

### sample conversations

show off control flow
```
@ welcome

    {good day|welcome back $name|how are you doing $name}!
    welcome to my store!  # excited

    = self introduction

        my name is pinosaur

        skip to any part of my introduction
        * what have i been doing                        -> backstory
        * what i am currently doing                     -> present day
        * what i will do in the future                  -> future
        * store introduction (skip this entire section) -> store introduction

        == backstory
        i have been making stores for over 20 years

        == present day
        i am currently making a store

        == future

        i will continue making stores

    now let's get to the store's introduciton!!!

    = store introduction

        my store is called the store of stores

    what would you like to do now?
    * go to shop -> @shop
    * contact us -> @contact

    welcome back (return from sub-topic)

@ shop

    todays items for sale
    * monday menu -> monday
    * tuesday menu -> tuesday
    * wednesday menu -> wednesday

    = monday
    burgers -> done

    = tuesday
    ramen -> done

    = wednesday
    pasta -> done

    = done

@ contact

```


