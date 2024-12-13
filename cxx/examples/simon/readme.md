# simon.cpp

This is a very rough port of the above Simon game to FreeWili.

* Original code is GPL by Giovanni Verrua [here](https://www.hackster.io/giobbino/simon-clone-with-arduino-nano-9edb51).
* Ported to Freewili WASM by William Skellenger (<wskellenger@intrepidcs.com>) October 2024

### Description
The Simon game from the late 1970s/early 1980s was a "follow me" type game.  

* The program will turn on an LED and sound a tone.
* Each color has a unique tone.
* The user has to press the colored button that is directly below the light that turned on with the same color.  (i.e.: Red LED turns on, press the red button)
* If the users pressed the correct color, the program will repeat that color/tone and add a new one.
* The user has to input the same colors that the program did, in the correct sequence.
* If out of sequence, the game is over.
* The time between presses will reduce with each new sequence (the sequence gets faster)

### Interface

When the game starts, you will see menu items above each button:


```
[Start] [Lvl1]  [ - ]  [ - ]  [Exit]
(gra)   (yel)   (grn)  (blu)  (red)
```

* The yellow button will cycle through levels of play, each level requires more correct guesses before losing.
* The gray button will start the game.

### Summary

This is a really rough implementation and likely has some bugs.  It would be a cool variation to trigger RF playback for each tone that turns LED braclets the same color!

The GUI could also use more work.
