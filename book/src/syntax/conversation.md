# Conversations and Bookmarks

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
