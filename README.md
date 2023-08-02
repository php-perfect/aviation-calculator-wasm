## About

This repository provides WASM bindings for the Aviation Calculator as well as a build workflow to publish it as a npm
package that can be used in JS projects.

## Usage

### 🔬 Dev Build

```
wasm-pack build --dev --scope php-perfect -- --features console_error_panic_hook
```

### 🛠️ Prod Build

```
wasm-pack build --release --scope php-perfect
```

### 🔬 Test in Headless Browsers

```
wasm-pack test --headless --firefox
```
