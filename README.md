Bun includes a TypeScript -> JavaScript transpiler, which is used
to power the runtime (run `.ts` files directly). It can be run in isolation
with the CLI command `bun build ./input.ts` (default output to stdout).

The bug comes from transpiling TypeScript enums inside of namespaces.
JavaScript has neither `enum` nor `namespace`, so it's the transpiler's job
to lower this syntax into valid JavaScript.

Given the input:

```ts
namespace Foo {
  export enum Bar {}
}
```

Bun currently outputs the following, which when run throws a syntax error:

```ts
var Foo;
(Foo => {
  export let Bar; // <-- SyntaxError: Unexpected keyword 'export'
  (Bar2 => {})((Bar = Foo.Bar ||= {}));
})((Foo ||= {}));
```

It is worth noting that parser/transpiler in Bun was originally written by porting [esbuild][1] to
Zig, with many functions names being identical, down to copying some of their
comments. Esbuild does not have this bug.

Bun can be rebuild with `ninja -Cbuild/debug`, where it is available as `./build/debug/bun-debug`.
VSCode's debugger has been configured to run `bun-debug build` on the sample
file for you.

[1]: https://github.com/evanw/esbuild
