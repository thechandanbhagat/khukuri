" Vim syntax file
" Language: Khukuri (Nepali Gen-Z Programming Language)
" Maintainer: Khukuri Development Team
" Latest Revision: 5 November 2025

if exists("b:current_syntax")
  finish
endif

" Keywords
syn keyword khuKuriKeyword maanau yedi bhane natra jaba samma pratyek ma kaam pathau bhan rok jane aayaat
syn keyword khuKuriBoolean sahi galat
syn keyword khuKuriOperator ra wa hoina

" Numbers
syn match khuKuriNumber '\v<\d+>'
syn match khuKuriFloat '\v<\d+\.\d+>'

" Strings
syn region khuKuriString start='"' end='"' contained

" Comments
syn match khuKuriComment "//.*$"

" Functions
syn match khuKuriFunction '\v<\w+\ze\('

" Operators
syn match khuKuriOperators "\v(\+|\-|\*|\/|\%|\=\=|\!\=|\<\=|\>\=|\<|\>|\=)"

" Brackets
syn match khuKuriBrackets "[\[\]{}()]"

" Define the default highlighting
hi def link khuKuriKeyword     Keyword
hi def link khuKuriBoolean     Boolean
hi def link khuKuriOperator    Operator
hi def link khuKuriNumber      Number
hi def link khuKuriFloat       Float
hi def link khuKuriString      String
hi def link khuKuriComment     Comment
hi def link khuKuriFunction    Function
hi def link khuKuriOperators   Operator
hi def link khuKuriBrackets    Delimiter

let b:current_syntax = "khukuri"