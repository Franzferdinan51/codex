#!/usr/bin/env node
// Unified entry point for the DuckHive CLI.

import { spawn } from "node:child_process";
import { existsSync, realpathSync } from "fs";
import { createRequire } from "node:module";
import path from "path";
import { fileURLToPath } from "url";
import os from "os";

// __dirname equivalent in ESM
const __filename = fileURLToPath(import.meta.url);
const __dirname = path.dirname(__filename);
const require = createRequire(import.meta.url);

// DuckHive Custom Provider Configuration - stored in user's home directory
const DUCKHIVE_HOME = path.join(os.homedir(), '.duckhive');
const PROVIDER_CONFIG_PATH = path.join(DUCKHIVE_HOME, 'providers.json');
const ENCRYPTED_KEYS_FILE = path.join(DUCKHIVE_HOME, '.keys.enc');

const providerConfig = {
  providers: {},
  activeProvider: null,
  activeModel: null
};

// Simple XOR encryption for API keys (basic obfuscation)
const ENCRYPTION_KEY = Buffer.from('DuckHive-Provider-Secret-2024').slice(0, 32);

function encryptKey(key) {
  if (!key) return '';
  const encrypted = Buffer.alloc(key.length);
  for (let i = 0; i < key.length; i++) {
    encrypted[i] = key.charCodeAt(i) ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.length];
  }
  return encrypted.toString('base64');
}

function decryptKey(encrypted) {
  if (!encrypted) return '';
  try {
    const buffer = Buffer.from(encrypted, 'base64');
    const decrypted = Buffer.alloc(buffer.length);
    for (let i = 0; i < buffer.length; i++) {
      decrypted[i] = buffer[i] ^ ENCRYPTION_KEY[i % ENCRYPTION_KEY.length];
    }
    return decrypted.toString();
  } catch {
    return '';
  }
}

// Load provider config from file
function loadProviderConfig() {
  try {
    const { readFileSync } = require('fs');
    if (existsSync(PROVIDER_CONFIG_PATH)) {
      const data = readFileSync(PROVIDER_CONFIG_PATH, 'utf8');
      const loaded = JSON.parse(data);
      Object.assign(providerConfig, loaded);
    }
  } catch (e) {
    // Config file doesn't exist or is invalid, use defaults
  }
}

// Save provider config to file
function saveProviderConfig() {
  try {
    const { writeFileSync, mkdirSync } = require('fs');
    const configDir = path.dirname(PROVIDER_CONFIG_PATH);
    if (!existsSync(configDir)) {
      mkdirSync(configDir, { recursive: true });
    }
    writeFileSync(PROVIDER_CONFIG_PATH, JSON.stringify(providerConfig, null, 2));
  } catch (e) {
    console.error('Failed to save provider config:', e.message);
  }
}

// Save encrypted API keys
function saveApiKeys(apiKeys) {
  try {
    const { writeFileSync, mkdirSync } = require('fs');
    const configDir = path.dirname(ENCRYPTED_KEYS_FILE);
    if (!existsSync(configDir)) {
      mkdirSync(configDir, { recursive: true });
    }
    const encrypted = {};
    for (const [name, key] of Object.entries(apiKeys)) {
      encrypted[name] = encryptKey(key);
    }
    writeFileSync(ENCRYPTED_KEYS_FILE, JSON.stringify(encrypted));
  } catch (e) {
    console.error('Failed to save API keys:', e.message);
  }
}

// Load encrypted API keys
function loadApiKeys() {
  try {
    const { readFileSync } = require('fs');
    if (existsSync(ENCRYPTED_KEYS_FILE)) {
      const data = readFileSync(ENCRYPTED_KEYS_FILE, 'utf8');
      const encrypted = JSON.parse(data);
      const decrypted = {};
      for (const [name, enc] of Object.entries(encrypted)) {
        decrypted[name] = decryptKey(enc);
      }
      return decrypted;
    }
  } catch (e) {
    // File doesn't exist or is invalid
  }
  return {};
}

// Provider templates for quick setup
const PROVIDER_TEMPLATES = {
  lmstudio: {
    name: 'lmstudio',
    description: 'LM Studio (local AI models)',
    baseUrl: 'http://localhost:1234',
    modelListUrl: 'http://localhost:1234/v1/models',
    requiresApiKey: false
  },
  openrouter: {
    name: 'openrouter',
    description: 'OpenRouter (multiple model providers)',
    baseUrl: 'https://openrouter.ai/api',
    modelListUrl: 'https://openrouter.ai/api/v1/models',
    requiresApiKey: true
  },
  nvidia: {
    name: 'nvidia',
    description: 'NVIDIA NIM (NVIDIA hosted models)',
    baseUrl: 'https://integrate.api.nvidia.com/v1',
    modelListUrl: 'https://integrate.api.nvidia.com/v1/models',
    requiresApiKey: true
  },
  minimax: {
    name: 'minimax',
    description: 'MiniMax (Chinese AI provider)',
    baseUrl: 'https://api.minimax.chat/v1',
    modelListUrl: 'https://api.minimax.chat/v1/models',
    requiresApiKey: true
  },
  ollama: {
    name: 'ollama',
    description: 'Ollama (local AI models)',
    baseUrl: 'http://localhost:11434',
    modelListUrl: 'http://localhost:11434/api/tags',
    requiresApiKey: false
  },
 together: {
    name: 'together',
    description: 'Together AI',
    baseUrl: 'https://api.together.xyz/v1',
    modelListUrl: 'https://api.together.xyz/v1/models',
    requiresApiKey: true
  },
  deepseek: {
    name: 'deepseek',
    description: 'DeepSeek',
    baseUrl: 'https://api.deepseek.com/v1',
    modelListUrl: 'https://api.deepseek.com/v1/models',
    requiresApiKey: true
  },
  groq: {
    name: 'groq',
    description: 'Groq (fast inference)',
    baseUrl: 'https://api.groq.com/openai/v1',
    modelListUrl: 'https://api.groq.com/openai/v1/models',
    requiresApiKey: true
  },
  fireworks: {
    name: 'fireworks',
    description: 'Fireworks AI',
    baseUrl: 'https://api.fireworks.ai/v1',
    modelListUrl: 'https://api.fireworks.ai/v1/models',
    requiresApiKey: true
  },
  cerebras: {
    name: 'cerebras',
    description: 'Cerebras (fast inference)',
    baseUrl: 'https://api.cerebras.ai/v1',
    modelListUrl: 'https://api.cerebras.ai/v1/models',
    requiresApiKey: true
  }
};

// Handle /provider command
async function handleProviderCommand(args) {
  const subcommand = args[0];
  loadProviderConfig();

  switch (subcommand) {
    case 'add': {
      // Check for --template flag
      if (args[1] === '--template' || args[1] === '-t') {
        const templateName = args[2];
        if (!templateName) {
          console.log('Usage: /provider add --template <template-name> [apiKey]');
          console.log('');
          console.log('Available templates:');
          Object.entries(PROVIDER_TEMPLATES).forEach(([key, template]) => {
            const req = template.requiresApiKey ? '(API key required)' : '(no API key)';
            console.log(`  ${key.padEnd(12)} ${template.description} ${req}`);
          });
          return;
        }
        const template = PROVIDER_TEMPLATES[templateName];
        if (!template) {
          console.log(`Unknown template '${templateName}'. Use /provider templates to see available options.`);
          return;
        }
        const apiKey = args[3] || '';
        addProvider(template.name, template.baseUrl, apiKey, template.modelListUrl);
        console.log(`Provider '${templateName}' added from template.`);
        console.log(`Base URL: ${template.baseUrl}`);
        console.log(`Model List URL: ${template.modelListUrl}`);
        if (template.requiresApiKey && !apiKey) {
          console.log('');
          console.log(`Use /provider apikey ${template.name} <your-api-key> to set an API key.`);
        }
        return;
      }

      const [name, baseUrl, apiKey, modelListUrl] = args.slice(1);
      if (!name || !baseUrl) {
        console.log('Usage: /provider add <name> <baseUrl> [apiKey] [modelListUrl]');
        console.log('       /provider add --template <template-name> [apiKey]');
        console.log('');
        console.log('Examples:');
        console.log('  /provider add --template lmstudio');
        console.log('  /provider add --template openrouter <your-api-key>');
        console.log('  /provider add --template nvidia <your-api-key>');
        console.log('  /provider add --template minimax <your-api-key>');
        console.log('  /provider add custom http://localhost:1234');
        console.log('');
        console.log('Use /provider templates to see all available provider templates.');
        return;
      }
      addProvider(name, baseUrl, apiKey || '', modelListUrl || '');
      console.log(`Provider '${name}' added successfully.`);
      console.log(`Base URL: ${baseUrl}`);
      console.log(`Model List URL: ${modelListUrl || baseUrl + '/models'}`);
      if (!apiKey) {
        console.log('');
        console.log(`Use /provider apikey ${name} <your-api-key> to set an API key.`);
      }
      break;
    }
    case 'templates':
    case 'template': {
      console.log('Available Provider Templates:');
      console.log('============================');
      Object.entries(PROVIDER_TEMPLATES).forEach(([key, template]) => {
        const req = template.requiresApiKey ? 'API key required' : 'No API key needed';
        console.log(`\n${key}:`);
        console.log(`  Description: ${template.description}`);
        console.log(`  Base URL: ${template.baseUrl}`);
        console.log(`  Models URL: ${template.modelListUrl}`);
        console.log(`  Auth: ${req}`);
        console.log(`  Add: /provider add --template ${key} <api-key-if-needed>`);
      });
      break;
    }
    case 'remove':
    case 'delete': {
      const nameToRemove = args[1];
      if (!nameToRemove) {
        console.log('Usage: /provider remove <name>');
        return;
      }
      removeProvider(nameToRemove);
      console.log(`Provider '${nameToRemove}' removed.`);
      break;
    }
    case 'list': {
      console.log('Configured providers:');
      const providers = Object.keys(providerConfig.providers);
      if (providers.length === 0) {
        console.log('  (none)');
        console.log('');
        console.log('Add a provider:');
        console.log('  /provider add --template <template-name> [api-key]');
        console.log('  /provider add <name> <base-url> [api-key] [model-list-url]');
        console.log('');
        console.log('Use /provider templates to see available templates.');
      } else {
        providers.forEach(name => {
          const isActive = name === providerConfig.activeProvider ? ' (active)' : '';
          const prov = providerConfig.providers[name];
          console.log(`  - ${name}${isActive} -> ${prov.baseUrl}`);
        });
      }
      break;
    }
    case 'use': {
      const nameToUse = args[1];
      if (!nameToUse) {
        console.log('Usage: /provider use <name>');
        console.log('Current active provider:', providerConfig.activeProvider || '(none)');
        return;
      }
      if (setActiveProvider(nameToUse)) {
        console.log(`Switched to provider '${nameToUse}'.`);
        console.log('');
        console.log(`Use /provider models ${nameToUse} to see available models.`);
      } else {
        console.log(`Provider '${nameToUse}' not found.`);
        console.log('Use /provider list to see configured providers.');
      }
      break;
    }
    case 'models': {
      const providerName = args[1] || providerConfig.activeProvider;
      if (!providerName) {
        console.log('No active provider. Use /provider use <name> first.');
        console.log('Or: /provider models <provider-name>');
        return;
      }
      const provider = providerConfig.providers[providerName];
      if (!provider) {
        console.log(`Provider '${providerName}' not found.`);
        return;
      }
      console.log(`Fetching models from '${providerName}'...`);
      console.log(`URL: ${provider.modelListUrl || 'not configured'}`);
      console.log('');
      try {
        const models = await fetchModelsFromProvider(providerName);
        if (models.length === 0) {
          console.log('No models found or unable to parse model list.');
          console.log('The provider may not support model listing or requires authentication.');
        } else {
          console.log(`Available models (${models.length}):`);
          console.log('---');
          models.forEach(model => {
            console.log(`  ${model.id}`);
          });
          console.log('---');
          console.log('');
          console.log(`Use /provider model <model-id> to select a model.`);
        }
      } catch (error) {
        console.log(`Error: ${error.message}`);
        console.log('');
        console.log('Troubleshooting:');
        console.log('  1. Make sure the provider server is running');
        console.log('  2. Check the base URL is correct');
        console.log('  3. If API key is required, use: /provider apikey', providerName, '<key>');
      }
      break;
    }
    case 'apikey': {
      const providerName = args[1];
      const apiKey = args[2];
      if (!providerName) {
        console.log('Usage: /provider apikey <name> [apiKey]');
        console.log('  Set the API key for a provider.');
        return;
      }
      if (!providerConfig.providers[providerName]) {
        console.log(`Provider '${providerName}' not found.`);
        return;
      }
      if (!apiKey) {
        console.log(`Usage: /provider apikey ${providerName} <your-api-key>`);
        console.log(`Current API key: ${getProviderApiKey(providerName) ? '****** (set)' : '(not set)'}`);
        return;
      }
      updateProviderApiKey(providerName, apiKey);
      console.log(`API key for '${providerName}' has been saved.`);
      break;
    }
    case 'model': {
      const modelName = args[1];
      if (!modelName) {
        console.log('Usage: /provider model <model-id>');
        console.log('Current model:', providerConfig.activeModel || '(none)');
        return;
      }
      if (!providerConfig.activeProvider) {
        console.log('No active provider. Use /provider use <name> first.');
        return;
      }
      setActiveModel(modelName);
      console.log(`Model set to '${modelName}' for provider '${providerConfig.activeProvider}'.`);
      console.log('');
      console.log('Provider configuration:');
      console.log(`  Provider: ${providerConfig.activeProvider}`);
      console.log(`  Model: ${modelName}`);
      const apiKey = getProviderApiKey(providerConfig.activeProvider);
      console.log(`  API Key: ${apiKey ? '******' + apiKey.slice(-4) : '(not set)'}`);
      break;
    }
    case 'status': {
      console.log('DuckHive Provider Status');
      console.log('=======================');
      console.log(`Active Provider: ${providerConfig.activeProvider || '(none)'}`);
      console.log(`Active Model: ${providerConfig.activeModel || '(none)'}`);
      if (providerConfig.activeProvider) {
        const provider = providerConfig.providers[providerConfig.activeProvider];
        console.log('');
        console.log('Provider Details:');
        console.log(`  Base URL: ${provider.baseUrl}`);
        console.log(`  Model List URL: ${provider.modelListUrl}`);
        const apiKey = getProviderApiKey(providerConfig.activeProvider);
        console.log(`  API Key: ${apiKey ? '******' + apiKey.slice(-4) : '(not set)'}`);
      }
      break;
    }
    case 'help':
      console.log('DuckHive /provider Command Help');
      console.log('=============================');
      console.log('');
      console.log('Provider Management:');
      console.log('  /provider templates       - Show all available provider templates');
      console.log('  /provider add --template <name> [key] - Add provider from template');
      console.log('  /provider add <name> <url> [key] [url] - Add custom provider');
      console.log('  /provider remove <name>   - Remove a provider');
      console.log('  /provider list            - List configured providers');
      console.log('  /provider use <name>     - Set active provider');
      console.log('');
      console.log('API Key Management:');
      console.log('  /provider apikey <name> [key] - Set or update API key');
      console.log('');
      console.log('Model Selection:');
      console.log('  /provider models [name]  - Fetch and list available models');
      console.log('  /provider model <id>      - Set active model');
      console.log('');
      console.log('Status & Info:');
      console.log('  /provider status          - Show current configuration');
      console.log('  /provider help           - Show this help');
      console.log('');
      console.log('Quick Start:');
      console.log('  1. /provider add --template lmstudio');
      console.log('  2. /provider models lmstudio');
      console.log('  3. /provider model <model-id>');
      console.log('  4. /provider status');
      break;
    default:
      console.log('DuckHive /provider Command');
      console.log('==========================');
      console.log('');
      console.log('Usage: /provider <command> [options]');
      console.log('');
      console.log('Commands:');
      console.log('  templates      Show available provider templates');
      console.log('  add            Add a provider (use --template for quick setup)');
      console.log('  remove         Remove a provider');
      console.log('  list           List configured providers');
      console.log('  use            Set active provider');
      console.log('  apikey         Set API key for a provider');
      console.log('  models         Fetch and list available models');
      console.log('  model          Set active model');
      console.log('  status         Show current provider status');
      console.log('  help           Show help information');
      console.log('');
      console.log('Examples:');
      console.log('  /provider templates');
      console.log('  /provider add --template openrouter <api-key>');
      console.log('  /provider use openrouter');
      console.log('  /provider models');
      console.log('  /provider model gpt-4o');
      console.log('  /provider status');
  }
}

// Initialize provider config
loadProviderConfig();

const PLATFORM_PACKAGE_BY_TARGET = {
  "x86_64-unknown-linux-musl": "@openai/codex-linux-x64",
  "aarch64-unknown-linux-musl": "@openai/codex-linux-arm64",
  "x86_64-apple-darwin": "@openai/codex-darwin-x64",
  "aarch64-apple-darwin": "@openai/codex-darwin-arm64",
  "x86_64-pc-windows-msvc": "@openai/codex-win32-x64",
  "aarch64-pc-windows-msvc": "@openai/codex-win32-arm64",
};

const { platform, arch } = process;

let targetTriple = null;
switch (platform) {
  case "linux":
  case "android":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-unknown-linux-musl";
        break;
      case "arm64":
        targetTriple = "aarch64-unknown-linux-musl";
        break;
      default:
        break;
    }
    break;
  case "darwin":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-apple-darwin";
        break;
      case "arm64":
        targetTriple = "aarch64-apple-darwin";
        break;
      default:
        break;
    }
    break;
  case "win32":
    switch (arch) {
      case "x64":
        targetTriple = "x86_64-pc-windows-msvc";
        break;
      case "arm64":
        targetTriple = "aarch64-pc-windows-msvc";
        break;
      default:
        break;
    }
    break;
  default:
    break;
}

if (!targetTriple) {
  throw new Error(`Unsupported platform: ${platform} (${arch})`);
}

const platformPackage = PLATFORM_PACKAGE_BY_TARGET[targetTriple];
if (!platformPackage) {
  throw new Error(`Unsupported target triple: ${targetTriple}`);
}

const codexBinaryName = process.platform === "win32" ? "codex.exe" : "codex";
const localVendorRoot = path.join(__dirname, "..", "vendor");
const packageBinaryPath = (vendorRoot) =>
  path.join(vendorRoot, targetTriple, "bin", codexBinaryName);
const legacyBinaryPath = (vendorRoot) =>
  path.join(vendorRoot, targetTriple, "codex", codexBinaryName);

function resolveNativePackage(vendorRoot) {
  const packageRoot = path.join(vendorRoot, targetTriple);
  const binaryPath = packageBinaryPath(vendorRoot);
  if (existsSync(binaryPath)) {
    return {
      binaryPath,
      pathDir: path.join(packageRoot, "codex-path"),
    };
  }

  const legacyPath = legacyBinaryPath(vendorRoot);
  if (existsSync(legacyPath)) {
    return {
      binaryPath: legacyPath,
      pathDir: path.join(packageRoot, "path"),
    };
  }

  return null;
}

let nativePackage;
try {
  const packageJsonPath = require.resolve(`${platformPackage}/package.json`);
  nativePackage = resolveNativePackage(
    path.join(path.dirname(packageJsonPath), "vendor"),
  );
} catch {
  nativePackage = resolveNativePackage(localVendorRoot);
}

if (!nativePackage) {
  const packageManager = detectPackageManager();
  const updateCommand =
    packageManager === "bun"
      ? "bun install -g @openai/codex@latest"
      : "npm install -g @openai/codex@latest";
  throw new Error(
    `Missing optional dependency ${platformPackage}. Reinstall DuckHive: ${updateCommand}`,
  );
}

const { binaryPath, pathDir } = nativePackage;

// Use an asynchronous spawn instead of spawnSync so that Node is able to
// respond to signals (e.g. Ctrl-C / SIGINT) while the native binary is
// executing. This allows us to forward those signals to the child process
// and guarantees that when either the child terminates or the parent
// receives a fatal signal, both processes exit in a predictable manner.

function getUpdatedPath(newDirs) {
  const pathSep = process.platform === "win32" ? ";" : ":";
  const existingPath = process.env.PATH || "";
  const updatedPath = [
    ...newDirs,
    ...existingPath.split(pathSep).filter(Boolean),
  ].join(pathSep);
  return updatedPath;
}

/**
 * Use heuristics to detect the package manager that was used to install DuckHive
 * in order to give the user a hint about how to update it.
 */
function detectPackageManager() {
  const userAgent = process.env.npm_config_user_agent || "";
  if (/\bbun\//.test(userAgent)) {
    return "bun";
  }

  const execPath = process.env.npm_execpath || "";
  if (execPath.includes("bun")) {
    return "bun";
  }

  if (
    __dirname.includes(".bun/install/global") ||
    __dirname.includes(".bun\\install\\global")
  ) {
    return "bun";
  }

  return userAgent ? "npm" : null;
}

const additionalDirs = [];
if (existsSync(pathDir)) {
  additionalDirs.push(pathDir);
}
const updatedPath = getUpdatedPath(additionalDirs);

// Check for /provider command before spawning codex
const cliArgs = process.argv.slice(2);
if (cliArgs[0] === '/provider' || cliArgs[0] === 'provider') {
  // Strip the /provider prefix
  const providerArgs = cliArgs.slice(1);
  await handleProviderCommand(providerArgs);
  process.exit(0);
}

// Check for interactive provider setup
if (cliArgs[0] === '--provider-setup') {
  console.log('DuckHive Provider Setup');
  console.log('======================');
  console.log('');
  loadProviderConfig();
  const providers = Object.keys(providerConfig.providers);
  if (providers.length > 0) {
    console.log('Configured providers:');
    providers.forEach(name => {
      const isActive = name === providerConfig.activeProvider ? ' (active)' : '';
      console.log(`  - ${name}${isActive}`);
    });
  } else {
    console.log('No providers configured.');
    console.log('');
    console.log('Add a provider with:');
    console.log('  /provider add <name> <baseUrl> [modelListUrl]');
  }
  process.exit(0);
}

const env = { ...process.env, PATH: updatedPath };
const packageManagerEnvVar =
  detectPackageManager() === "bun"
    ? "CODEX_MANAGED_BY_BUN"
    : "CODEX_MANAGED_BY_NPM";
env[packageManagerEnvVar] = "1";
env.CODEX_MANAGED_PACKAGE_ROOT = realpathSync(path.join(__dirname, ".."));

const child = spawn(binaryPath, cliArgs, {
  stdio: "inherit",
  env,
});

child.on("error", (err) => {
  // Typically triggered when the binary is missing or not executable.
  // Re-throwing here will terminate the parent with a non-zero exit code
  // while still printing a helpful stack trace.
  // eslint-disable-next-line no-console
  console.error(err);
  process.exit(1);
});

// Forward common termination signals to the child so that it shuts down
// gracefully. In the handler we temporarily disable the default behavior of
// exiting immediately; once the child has been signaled we simply wait for
// its exit event which will in turn terminate the parent (see below).
const forwardSignal = (signal) => {
  if (child.killed) {
    return;
  }
  try {
    child.kill(signal);
  } catch {
    /* ignore */
  }
};

["SIGINT", "SIGTERM", "SIGHUP"].forEach((sig) => {
  process.on(sig, () => forwardSignal(sig));
});

// When the child exits, mirror its termination reason in the parent so that
// shell scripts and other tooling observe the correct exit status.
// Wrap the lifetime of the child process in a Promise so that we can await
// its termination in a structured way. The Promise resolves with an object
// describing how the child exited: either via exit code or due to a signal.
const childResult = await new Promise((resolve) => {
  child.on("exit", (code, signal) => {
    if (signal) {
      resolve({ type: "signal", signal });
    } else {
      resolve({ type: "code", exitCode: code ?? 1 });
    }
  });
});

if (childResult.type === "signal") {
  // Re-emit the same signal so that the parent terminates with the expected
  // semantics (this also sets the correct exit code of 128 + n).
  process.kill(process.pid, childResult.signal);
} else {
  process.exit(childResult.exitCode);
}
