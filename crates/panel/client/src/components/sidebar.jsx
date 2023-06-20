// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CdsNavigation, CdsNavigationItem } from "@cds/react/navigation";
import { CdsIcon } from "@cds/react/icon";
import { ClarityIcons, cogIcon, cogIconName, fileIcon, fileIconName, homeIcon, homeIconName } from '@cds/core/icon';
import { NavLink } from "react-router-dom";

ClarityIcons.addIcons(cogIcon, fileIcon, homeIcon);

import "./sidebar.scss";

const items = [
  {
    name: "Server",
    url: "/_panel/",
    shape: homeIconName,
  },
  {
    name: "Workers",
    url: "/_panel/workers",
    shape: cogIconName,
  }
];

// Main submenu
const Sidebar = () => (
  <div className="sidebar">
    <nav cds-layout="m-t:lg">
      <CdsNavigation expanded>
        {items.map(({ name, url, shape }, i) => (
          <NavLink to={url}>
            {({ isActive }) => (
              <CdsNavigationItem active={isActive}>
                <a>
                  <CdsIcon shape={shape} size="sm" />
                  {name}
                </a>
              </CdsNavigationItem>
            )}
          </NavLink>
        ))}
      </CdsNavigation>
    </nav>
  </div>
);

export default Sidebar;
