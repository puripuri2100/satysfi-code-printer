# satysfi-code-printer

Typeset source code with SATySFi.

# Install

You can install `code-printer` package with [Satyrographos](https://github.com/na4zagin3/satyrographos).

```sh
opam pin add "git+https://github.com/puripuri2100/satysfi-code-printer.git"

opam install satysfi-code-printer

satyrographos install
```

OR

```sh
opam update

opam install satysfi-code-printer

satyrographos install
```

After installation, you can import this package by writing the code in preamble.

```
@require code-printer/code-printer
@require code-printer/code-syntax
@require code-printer/code-theme
```

# Usage

Write the code like this:

```
+p{\inline-code (`inline_code`);}
+code (```let-rec factorial n =
  match n with
  | 0 -> 1
  | _ -> n * (factorial (n - 1))```);
```


When typesetting code with syntax-highlight, write the code like this:

```
+p{\inline-code ?:(CodePrinter.make-config CodeSyntax.rust CodeTheme.basic-light) (`inline_code`);}
+code ?:(
  CodePrinter.make-config CodeSyntax.satysfi CodeTheme.basic-dark
)(```let-rec factorial n =
  match n with
  | 0 -> 1
  | _ -> n * (factorial (n - 1))```);
```

`CodePrinter.make-config` function sets config of syntax and color.

<details>
<summary>List of syntax config constant:</summary>

- `CodeSyntax.rust`
- `CodeSyntax.ocaml`
- `CodeSyntax.satysfi`
- `CodeSyntax.fsharp`
- `CodeSyntax.fortran`
- `CodeSyntax.haskell`
- `CodeSyntax.java`
- `CodeSyntax.javascript`
- `CodeSyntax.julia`
- `CodeSyntax.kotlin`
- `CodeSyntax.lua`
- `CodeSyntax.nim`
- `CodeSyntax.perl`
</details>

<details>
<summary>List of color theme config constant:</summary>

- `CodeTheme.basic-light`
- `CodeTheme.basic-dark`
</details>

# License

This package released under [the MIT license](https://github.com/puripuri2100/satysfi-code-printer/blob/master/LICENSE).

---

(c) 2021 Naoki Kaneko (a.k.a. "puripuri2100")

