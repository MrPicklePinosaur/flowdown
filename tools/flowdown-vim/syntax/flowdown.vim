
" syntax definition
syn match flowdownDialog '@\(\w\|\s\)\+'
syn match flowdownBookmark '=\+\(\w\|\s\)\+'
syn match flowdownJump '->'
syn keyword flowdownCommand contained sound run end capture listen set code
syn match flowdownString '"\(\w\|\s\)\+"' contained
syn match flowdownVariable '\$\w\+' contained
syn region flowdownCommandBlock start="\[" end="\]" transparent contains=flowdownCommand,flowdownString,flowdownVariable

" highlight
hi def link flowdownCommand Statement
hi def link flowdownDialog Function
hi def link flowdownBookmark Macro
hi def link flowdownJump Operator
hi def link flowdownVariable Identifier
hi def link flowdownString String
