# run
`cargo espflash --monitor`

## About
this code previously had a race condition that could cause the application to crash if both intrerrupts occurred before entering the `WFI` instruction in the thread mode executor. It should work fine now though