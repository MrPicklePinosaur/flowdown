# Control Flow

## choice

simple button like responses can be modeled by:
```
what's your favorite color?
* red: -> red
* blue: blue is mid
* green: i guess green is an ok color... -> green

= red
yay, my favorite color is red too!

= green
but is it really?
```

## if conditions (clunky - need redesign)

when and unless clauses
```
// only execute if condition is true
[when $pizza_type == "pineapple"] i love pineapple pizza!

// only execute if condition if false
[unless $pizza_type == "pineapple"] you should really try pineapple pizza!
```

otherwise/else clause - executes if the previously evaluated condition was false
```
[when $pizza_type == "pineapple"] i love pineapple pizza!
[otherwise] you should really try pineapple pizza!

// alternatively
[unless $pizza_type == "pineapple"] you should really try pineapple pizza!
[otherwise $pizza_type == "pineapple"] i love pineapple pizza!
```

[if $pizza_type == "pineapple"] -> pineapple
