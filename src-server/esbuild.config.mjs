/** @type{import("esbuild").BuildOptions} */

import esbuild from 'esbuild'

const define = {}



esbuild
    .build({
        entryPoints: ['./src/server/index.ts'],
        bundle: true,
        minify: true,
        sourcemap: false,
        outfile: './bundle/server.bundle.js',
        platform: 'node',
        target: ['node20.0'],
        logLevel: 'info',
        define,
    })
    .catch(() => process.exit(1))