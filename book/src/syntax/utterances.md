# Utterances

The most basic 'program' is just text:
```
hello world!
```

Curly braces represent special text actions, we will break down a couple.
Firstly, variables can be incorporated by prefixing an identifier with a dollar
sign
```
hello {$name}, nice to meet you!
```

We can introduce variations on the text by using the pipe `|` character. This
will generate all the appropriate utterance variations for you.
```
{what can I get for|how can i help} you today?
```

## Comments

Single line comments supported with (`//`) and multi line comments with (`/* */`)

<!--
## Escapes and block quotes (don't include)

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
-->

## Tones and Voices

You can control how the voice assistant outputs the utterance with tone
indicators. For example, the voice that is used to say the utterance can be
specified.
```
hello I am the default voice
hello I am a japanese voice # ja-JP-standard-A 
```

