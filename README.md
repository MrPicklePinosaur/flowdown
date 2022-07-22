
<div align="center">

# flowdown

lightweight markup language for conversation design

</div>

**flowdown** is a minimal markup language for writing conversations,
specifically for the [voiceflow](https://github.com/voiceflow) platform. It can
be thought of a 'voiceflow programming language' of sorts.

**flowdown** is designed to be both easy to read and write. Here is an example
conversation:
```markdown
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

```

## SETTING UP FOR DEVELOPMENT

First install the git hooks and run other development environment setup.
```
$ just devsetup
```

Then to run
```
$ just run
```

To read the documentation / language specification locally:
```
$ just book
```

## EDITOR TOOLS

**flowdown** comes bundled with a vim plugin, `flowdown-vim`.

To install (for vim-plug users), simply add the line
```
Plug 'MrPicklePinosaur/flowdown', { 'rtp': 'tools/flowdown-vim' }
```

## TODO / PLANNED FEATURES

- [x] vim syntax highlight
- [x] cli
- [x] support for external files (audio, code)
- [ ] project configuration toml
- [ ] linker

## RESOURCES/RESEARCH

some resources that were used in the making of this project.
- [md parser in OCaml](https://github.com/MFP/OcsiBlog/blob/master/simple_markup.ml)
- [peg grammar for md](https://github.com/jgm/peg-markdown/blob/master/markdown_parser.leg)
- [write a parser in rust blog](https://adriann.github.io/rust_parser.html)
- [pest peg grammar for md](https://github.com/kivikakk/comrak/blob/main/src/lexer.pest)
- [learn to build a parser for fun and profit](https://medium.com/code-zen/learn-to-build-a-parser-in-rust-for-fun-and-profit-e22ca0e0ce4c)

alternatives that were considered
- [nom combinator parsing](https://github.com/Geal/nom): too much work (i think)
- [lalrpop](https://github.com/lalrpop/lalrpop): not used since md is not a regular language
- [gnu bison](https://en.wikipedia.org/wiki/GNU_Bison): rather use a rust based tool

