# Getting Started

## Installation

You can get the binary from the [github
release](https://github.com/MrPicklePinosaur/flowdown/releases). Currently M1
macos and linux are supported. Simply unzip the files, make the binary
executable and place it somewhere in your path.

### Installation example for linux
```
$ wget https://github.com/MrPicklePinosaur/flowdown/releases/download/v0.1/fdc-linux-v0.1.zip
$ unzip fdc-linux-v0.1.zip
$ chmod +x fdc
$ mv fdc /usr/local/bin
```

### Installation example for macos
```
$ wget https://github.com/MrPicklePinosaur/flowdown/releases/download/v0.1/fdc-macos-v0.1.zip
$ unzip fdc-linux-v0.1.zip
$ chmod +x fdc
$ mv fdc /usr/local/bin
```

Packaging for various platforms may or may not be coming in the future.

## Hello World Conversation

Let's compile a basic hello world conversation. The most basic flowdown file is
just a single utterance.
```
$ echo 'Hello world' > hello.fd
$ fdc -o hello.vf hello.fd
```

## Import to Voiceflow

To be able to run the compiled voiceflow conversation, you will need an account
on the [voiceflow platform](https://creator.voiceflow.com/signup).

Once you are setup, navigate to your dashboard and locate the import button on
the top right corner.

![import button](images/import_button.png)

Select and upload the compiled `hello.vf` file, and you should see the project
appear in your dashboard. Running our project, we get our first compiled
conversation!

![run conversation](images/run_conversation.png)
