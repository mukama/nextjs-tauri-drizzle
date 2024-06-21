import { defineConfig } from "drizzle-kit";

export default defineConfig({
  schema: "./src/db/schema.ts",
  verbose: false,
  strict: true,
  out: "./src-tauri/migrations",
  dialect: "sqlite",
  dbCredentials : {
    url: "sqlite:test.db"
  }
});