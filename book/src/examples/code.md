# Custom Code

You can provide code snippets for code blocks. Just ensure that the location of
the source code is relative to the flowdown file. Note that you need to forward
declare any variables if the script introduces them.

```
// dToF.fd
Input temperature
[capture $temperature]
[set $converted "0"]
[code convert.js]

temperature in fahrenheight is {$converted}
```

```javascript
// convert.js

// all variables are strings (for now)
const _temp = parseInt(temperature);
converted = (9/5 * _temp) + 32;
```
