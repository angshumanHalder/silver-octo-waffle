import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import { NodeGlobalsPolyfillPlugin } from "@esbuild-plugins/node-globals-polyfill";
import wasmPack from "vite-plugin-wasm-pack";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasmPack(["./ring-sig"])],
  server: {
    host: true,
    port: 3000,
  },
  define: {
    "process.env": process.env,
  },
  optimizeDeps: {
    esbuildOptions: {
      define: {
        global: "globalThis",
      },
      plugins: [
        NodeGlobalsPolyfillPlugin({
          buffer: true,
        }),
      ],
    },
  },
});
