// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import "./content.scss";

const Content = ({ children }) => (
    <div className="content" cds-layout="vertical gap:md p:xl">
        {children}
    </div>
);

export default Content;
