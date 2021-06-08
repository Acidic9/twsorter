<!-- [![npm version](https://badge.fury.io/js/prettier-plugin-tailwind.svg)](https://badge.fury.io/js/prettier-plugin-tailwind) -->

<h1 align="center">TwSorter</h1>

<div align="center">
	A cli to sort tailwind classes in your source code.
</div>

<br>

Supports

- HTML
- ReactJS (JSX, TSX)
- VueJS
- Custom (uses regex patterns)

**Go from this:**

```html
<div class="z-50 z-10 container  text-left md:text-center justify-center">
  ...
</div>
```

**To this:**

```html
<div class="container justify-center text-left z-10 z-50 md:text-center">
  ...
</div>
```

This plugin reads your `tailwind.config.js` to sort tailwind classes in your project.

## Installation VSCode

_Coming soon. For now, you can clone the project and build manually._

**Other IDE's are supported.**

_Coming soon. For now, you can clone the project and build manually._

## Configuration

The following options can be put in a `twsorter.config.yaml` file in the root of your project.

- **`tw_config`** - Path to tailwind config relative to the root of the project. Defaults to: `tailwind.config.js`
- **`files`** - List of glob patterns to search and sort tailwind classes in. Defaults to: `./src/**/*`
- **`patterns`** - List of regex patterns to sort classes in. Sorts anything within the first capture group. Defaults to: `class(?:Name)?=["'](.*)["']`

## How does it work?

Well it's written in Rust, and simply uses regex patterns to sort a list of classes. The default regex pattern matches basic HTML and React classes: `class="a b c"`, `className='a b c'`. This can be configured using the `patterns` option in the config file.

## Road map

- [x] Beta version with basic functionality
- [ ] More regex patterns
- [ ] More configuration options

## Contributing 🙌

Contributions are more than welcome. If you see any changes fit, go ahead and open an issue or PR.

---

Any support is a huge motivation, thank you very much!

<a href="https://www.buymeacoffee.com/ariseyhun" target="_blank"><img src="https://cdn.buymeacoffee.com/buttons/v2/default-orange.png" alt="Buy Me A Coffee" height="32" width="140"></a>
