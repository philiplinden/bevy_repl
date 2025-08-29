# Changelog


See [GitHub Releases](https://github.com/philiplinden/bevy_repl/releases) and the [Bevy REPL Book](https://philiplinden.github.io/bevy_repl) for details.

## [0.4.1](https://github.com/philiplinden/bevy_repl/compare/v0.3.0..v0.4.1) - 2025-08-29

### 📚 Documentation

- Add mdbook plugins ([`9b8ebd4`](https://github.com/philiplinden/bevy_repl/commit/9b8ebd43dc2d657436681c62bef6dcee7cd24bc2))
- Introducing the Bevy REPL Book ([`b7710e0`](https://github.com/philiplinden/bevy_repl/commit/b7710e028bd3ad6bcb1c85e39e48acae6d0e0b26))
- Update readme toc ([`e4390a3`](https://github.com/philiplinden/bevy_repl/commit/e4390a3ecf4f3a7e49f902e0c6d0a9efcdd90fc4))
- Update readme ([`43493fc`](https://github.com/philiplinden/bevy_repl/commit/43493fc36b5ea83b9ad0b6b64b7b5ecf8fe5e0c5))
- Add docs for other design experiments ([`b1b2abc`](https://github.com/philiplinden/bevy_repl/commit/b1b2abc013ee539c9655f9a1dd8703a7dba2547e))

### 🧪 Experimental

- Context refactor and better keybinds ([`dd28099`](https://github.com/philiplinden/bevy_repl/commit/dd28099b3dab46a554df1986105996f8db00557a))
- Custom keybinds and cleaner examples ([`2209b9d`](https://github.com/philiplinden/bevy_repl/commit/2209b9ddb43429138fee7e897081d19eab11d65d))
- Add placeholders for help ([`4354571`](https://github.com/philiplinden/bevy_repl/commit/4354571c6de62cc78d70f44d8d0c7f5d120aeba9))
- Remove scrollreadyset, always stdout ([`bed7d47`](https://github.com/philiplinden/bevy_repl/commit/bed7d47147ffe3f7d78d0ed69b23b543efef08dd))
- Move stdout behind feature flag ([`a6460bc`](https://github.com/philiplinden/bevy_repl/commit/a6460bca2ead70517691f120131c71f7795e05e5))
- Make context mgmt better ([`8b0db88`](https://github.com/philiplinden/bevy_repl/commit/8b0db88ad54667d574dfeb219cbe71b9050baed6))
- Remove pretty stuff ([`ce50975`](https://github.com/philiplinden/bevy_repl/commit/ce50975511f960847ca962f6c9096d35785642b8))

### ⚙️ Repository

- *(docs)* Build mdBook on PR, deploy only on tags using mdBook action ([`2e090cd`](https://github.com/philiplinden/bevy_repl/commit/2e090cd8acd32380afbd60a2e0dc18bce9fd5390))
- Use the real github action for pages ([`06030f1`](https://github.com/philiplinden/bevy_repl/commit/06030f1ad0c06b38946cafc7be621f7e8b1fc067))
- Deploy the book on every push to main ([`c4589a2`](https://github.com/philiplinden/bevy_repl/commit/c4589a25f531830ba41b1ab5334dcf6383fe07ac))
- Make the book on every push to main ([`563574c`](https://github.com/philiplinden/bevy_repl/commit/563574cd55ca3028cf9ddb66fd6b1e5f3eb920ed))
- Enable changelog generation ([`82832ca`](https://github.com/philiplinden/bevy_repl/commit/82832cab3f93282713862ad56b557e36af15466c))

## [0.3.0] - 2025-08-14

### 📚 Documentation

- Update readme ([`a958b70`](https://github.com/philiplinden/bevy_repl/commit/a958b705327bd2623a7b98854ec84197ab1d9808))
- Add demo tape to readme ([`ce5a626`](https://github.com/philiplinden/bevy_repl/commit/ce5a62600c6dcc7c4af8d240fc202b7a00da6f06))
- Readability updates ([`55104ba`](https://github.com/philiplinden/bevy_repl/commit/55104ba0625ef91d54c16136e52e053d9abf7283))
- Fancy demo ([`a328508`](https://github.com/philiplinden/bevy_repl/commit/a328508202d64ad721aca2f60fd4186e978e0c38))
- Limitations ([`69b39fb`](https://github.com/philiplinden/bevy_repl/commit/69b39fb3109362e2fb6593032870d4884126fa1b))
- Add known issues to readme ([`3a7dbe1`](https://github.com/philiplinden/bevy_repl/commit/3a7dbe1c7eb931a82a4b755f797e2d92aaea7bef))

### 🐛 Bug Fixes

- Disable the changelog for now ([`ca1d368`](https://github.com/philiplinden/bevy_repl/commit/ca1d36876c9188de896cfc04990fd0ad7d3f6bf8))
- Clean up examples ([`3f6f234`](https://github.com/philiplinden/bevy_repl/commit/3f6f23478cb7fbd01e5b454cc2b9fa93857ff8d5))
- Bad plugin setup ([`a1ebbe2`](https://github.com/philiplinden/bevy_repl/commit/a1ebbe262f5e09933b2085c6acf93c0ff20bbb93))
- Close and quit were misconfigured ([`df104b1`](https://github.com/philiplinden/bevy_repl/commit/df104b1c2d681092473b18b247078bd3cd033c64))
- Remove broken cmds that need world access ([`19aca25`](https://github.com/philiplinden/bevy_repl/commit/19aca254c1bdd550ea37cc8e835d5ae61730bad3))

### 🧪 Experimental

- Disable changelog on main for now ([`9b02990`](https://github.com/philiplinden/bevy_repl/commit/9b02990de3259c45000b042907dcee764eb7ad8f))
- Demo ([`a1ff6b3`](https://github.com/philiplinden/bevy_repl/commit/a1ff6b35814d12e86fea38dd553c1ff1e7a432c0))
- Custom log layer ([`bd50098`](https://github.com/philiplinden/bevy_repl/commit/bd5009857148a6e4d3016c2539a1213aef5e0776))
- Pretty renderer is working ([`3cd37cd`](https://github.com/philiplinden/bevy_repl/commit/3cd37cd56beeb075ec2ef06ffba426b960b3c3b1))
- Repl println macro ([`a7bb78f`](https://github.com/philiplinden/bevy_repl/commit/a7bb78fe151ce3e30f634d93e67bb122611e465b))
- Lots of examples and ergonomic renderer settings ([`7e3cbee`](https://github.com/philiplinden/bevy_repl/commit/7e3cbee95dc8c65a2238c4f4056c618dd125e835))
- Custom renderer ([`ca17531`](https://github.com/philiplinden/bevy_repl/commit/ca17531f035b85ef7ff265ad74815acee74b532d))
- Even more examples ([`e70cf98`](https://github.com/philiplinden/bevy_repl/commit/e70cf9878b71bdbe277cfddfa1c6b983c0b28bf5))
- Overhaul ([`d0bbea2`](https://github.com/philiplinden/bevy_repl/commit/d0bbea2136678e53368615042bd024a6e4b8e075))
- Works with windowed apps too! ([`955262b`](https://github.com/philiplinden/bevy_repl/commit/955262b2e3b1df9d111bc8fc3851bc3f19fb2c06))
- Built-in commands, derive, and examples ([`a781f21`](https://github.com/philiplinden/bevy_repl/commit/a781f21f1222fb4c32c9b9307cff1b6074fd6329))
- Add more examples ([`d455679`](https://github.com/philiplinden/bevy_repl/commit/d4556793a22b81c1774cdc0cc951ae582520fe45))
- Command parsing works!!!! ([`a7edc34`](https://github.com/philiplinden/bevy_repl/commit/a7edc347aa086fe86df68ca95545277dce122ed3))
- Buffer text input and backspace ([`d9df188`](https://github.com/philiplinden/bevy_repl/commit/d9df188f28192f5d195f49c51f1842040da0e066))
- Custom ratatui context ([`fc6bba4`](https://github.com/philiplinden/bevy_repl/commit/fc6bba4f7a055d17baa655232bd9ce259dae3703))
- Drop bevy_crossterm and use bevy_ratatui ([`60231d1`](https://github.com/philiplinden/bevy_repl/commit/60231d1eaec85576ec8de5e6b330f41dba88506a))
- Bevy_crossterm and simplify ([`2c8295e`](https://github.com/philiplinden/bevy_repl/commit/2c8295ea553e6e66f28716d33e740b86c78fbfbc))
- Switch from rustyline to crossterm ([`3bda182`](https://github.com/philiplinden/bevy_repl/commit/3bda18256f6c90c34547ae6345172d4d95a8ffa5))
- Tweaking the derive macros ([`ad8421b`](https://github.com/philiplinden/bevy_repl/commit/ad8421ba2191beb8d994fe792e465b2474aa66c1))
- Cleanup ([`b331b52`](https://github.com/philiplinden/bevy_repl/commit/b331b52685b0ccca1616ce20cb255c5ce6576266))
- Derive ([`2ed17bc`](https://github.com/philiplinden/bevy_repl/commit/2ed17bc2fe412cf78d565dd24f38d7281d4ac138))
- More advanced patterns ([`4df61ee`](https://github.com/philiplinden/bevy_repl/commit/4df61ee8a5455010b1d589bc1fd4d6b258406748))
- More designing through the readme ([`0f7432b`](https://github.com/philiplinden/bevy_repl/commit/0f7432be047520eb1b16e599df585c0cf9ba9994))
- Add rustyline, write out design docs ([`fd4ea0f`](https://github.com/philiplinden/bevy_repl/commit/fd4ea0f9ed8be49b0333468f540dbebf32a5fa49))


### New Contributors ❤️

* @philiplinden made their first contribution
<!-- generated by git-cliff -->
