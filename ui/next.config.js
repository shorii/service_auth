/** @type {import('next').NextConfig} */
const nextConfig = {
  reactStrictMode: true,
  env: {
    API_SERVER: "localhost:10000",
    API_PREFIX: "/api",
  }
}

module.exports = nextConfig
