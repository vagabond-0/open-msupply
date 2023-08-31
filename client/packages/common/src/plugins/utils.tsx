import React from 'react';

/* eslint-disable camelcase */
declare const __webpack_init_sharing__: (shareScope: string) => Promise<void>;
declare const __webpack_share_scopes__: Record<string, unknown>;

type Factory = Promise<() => { default: React.ComponentType }>;

interface loadPluginProps {
  plugin: string;
  url: string;
  module: string;
  scope?: string;
}

type Container = {
  get: (module: string) => Factory;
  init: (shareScope: unknown) => Promise<void>;
};

type PluginModule = { default: React.ComponentType<{ data: unknown }> };

export const fetchPlugin = (url: string, plugin: string): Promise<Container> =>
  new Promise((resolve, reject) => {
    // We define a script tag to use the browser for fetching the plugin js file
    const script = document.createElement('script');
    script.src = url;
    script.onerror = err => {
      const message = typeof err === 'string' ? err : 'unknown';
      reject(new Error(`Failed to fetch remote: ${plugin}. Error: ${message}`));
    };

    // When the script is loaded we need to resolve the promise back to Module Federation
    script.onload = () => {
      // The script is now loaded on window using the name defined within the remote
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const container = window[plugin as any] as unknown as Container;
      if (!container) reject(new Error(`Failed to load plugin: ${plugin}`));

      const proxy = {
        get: async (request: string) => container.get(request),
        init: (scope: unknown) => container.init(scope),
      };
      resolve(proxy);
    };
    // Lastly we inject the script tag into the document's head to trigger the script load
    document.head.appendChild(script);
  });

export const loadPlugin =
  ({ plugin, url, module, scope = 'default' }: loadPluginProps) =>
  async (): Promise<PluginModule> => {
    try {
      // Check if this plugin has already been loaded
      if (!(plugin in window)) {
        // Initializes the shared scope. Fills it with known provided modules from this build and all remotes
        await __webpack_init_sharing__(scope);
        // Fetch the plugin app
        const fetchedContainer = await fetchPlugin(`${url}`, plugin);
        // Initialize the plugin app
        await fetchedContainer.init(__webpack_share_scopes__[scope]);
      }
      // `container` is the plugin app
      // eslint-disable-next-line @typescript-eslint/no-explicit-any
      const container = window[plugin as any] as unknown as Container;
      if (!container)
        throw new Error(`Plugin container not found for ${plugin}`);

      // The module passed to get() must match the `exposes` item in our plugin app's webpack.config
      const factory = await container.get(module);

      // `Module` is the React Component exported from the plugin
      const Module = factory?.();
      if (!Module?.default)
        throw new Error(`Module has no default for plugin ${plugin}`);

      return Module as PluginModule;
    } catch (e) {
      console.error(e);
    }
    return new Promise((_resolve, reject) =>
      reject(new Error(`Failed to load plugin ${plugin}`))
    );
  };
