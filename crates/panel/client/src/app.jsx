// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import Header from './components/header';
import Sidebar from './components/sidebar';
import Content from './components/content';
import { Outlet } from 'react-router-dom';

function App() {
  return (
    <div cds-layout="vertical align:stretch">
      <Header />
      <div cds-layout="horizontal align:vertical-stretch wrap:none">
        <Sidebar />
        <div cds-layout="vertical align:stretch">
          <Content>
            <Outlet />
          </Content>
        </div>
      </div>
    </div>
  )
}

export default App
