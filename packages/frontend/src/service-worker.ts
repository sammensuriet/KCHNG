/// <reference types="@sveltejs/kit" />
/// <reference types="@sveltejs/adapter-static" />

import { build, files, version } from "$service-worker";

// Create a unique cache name for this deployment
const CACHE = `cache-${version}`;

const ASSETS = [
  ...build,
  ...files,
];

// Ensure the service worker is registered immediately after page load
self.addEventListener("install", (event) => {
  async function addFilesToCache() {
    const cache = await caches.open(CACHE);
    await cache.addAll(ASSETS);
  }

  event.waitUntil(addFilesToCache());
});

// Remove previous cached data from disk
self.addEventListener("activate", (event) => {
  async function deleteOldCaches() {
    const names = await caches.keys();
    await Promise.all(
      names.map((name) => {
        if (name !== CACHE) return caches.delete(name);
      }),
    );
  }

  event.waitUntil(deleteOldCaches());
});

// Fetch assets from cache, fallback to network
self.addEventListener("fetch", (event) => {
  // Ignore non-GET requests
  if (event.request.method !== "GET") return;

  async function respond() {
    const url = new URL(event.request.url);
    const cache = await caches.open(CACHE);

    // Serve build files from cache
    if (ASSETS.includes(url.pathname)) {
      const response = await cache.match(url.pathname);
      if (response) return response;
    }

    // Try the network first
    try {
      const response = await fetch(event.request);
      if (response.status === 200) {
        cache.put(event.request, response.clone());
      }
      return response;
    } catch {
      // If network fails, try cache
      const response = await cache.match(event.request);
      if (response) return response;

      // If nothing in cache, return offline page
      return new Response("Offline", {
        status: 503,
        statusText: "Service Unavailable",
        headers: new Headers({
          "Content-Type": "text/plain",
        }),
      });
    }
  }

  event.respondWith(respond());
});

export {};
