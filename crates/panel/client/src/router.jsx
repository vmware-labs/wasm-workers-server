// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import {
  Outlet,
  createBrowserRouter,
} from "react-router-dom";

import App from "./app";
import Home from "./routes/home";
import Workers from "./routes/workers";
import Worker from "./routes/worker";

const router = createBrowserRouter([
  {
    path: "/_panel/",
    element: <App />,
    children: [
      {
        index: true,
        element: <Home />,
      },
      {
        path: "workers",
        element: <Outlet />,
        children: [
          {
            index: true,
            element: <Workers />
          },
          {
            path: ":id",
            element: <Worker />
          }
        ]
      },
    ]
  },
]);

export default router;
