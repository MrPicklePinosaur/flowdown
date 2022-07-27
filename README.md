
<div align="center">

# flowdown

lightweight markup language for conversation design

[![book](https://img.shields.io/badge/book-website-orange)](https://mrpicklepinosaur.github.io/flowdown/)
[![build](https://github.com/MrPicklePinosaur/flowdown/workflows/Release/badge.svg)](https://github.com/MrPicklePinosaur/flowdown/actions)
[![MIT/Apache 2.0](https://img.shields.io/badge/license-MIT%2FApache-blue.svg)](#)

</div>

**flowdown** is a minimal markup language for writing conversations,
specifically for the [voiceflow](https://github.com/voiceflow) platform. It can
be thought of a 'voiceflow programming language' of sorts.

**flowdown** is designed to be both easy to read and write. Here is an example
conversation:
```markdown

Hello! Welcome to Flowdown Pizzaria, what can I do for you?
[capture $mode]
* $mode == "order pizza": -> @order
* $mode == "menu": [image https://flowdownpizza/menu.png]

@ order

    What type of pizza would you like?
    [capture $pizzaType]

    What size of pizza?
    [capture $pizzaSize]

    How would you like to recieve your pizza?
    [capture $pizzaMethod]
    * $pizzaMethod == "delivery": -> @delivery
    * $pizzaMethod == "take out": -> @take out

    Thank you for choosing Flowdown Pizzaria!
    -> @survey

@ delivery

    Can I get an address
    [capture $address]

    // run scripts to calculate final price and estimate delivery time
    [code calculatePrice.js]
    [code computeDeliveryRoute.js]

    Your final price is {$price} and you will get your pizza in about {$deliveryTime}! 

@ take out

    When would you like to pick up your food?
    [capture $pickupTime}

    [code calculatePrice.js]

    Your final price is ${price}.

@ survey

    Would you like to complete an optional survey?
    [capture $survey]
    * $survey == "yes": -> start survey
    * $survey == "no": -> end survey

    = start survey

        How would you rate today's experience?
        [capture $rating]

        Is there any feedback you would like to give?
        [capture $feedback]

    = end survey

        Thank you!

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

**flowdown** comes bundled with a vim plugin. To install (for vim-plug users),
simply add the line
```
Plug 'MrPicklePinosaur/flowdown', { 'rtp': 'tools/vim' }
```

## TODO / PLANNED FEATURES

- [x] vim syntax highlight
- [ ] vscode plugin
- [x] cli
- [x] support for external files (audio, code)
- [ ] project configuration toml
- [ ] linker
- [ ] packages for various platforms

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

