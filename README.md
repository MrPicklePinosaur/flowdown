
# flowdown

lightweight markup language for conversation design

## RESOURCES/RESEARCH

some resources that were used in the making of this project.
- [md parser in OCaml](https://github.com/MFP/OcsiBlog/blob/master/simple_markup.ml)
- [peg grammar for md](https://github.com/jgm/peg-markdown/blob/master/markdown_parser.leg)
- [write a parser in rust blog](https://adriann.github.io/rust_parser.html)
- [pest peg grammar for md](https://github.com/kivikakk/comrak/blob/main/src/lexer.pest)

alternatives that were considered
- [nom combinator parsing](https://github.com/Geal/nom): too much work (i think)
- [lalrpop](https://github.com/lalrpop/lalrpop): not used since md is not a regular language
- [gnu bison](https://en.wikipedia.org/wiki/GNU_Bison): rather use a rust based tool
