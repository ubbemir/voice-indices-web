# Voice Indices Web
A Counter-Strike 2 demo parser with one purpose: to easily give the users the ability to listen to voice comms only from selected players. It automates the previously manual and tedious process previously described in threads such as this one:
https://www.reddit.com/r/FACEITcom/comments/16vvidt/no_recorded_voice_chat_in_faceit_cs2_demos/

The manual process is also described in this video: https://www.youtube.com/watch?v=8rc-ynAkrXw

## Comparison to other tools
Some other tools simplify the last part of the process which is converting a selection of player slots into a bitmask value for 'tv_listen_voice_indices'. They still require the player to manually first go into the demo, find all players they want to listen to by enumerating 'spec_player x' commands and remembering the corresponding player slots. This tool completely automates this tedious process aswell. 

## How it works
The tool works by parsing the demo file (using the Rust demo parser implementation by @LaihoE) and extracting each and every player's 'slot'. From that data it can then easily then compute the 'tv_listen_voice_indices' value.

A somewhat detailed explanation of the 'tv_listen_voice_indices' convar can be found here: https://www.reddit.com/r/GlobalOffensive/comments/17i3zuc/comment/k6s7fjz/
