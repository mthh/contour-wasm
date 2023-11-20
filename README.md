# Contour-wasm

[Contour](https://crates.io/crates/contour)
and [contour-isobands](https://crates.io/crates/contour-isobands) rust crates,
compiled to WebAssembly, for use in the browser.

## Usage

This module exposes two functions:
- `isobands(data, width, height, intervals[, options])`
- `isolines(data, width, height, intervals[, options])`

### isobands

**With default values**

```js
const module = await wasm_bindgen('./contour_wasm_bg.wasm');

const data = [
  0, 0, 0, 0,
  0, 2, 2, 0,
  0, 2, 2, 0,
  0, 0, 0, 0,
];

const result = module.isobands(data, 4, 4, [0, 1.5]);
```

**With options**

```js
const module = await wasm_bindgen('./contour_wasm_bg.wasm');

const data = [
  0, 0, 0, 0,
  0, 2, 2, 0,
  0, 2, 2, 0,
  0, 0, 0, 0,
];

const options = {
  use_quad_tree: false, // default: true
  x_origin: 22.5, // default: 0
  y_origin: 22.5, // default: 0
  x_step: 45, // default: 1
  y_step: -45, // default: 1
}

const result = module.isobands(data, 4, 4, [0, 1.5], options);
```


### isolines

**With default values**

```js
const module = await wasm_bindgen('./contour_wasm_bg.wasm');

const data = [
  0, 0, 0, 0,
  0, 2, 2, 0,
  0, 2, 2, 0,
  0, 0, 0, 0,
];

const result = module.isolines(data, 4, 4, [0, 1.5]);
```

**With options**

```js
const module = await wasm_bindgen('./contour_wasm_bg.wasm');

const data = [
  0, 0, 0, 0,
  0, 2, 2, 0,
  0, 2, 2, 0,
  0, 0, 0, 0,
];

const options = {
  x_origin: 22.5, // default: 0
  y_origin: 22.5, // default: 0
  x_step: 45, // default: 1
  y_step: -45, // default: 1
}

const result = module.isolines(data, 4, 4, [0, 1.5], options);
```

### Micro-benchmark

**Performance of `isobands` function from this module compared to [MarchingSquaresJS.isoBands](https://github.com/RaumZeit/MarchingSquares.js/)**

| Data size (number of points) | Number of intervals | MarchingSquares.js | contour-wasm |
|------------------------------|---------------------|--------------------|--------------|
| 5307                         | 23                  | 16ms               | **12ms**     |
| 17608                        | 15                  | 25ms               | **15ms**     |
| 336960                       | 14                  | 369ms              | **186ms**    |
| 1010880                      | 14                  | 832ms              | **492ms**    |


## Installation instructions

### 📦 Install with npm

```bash
npm install contour-wasm
```


## Build instructions

> *Only for developers. Otherwise, you can use the npm package.*

### 🛠️ Build with wasm-pack

```bash
wasm-pack build --target no-modules
```

### 🔬 Test in Headless Browsers with wasm-pack test

```bash
wasm-pack test --headless --firefox
```

### 🎁 Publish to NPM with wasm-pack publish

```bash
wasm-pack publish
```

## Demo

Demonstration page is available at https://mthh.github.io/contour-wasm/.