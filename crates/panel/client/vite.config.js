// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
    plugins: [react()],
    base: "/_panel/",
    server: {
        proxy: {
            '/_api': 'http://localhost:8080',
        }
    }
})
