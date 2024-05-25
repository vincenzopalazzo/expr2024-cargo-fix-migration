# expr 2024 migration lint

To see how the migration lints looks a the commit 
in a reverse way.


To reproduce the migration link you can follow the following steps

- 1. Build the following branch of the rust compiler https://github.com/vincenzopalazzo/rust/tree/macros/cargo-fix-expr2024
- 2. Run `./x build` and then link the stage1 `rustup default stage1`
- 3. Clone the https://github.com/vincenzopalazzo/expr2024-cargo-fix-migration.git
- 4. Run `git checkout ef16943f0ea049e5a376aa46593954795acb2c84`
- 5. Run `cargo fix --edition` and then check if the result is the same of the following commit e661ea5ec71c9ecc398f3e2ba7d2e631a4d1f9ec

## License

<div align="center">
  <img src="https://opensource.org/wp-content/uploads/2009/06/OSI_Keyhole.svg" width="150" height="200"/>
</div>

Copyright (c) 2024 Vincenzo Palazzo vincenzopalazzodev@gmail.com

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), 
to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, 
sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, 
FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, 
DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE 
OR OTHER DEALINGS IN THE SOFTWARE.
