/// <reference types="@sveltejs/kit" />
/// <reference types="@sveltejs/adapter-static" />

import { build, files, version } from "$service-worker";

// Create a unique cache name for this deployment
const CACHE = `cache-${version}`;

// Assets to cache for offline use
const ASSETS = [
  ...build, // /_app/immutable/...
  ...files, // static files
];

// Skip waiting and claim clients immediately
self.addEventListener("install", (event) => {
  async function addFilesToCache() {
    const cache = await caches.open(CACHE);
    await cache.addAll(ASSETS);
  }

  event.waitUntil(addFilesToCache());
  // Force the waiting service worker to become active
  (self as any).skipWaiting();
});

// Remove previous cached data from disk and claim clients
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
  // Claim all clients immediately
  (self as any).clients.claim();
});

// Fetch assets from cache, fallback to network
self.addEventListener("fetch", (event) => {
  // Ignore non-GET requests
  if (event.request.method !== "GET") return;

  // Ignore chrome-extension and other non-http(s) requests
  const url = new URL(event.request.url);
  if (!url.protocol.startsWith("http")) return;

  async function respond() {
    const cache = await caches.open(CACHE);

    // For navigation requests (HTML pages), use network-first strategy
    if (event.request.mode === "navigate") {
      try {
        const response = await fetch(event.request);
        if (response.status === 200) {
          cache.put(event.request, response.clone());
        }
        return response;
      } catch {
        // Network failed, try cache
        const cached = await cache.match(event.request);
        if (cached) return cached;

        // Return cached index.html for SPA routing
        const indexResponse = await cache.match("/");
        if (indexResponse) return indexResponse;

        // Last resort: offline message
        return new Response(
          JSON.stringify({
            error: "You are offline",
            message: "Please check your internet connection and try again."
          }),
          {
            status: 503,
            statusText: "Service Unavailable",
            headers: new Headers({
              "Content-Type": "application/json",
            }),
          }
        );
      }
    }

    // For static assets, use cache-first strategy
    if (ASSETS.includes(url.pathname)) {
      const response = await cache.match(url.pathname);
      if (response) return response;
    }

    // Try the network first, then cache
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

      // Return error for API calls
      if (url.pathname.startsWith("/api")) {
        return new Response(
          JSON.stringify({ error: "Offline", message: "This feature requires an internet connection." }),
          {
            status: 503,
            headers: new Headers({ "Content-Type": "application/json" }),
          }
        );
      }

      // Generic offline response
      return new Response("Offline", {
        status: 503,
        statusText: "Service Unavailable",
      });
    }
  }

  event.respondWith(respond());
});

export {};
