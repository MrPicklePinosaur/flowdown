# Commands

Commands are actions that we wish to take that are external to the
conversation. These should read similar to stage directions in a script. A good
way to think of commands are as 'side effects'. Most (if not all) voiceflow
blocks for voice assistants are implemented:

### Audio
The audio file is provided as a url.
```
please wait a moment...
[audio https://audio.com/mysound.wav]
```

### Image
Similar to the audio command, takes in an image url to display.
```
here's a cool picture
[image https://coolpic.com/image.png]
```

### Code
A javascript file can be passed in. It's path is relative to the flowdown file.
```
got it! placing an order for you right now!
[code placeOrder.js]
```

### Exit
Immediately terminate the program
```
goodbye!
[end]
```

### Capture
Prompt user and capture entire result to variable
```
what's your name?
[capture $firstName]
```

<!--
**intent** (listen for intent)
```
what would you like to do today?
[listen]
```
-->

### Set
Update or define variable.
```
[set $name "daniel"]
```
