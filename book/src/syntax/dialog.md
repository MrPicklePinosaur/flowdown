# Dialog and Bookmarks

A fundamental part of writing readable conversations is the ability to break
apart conversations into reusable and contained pieces. In the canvas, this is
accomplished by topics and flows.

Topics allow us to move intent triggers into it's own section (no functional
difference other than organization), whereas flows resemble function calls,
allowing us to package up reusable logic.

In flowdown, we have dialogs and bookmarks.

## Dialogs
Dialogs are analogous to function
calls, they allow us to define reusable blocks of dialog that we can jump to
from anywhere.

```
this is the main dialog

@ welcome

hello welcome to my store!

@ about

my store sells a lot of things

@ contact

contact me!
```
The output is as follows
```
> this is the main dialog
```
Notice how the other dialogs aren't executed. A flowdown conversation has an implicit `main` dialog before any
dialogs are declared. It's also the entry point into the conversation.

Dialogs can be jumped to by using the `->` operator like so:
```
this is the main dialog
-> @welcome

@ welcome
this is the welcome dialog
-> about

@ about
this is the about dialog
```
And our output will be:
```
> this is the main dialog
> this is the welcome dialog
> this is the about dialog
```

When a dialog ends, we entire terminate (if top level dialog), or we return
control to the caller.
```
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
This will output
```
> enter layer1
> enter layer2
> enter layer3
> exit layer3
> exit layer2
> exit layer1
```

## Bookmarks
Bookmarks are quite like html header links. They give us an anchor to jump to,
but don't offer any containment, when the next bookmark starts, we will start
executing it. Bookmarks are specifed by using
an equal sign (`=`) followed by a name for the bookmark. We can jump to
bookmarks by using the `->` operator again, but this time we do not prefix the
identifier with an `@` sign.
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

And of course, bookmarks are scoped, they are local to a dialog.
```
@ dialog 1

    jump to:
    -> bookmark 3 // this will error

    = bookmark 1

    = bookmark 2

@ dialog 2

    = bookmark 2

    = bookmark 3

```
