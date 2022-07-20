# Intents (WIP)

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
