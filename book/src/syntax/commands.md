# Commands

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
[capture $first_name]
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
