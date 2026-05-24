/**
 * DuckHive Provider Management
 * Handles custom provider configuration for LM Studio, MiniMax, OpenRouter, NVIDIA NIM, etc.
 */

const fs = require('fs');
const path = require('path');
const https = require('https');
const http = require('http');

const PROVIDERS_FILE = path.join(__dirname, 'providers.json');

/**
 * @typedef {Object} Provider
 * @property {string} name - Provider name (e.g., "lm-studio", "minimax")
 * @property {string} displayName - Human-readable name
 * @property {string} baseUrl - API base URL
 * @property {string} apiKey - API key (can be empty for local providers)
 * @property {string} modelListUrl - URL to fetch models list (optional)
 * @property {boolean} isActive - Whether this is the currently selected provider
 */

/** @type {Provider[]} */
let providers = [];
let activeProvider = null;

/**
 * Load providers from disk
 * @returns {Provider[]}
 */
function loadProviders() {
  try {
    if (fs.existsSync(PROVIDERS_FILE)) {
      const data = fs.readFileSync(PROVIDERS_FILE, 'utf8');
      providers = JSON.parse(data);
      activeProvider = providers.find(p => p.isActive) || null;
    }
  } catch (err) {
    console.error('Error loading providers:', err.message);
    providers = [];
  }
  return providers;
}

/**
 * Save providers to disk
 */
function saveProviders() {
  try {
    fs.writeFileSync(PROVIDERS_FILE, JSON.stringify(providers, null, 2));
  } catch (err) {
    console.error('Error saving providers:', err.message);
  }
}

/**
 * Fetch models from a provider's API
 * @param {Provider} provider
 * @returns {Promise<string[]>}
 */
async function fetchModelsFromApi(provider) {
  return new Promise((resolve, reject) => {
    const url = new URL(provider.modelListUrl || '/v1/models', provider.baseUrl);
    const options = {
      hostname: url.hostname,
      port: url.port || (url.protocol === 'https:' ? 443 : 80),
      path: url.pathname + url.search,
      method: 'GET',
      headers: {
        'Accept': 'application/json'
      },
      timeout: 10000
    };

    if (provider.apiKey) {
      options.headers['Authorization'] = `Bearer ${provider.apiKey}`;
    }

    const protocol = url.protocol === 'https:' ? https : http;

    const req = protocol.request(options, (res) => {
      let data = '';

      res.on('data', (chunk) => {
        data += chunk;
      });

      res.on('end', () => {
        try {
          const json = JSON.parse(data);

          // OpenAI-compatible format
          if (json.data && Array.isArray(json.data)) {
            resolve(json.data.map(m => m.id || m.id));
          }
          // LM Studio format
          else if (json.models && Array.isArray(json.models)) {
            resolve(json.models.map(m => m.id || m.name));
          }
          // Generic array
          else if (Array.isArray(json)) {
            resolve(json.map(m => typeof m === 'string' ? m : (m.id || m.name)));
          }
          else {
            reject(new Error('Unexpected response format'));
          }
        } catch (e) {
          reject(new Error(`Failed to parse response: ${e.message}`));
        }
      });
    });

    req.on('error', reject);
    req.on('timeout', () => {
      req.destroy();
      reject(new Error('Request timeout'));
    });

    req.end();
  });
}

/**
 * Add a new provider
 * @param {string} name - Short name (e.g., "lm-studio")
 * @param {string} displayName - Human-readable name
 * @param {string} baseUrl - API base URL
 * @param {string} apiKey - API key (optional)
 * @param {string} modelListUrl - Optional custom models endpoint
 */
function addProvider(name, displayName, baseUrl, apiKey = '', modelListUrl = '') {
  loadProviders();

  // Check if provider already exists
  if (providers.some(p => p.name === name)) {
    console.log(`Provider "${name}" already exists. Use /provider update to modify.`);
    return false;
  }

  const provider = {
    name,
    displayName,
    baseUrl: baseUrl.replace(/\/$/, ''), // Remove trailing slash
    apiKey,
    modelListUrl: modelListUrl || '',
    isActive: providers.length === 0 // First provider becomes active
  };

  providers.push(provider);
  saveProviders();

  if (provider.isActive) {
    activeProvider = provider;
  }

  console.log(`Provider "${displayName}" added successfully.`);
  if (provider.isActive) {
    console.log('This is now your active provider.');
  }
  return true;
}

/**
 * Remove a provider
 * @param {string} name
 */
function removeProvider(name) {
  loadProviders();

  const index = providers.findIndex(p => p.name === name);
  if (index === -1) {
    console.log(`Provider "${name}" not found.`);
    return false;
  }

  const wasActive = providers[index].isActive;
  providers.splice(index, 1);

  // If removed provider was active, activate the first remaining
  if (wasActive && providers.length > 0) {
    providers[0].isActive = true;
    activeProvider = providers[0];
    console.log(`Provider "${name}" removed. "${providers[0].displayName}" is now active.`);
  } else if (providers.length === 0) {
    activeProvider = null;
    console.log(`Provider "${name}" removed. No providers remaining.`);
  } else {
    console.log(`Provider "${name}" removed.`);
  }

  saveProviders();
  return true;
}

/**
 * List all configured providers
 */
function listProviders() {
  loadProviders();

  if (providers.length === 0) {
    console.log('No providers configured.');
    console.log('Use /provider add <name> <base_url> [api_key] to add one.');
    return;
  }

  console.log('\nConfigured Providers:\n');
  providers.forEach(p => {
    const marker = p.isActive ? ' [ACTIVE]' : '';
    const apiKeyNote = p.apiKey ? ' (API key set)' : ' (no API key)';
    console.log(`  ${p.name}${marker}`);
    console.log(`    Display: ${p.displayName}`);
    console.log(`    URL: ${p.baseUrl}`);
    console.log(`    API Key: ${apiKeyNote}`);
    if (p.modelListUrl) {
      console.log(`    Models URL: ${p.modelListUrl}`);
    }
    console.log('');
  });
}

/**
 * Select an active provider
 * @param {string} name
 */
function selectProvider(name) {
  loadProviders();

  const provider = providers.find(p => p.name === name);
  if (!provider) {
    console.log(`Provider "${name}" not found.`);
    return false;
  }

  // Deactivate all
  providers.forEach(p => p.isActive = false);
  provider.isActive = true;
  activeProvider = provider;
  saveProviders();

  console.log(`Switched to provider: ${provider.displayName}`);
  return true;
}

/**
 * Get the currently active provider
 * @returns {Provider|null}
 */
function getActiveProvider() {
  if (!activeProvider) {
    loadProviders();
  }
  return activeProvider;
}

/**
 * Show models from a provider
 * @param {string} providerName - Optional provider name, uses active if not specified
 */
async function showModels(providerName = null) {
  loadProviders();

  /** @type {Provider} */
  let provider;

  if (providerName) {
    provider = providers.find(p => p.name === providerName);
    if (!provider) {
      console.log(`Provider "${providerName}" not found.`);
      return;
    }
  } else {
    provider = activeProvider;
    if (!provider) {
      console.log('No active provider. Use /provider select <name> or specify a provider.');
      return;
    }
  }

  console.log(`\nFetching models from ${provider.displayName}...`);
  console.log(`URL: ${provider.modelListUrl || '/v1/models'}\n`);

  try {
    const models = await fetchModelsFromApi(provider);
    if (models.length === 0) {
      console.log('No models found.');
    } else {
      console.log(`Found ${models.length} models:\n`);
      models.forEach(m => console.log(`  - ${m}`));
    }
  } catch (err) {
    console.error(`Error fetching models: ${err.message}`);
    console.log('\nMake sure the provider is running and the URL is correct.');
  }
}

/**
 * Update provider details
 * @param {string} name
 * @param {Object} updates
 */
function updateProvider(name, updates) {
  loadProviders();

  const provider = providers.find(p => p.name === name);
  if (!provider) {
    console.log(`Provider "${name}" not found.`);
    return false;
  }

  if (updates.displayName) provider.displayName = updates.displayName;
  if (updates.baseUrl) provider.baseUrl = updates.baseUrl.replace(/\/$/, '');
  if (updates.apiKey !== undefined) provider.apiKey = updates.apiKey;
  if (updates.modelListUrl !== undefined) provider.modelListUrl = updates.modelListUrl;

  saveProviders();
  console.log(`Provider "${name}" updated.`);
  return true;
}

/**
 * Parse and execute provider command
 * @param {string} input - The command input after "/provider "
 * @returns {Promise<string>} - The response text
 */
async function handleProviderCommand(input) {
  const args = input.trim().split(/\s+/);
  const subcommand = args[0] || '';
  const subArgs = args.slice(1);

  let output = '';

  switch (subcommand) {
    case 'add': {
      // /provider add <name> <displayName> <baseUrl> [apiKey] [modelListUrl]
      if (subArgs.length < 3) {
        output = `Usage: /provider add <name> <displayName> <baseUrl> [apiKey] [modelListUrl]

Example:
  /provider add lm-studio "LM Studio" http://localhost:1234
  /provider add openrouter "OpenRouter" https://openrouter.ai/api/v1 sk-xxx
  /provider add minimax "MiniMax" https://api.minimax.chat/v1 xxx`;
      } else {
        const [name, displayName, baseUrl, apiKey, modelListUrl] = subArgs;
        addProvider(name, displayName, baseUrl, apiKey || '', modelListUrl || '');
        output = 'Done.';
      }
      break;
    }

    case 'list':
    case 'ls': {
      listProviders();
      output = 'Done.';
      break;
    }

    case 'select': {
      if (subArgs.length < 1) {
        output = 'Usage: /provider select <name>';
      } else {
        selectProvider(subArgs[0]);
        output = 'Done.';
      }
      break;
    }

    case 'models':
    case 'models list': {
      const providerName = subArgs[0] || null;
      await showModels(providerName);
      output = 'Done.';
      break;
    }

    case 'remove':
    case 'delete': {
      if (subArgs.length < 1) {
        output = 'Usage: /provider remove <name>';
      } else {
        removeProvider(subArgs[0]);
        output = 'Done.';
      }
      break;
    }

    case 'update': {
      // /provider update <name> [key=value]...
      if (subArgs.length < 2) {
        output = `Usage: /provider update <name> [key=value]...

Keys: displayName, baseUrl, apiKey, modelListUrl

Example:
  /provider update lm-studio apiKey=sk-newkey
  /provider update openrouter displayName="OpenRouter Pro"`;
      } else {
        const name = subArgs[0];
        const updates = {};
        subArgs.slice(1).forEach(pair => {
          const [key, value] = pair.split('=');
          if (key && value !== undefined) {
            updates[key] = value;
          }
        });
        updateProvider(name, updates);
        output = 'Done.';
      }
      break;
    }

    case 'active': {
      const p = getActiveProvider();
      if (p) {
        output = `Active provider: ${p.displayName} (${p.name})\nURL: ${p.baseUrl}`;
      } else {
        output = 'No active provider. Use /provider select <name>.';
      }
      break;
    }

    case 'help':
    case '': {
      output = `DuckHive Provider Management

Usage: /provider <command> [options]

Commands:
  add <name> <displayName> <baseUrl> [apiKey] [modelListUrl]
    Add a new provider
    Examples:
      /provider add lm-studio "LM Studio" http://localhost:1234
      /provider add openrouter "OpenRouter" https://openrouter.ai/api/v1 sk-xxx

  list, ls
    List all configured providers

  select <name>
    Set the active provider

  models [providerName]
    Fetch and display models from the provider's API
    Uses active provider if no name specified

  update <name> [key=value ...]
    Update provider settings
    Keys: displayName, baseUrl, apiKey, modelListUrl

  remove <name>
    Remove a provider

  active
    Show the currently active provider

Examples:
  /provider add minimax "MiniMax" https://api.minimax.chat/v1 sk-xxx
  /provider models
  /provider select lm-studio`;
      break;
    }

    default:
      output = `Unknown subcommand: "${subcommand}". Use /provider help for usage.`;
  }

  return output;
}

module.exports = {
  handleProviderCommand,
  getActiveProvider,
  loadProviders,
  addProvider,
  removeProvider,
  listProviders,
  selectProvider,
  showModels,
  fetchModelsFromApi
};