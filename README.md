<h1 align="center">Learning Rust</h1>

<p align="center">
  <img
    alt="GitHub top language"
    src="https://img.shields.io/github/languages/top/kauefraga/learning-rust.svg"
  />
  <img
    alt="Repository size"
    src="https://img.shields.io/github/repo-size/kauefraga/learning-rust.svg"
  />
  <a href="https://github.com/kauefraga/learning-rust/commits/main">
    <img
      alt="GitHub last commit"
      src="https://img.shields.io/github/last-commit/kauefraga/learning-rust.svg"
    />
  </a>
  <img
    alt="GitHub LICENSE"
    src="https://img.shields.io/github/license/kauefraga/learning-rust.svg"
  />
</p>

<h4 align="center">🦀 Trying out new language using Rust 🦀</h4>

## ⬇️ How to download

```bash
git clone https://github.com/kauefraga/learning-rust.git
cd learning-rust

cargo run
```

## 📚 Learned

- [Primitive types](https://doc.rust-lang.org/std/index.html#primitives)
  - Boolean (bool)
  - Integer (i8, i16...)
    - Start counting negative numbers (-128...127)
  - Unsigned Integer (u8, u16...)
    - Start counting by 0 (0...255)
  - Char
- [Variables](https://doc.rust-lang.org/std/keyword.let.html) (constants and statics)
  - In Rust, variables are immutable by default
    - change this by using `mut` like `let mut var...`
  - [Constants](https://doc.rust-lang.org/std/keyword.const.html) are *inlined* (what does it mean?)
    - Don't have space on the stack (memory allocation)
    - All values, parameters assigned with the value of a constant are replaced in compilation time
  - Use `static` keyword to create GLOBAL variables similar to constants
    - [Static](https://doc.rust-lang.org/std/keyword.static.html) values can be mutable (but it is not memory safe)
    - Have memory allocation
  - Scopes exist, like in other languages
    - Anonymous scope
    - *Shadowing*
- Functions
  - Should return something
    - Omit `;` and this expression must be returned
    - Also can use `return` keyword

See this [code](https://github.com/kauefraga/learning-rust/blob/main/src/main.rs)


## 💻 Technologies

- 🦀 [Rust](https://rust-lang.org)

## 📝 License

This project is licensed under the MIT License - See the [LICENSE](https://github.com/kauefraga/learning-rust/blob/main/LICENSE) for more information.

---

<div align="center">
  <img alt="Built with love" src="https://forthebadge.com/images/badges/built-with-love.svg">
  <img alt="Powered by coffee" src="https://forthebadge.com/images/badges/powered-by-coffee.svg">
</div>
