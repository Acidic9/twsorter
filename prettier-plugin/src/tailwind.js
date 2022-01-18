const workingDir = process.cwd();
const nodeModulesDir = `${workingDir}/node_modules`;

const resolveConfig = require(`${nodeModulesDir}/tailwindcss/resolveConfig`);
const corePlugins =
  require(`${nodeModulesDir}/tailwindcss/lib/corePlugins`).default;
const processPlugins =
  require(`${nodeModulesDir}/tailwindcss/lib/util/processPlugins`).default;
const path = require("path");
const fs = require("fs");

let tailwindConfig = {};
const tailwindConfigFile = "tailwind.config.js";
const tailwindConfigPath = `${workingDir}/${tailwindConfigFile}`;
try {
  tailwindConfig = require(tailwindConfigPath);
} catch (err) {
  console.warn(
    `[WARN] Could not read ${tailwindConfigFile} file - your custom config will be ignored`
  );
  console.warn(
    'You can specify a custom tailwind config file by creating a twsorter.config.yaml file and adding:\n  tw_config: "tailwind.config.js"\n'
  );
}

// Tailwind config
const config = resolveConfig(tailwindConfigPath);

// Plugins
const plugins = processPlugins(
  [...corePlugins(config), ...(config.plugins || [])],
  config
);

module.exports = {
  config,
  plugins,
};
