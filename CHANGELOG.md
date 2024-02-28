## [1.0.0-next.2](https://github.com/AliSajid/tictacrustle/compare/v1.0.0-next.1...v1.0.0-next.2) (2024-02-28)


### Bug Fixes

* cleanup settings and vale rules ([59fba1e](https://github.com/AliSajid/tictacrustle/commit/59fba1e531750fb0b3949d4a9c71d3b94db39e7c))
* clear warnings in ci scripts ([8f66930](https://github.com/AliSajid/tictacrustle/commit/8f6693089bef5702e03ea9bf2e7e11dc6ca5c925))

## 1.0.0-next.1 (2024-02-26)


### Features

* **board:** add a basic board capable of showing the board state ([5239930](https://github.com/AliSajid/tictacrustle/commit/5239930695eb9268d5d4c45633a7f7e39e438531))
* **board:** add a basic implementation of the game board ([a425882](https://github.com/AliSajid/tictacrustle/commit/a4258821fc28d4aef25920c0cda0a307b7302497))
* complete restructure of the project ([aed0ee1](https://github.com/AliSajid/tictacrustle/commit/aed0ee1cbcd4c33997bca6e8eaca780f7b04e9da))
* **Game:** add the players to the game implementation ([02bcaba](https://github.com/AliSajid/tictacrustle/commit/02bcabae5778e2f695d8475da52bbf018d61df83))
* **GameError:** add string conversion for GameError enum ([fbb9211](https://github.com/AliSajid/tictacrustle/commit/fbb92115aaa2951641e45b384e4bed1ee64e83d6))
* **GameError:** implement Display for GameError ([ed9cdb2](https://github.com/AliSajid/tictacrustle/commit/ed9cdb268d4dacb5992b4900ff4b7bd45bd99399))
* **Gameplay:** add a gameplay struct to encapsulate gameplay ([a1c1174](https://github.com/AliSajid/tictacrustle/commit/a1c11747e993b576bec8459756901d18837a607d))
* **main:** update the main function to use Game instead of Board ([7dc4a6f](https://github.com/AliSajid/tictacrustle/commit/7dc4a6f77b220efa65694517657125fd79e6d55f))
* **mod-components:** add a components module to the application ([02db7e5](https://github.com/AliSajid/tictacrustle/commit/02db7e5f8b6001a8c9c4a5d823f649eb6285c65c))
* **player:** add an independent player trait ([5199a10](https://github.com/AliSajid/tictacrustle/commit/5199a10aeb00454e2351a39e41808c4a240fdce6))
* **Player:** add the player struct to represent the player ([48ed9fb](https://github.com/AliSajid/tictacrustle/commit/48ed9fbebf5f53ba4113f4acd5334c294c58aa5e))
* reorganize the crate as a single binary ([2a77263](https://github.com/AliSajid/tictacrustle/commit/2a77263eb6e8105b1696a98a21d83b3c4fcc6363))
* **square_value:** add an enum to represent the value in any given square ([dfe25ac](https://github.com/AliSajid/tictacrustle/commit/dfe25ac8a83c804320bb508671266eb48d64d0af))
* **square:** add a struct to represent a square on the game board ([9e1d5a6](https://github.com/AliSajid/tictacrustle/commit/9e1d5a6d3d401faa62d571df11f260b025adcfee))
* **tttm:** replace main with a stub function ([6e2775b](https://github.com/AliSajid/tictacrustle/commit/6e2775b657060bf2ba9806d8d2744a6863ccd91f))


### Bug Fixes

* allow dead code for unimplemented parts ([1d99500](https://github.com/AliSajid/tictacrustle/commit/1d995001d58b4e0163f91f26203fe00d74bcf514))
* fix a print warning ([73b3238](https://github.com/AliSajid/tictacrustle/commit/73b3238559cff6edc9604b126454461b85c14c80))
* **GameError:** remove redundant and shadowed implementation of Display ([4a52dc6](https://github.com/AliSajid/tictacrustle/commit/4a52dc62b7eee71cf1eb2023059349acff2ba785))
* **game:** implement game.play() ([50ac86c](https://github.com/AliSajid/tictacrustle/commit/50ac86c2f1f1e6f58abe02d30d78a879cf904ca7))
* **player:** fix an ambiguity bug ([4d57e9b](https://github.com/AliSajid/tictacrustle/commit/4d57e9b836cd8be5bb18953dbfa657d7f736ecb3))
