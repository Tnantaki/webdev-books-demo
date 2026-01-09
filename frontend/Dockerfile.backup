# FROM oven/bun:1.3.5 as base
# WORKDIR /app

# # Install dependencies (cached layer)
# FROM base as install
# COPY package.json bun.lock ./
# RUN bun install --frozen-lockfile

# # Build application
# FROM base as build
# COPY --from=install /app/node_modules ./node_modules
# COPY . .
# RUN bun run build

# # Production runtime (smallest image)
# FROM oven/bun:1-slim as runtime
# WORKDIR /app

# # Copy built application
# COPY --from=build /app/build ./build
# COPY --from=build /app/package.json .
# COPY --from=build /app/node_modules ./node_modules

# EXPOSE 4173

# CMD ["bun", "./build/index.js"]

FROM oven/bun:1.3.5 as base

COPY package.json bun.lock ./

RUN bun install --frozen-lockfile

COPY . .

RUN bun run build

CMD ["bun", "./build/index.js"]