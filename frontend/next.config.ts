import type { NextConfig } from "next";

const nextConfig: NextConfig = {
  async redirects() {
    return [
      {
        source: '/',
        destination: '/playground',
        permanent: false, // use true if this should be cached permanently
      },
    ];
  },
};

export default nextConfig;
