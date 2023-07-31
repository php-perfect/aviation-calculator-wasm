## About

This repository provides WASM bindings for the Aviation Calculator as well as a build workflow to publish it as a npm
package that can be used in JS projects.

## Usage

### 🛠️ Prod Build

```
wasm-pack build --scope php-perfect
```

### 🔬 Dev Build

```
wasm-pack build --scope php-perfect -- --features console_error_panic_hook
```

### 🔬 Test in Headless Browsers

```
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM

```
wasm-pack publish --access public
```
