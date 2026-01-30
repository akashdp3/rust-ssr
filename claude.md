# Learning Milestones: Build an RSC Framework from Scratch

## Here's a progressive roadmap, each milestone building on the previous:

## Phase 1: Foundations

### Milestone 1: Basic HTTP Server in Rust

- Set up a Rust project with **Axum** and **Tokio**
- Serve static HTML files
- Handle basic routing `/`, `/about`, etc.)
- **Goal**: Understand async Rust and HTTP fundamentals

### Milestone 2: Embed a JavaScript Runtime

- Integrate **Deno Core** (or start simpler with **boa** or **quickjs**)
- Execute JavaScript strings from Rust
- Pass data between Rust ↔ JS (serialization with `serde_json`)
- **Goal**: Bridge Rust and JavaScript worlds

### Milestone 3: ES Module Loading

- Load `.js.ts` files from disk
- Resolve `import` statements
- Build a basic module loader
- **Goal**: Understand how bundlers/runtimes resolve modules

---

## Phase 2: React Integration

### Milestone 4: Server-Side React Rendering

- Bundle React + ReactDOMServer into your runtime
- Render a simple React component to HTML string
- Serve the HTML from your Rust server
- **Goal**: Basic SSR without RSC

### Milestone 5: File-Based Routing

- Scan an `app/` directory for `page.tsx` files
- Map file paths to routes `app/about/page.tsx` → `/about`)
- Dynamic routes with `[param]` folders
- **Goal**: Next.js-style app router basics

### Milestone 6: Client Hydration

- Generate a client bundle (use **esbuild** or **Vite**)
- Inject `<script>` tags into HTML
- Hydrate server-rendered HTML on the client
- **Goal**: Full SSR → hydration cycle

---

## Phase 3: React Server Components

### Milestone 7: RSC Wire Format

- Understand the RSC protocol (JSON-like streaming format)
- Parse RSC payloads: `$` references, `@` promises, component trees
- Serialize React elements to wire format
- **Goal**: Core RSC protocol understanding

### Milestone 8: Server vs Client Components

- Detect `'use client'` directive
- Split components: server-only vs client bundles
- Generate client component references `$L1`, `$L2`)
- **Goal**: The fundamental RSC split

### Milestone 9: Streaming SSR

- Implement chunked HTTP responses
- Stream HTML with `<template>` placeholders for Suspense
- Resolve Suspense boundaries progressively
- **Goal**: Progressive rendering

---

## Phase 4: Production Features

### Milestone 10: Server Actions

- Parse `'use server'` functions
- Generate action IDs and endpoints
- Handle form submissions → server function calls
- **Goal**: Data mutations

### Milestone 11: Hot Module Reloading

- Watch files for changes (use **notify** crate)
- Invalidate changed modules in the runtime
- Push updates to client via WebSocket
- **Goal**: Developer experience

### Milestone 12: Layouts & Loading States

- Nested `layout.tsx` support
- `loading.tsx` for Suspense fallbacks
- `error.tsx` for error boundaries
- **Goal**: App router completeness

---

## Phase 5: Optimization

### Milestone 13: Caching & Performance

- Component-level caching
- Static generation for cacheable routes
- Response compression (gzip/brotli)

### Milestone 14: Image Optimization

- On-demand image resizing
- Format conversion (WebP, AVIF)
- Lazy loading with blur placeholders

---

## Suggested Learning Order

```
1 → 2 → 3 → 4 → 5 → 6  (Get basic SSR working)
        ↓
    7 → 8 → 9          (Add RSC support)
        ↓
   10 → 11 → 12        (Full framework features)
```

**Start with Milestone 1-4** to get a working SSR setup. That alone teaches you 70% of the concepts. RSC (Milestones 7-9) is the hardest part — tackle it once you're comfortable with the basics.
Would you like me to help you get started with Milestone 1? DO NOT WRITE OR SHOW ME CODE
