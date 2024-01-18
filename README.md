# QuickTeX

A LaTeX preprocessor to reduce the amount typed 

## Basic Syntax

Use `#` to start a command instead of `\` and `@` instead of `,` in
lists.

For example, `\frac{1}{2}` becomes `#frac(1 @ 2)`

## Examples

```
1 + #bp(#frac(1 @ #sum(i=0 @ n @ #sum(j=0 @ n @ #ang(i, j)))))^{#partial(d^2u @ dx^2)\big|_{x=0}}
```

produces

$$1 +\left( \frac{1}{\sum_{i=0}^{n}\sum_{j=0}^{n}\left\langle i, j \right\rangle} \right)^{\dfrac{\partial d^2u}{\partial dx^2}\big|_{x=0}}$$