// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { CdsCard } from "@cds/react/card";
import { CdsDivider } from "@cds/react/divider";
import { CdsIcon } from "@cds/react/icon";
import { CdsButton } from "@cds/react/button";
import { ClarityIcons, linkIcon, linkIconName, eyeIcon, eyeIconName } from '@cds/core/icon';
import { Link } from "react-router-dom";

ClarityIcons.addIcons(linkIcon, eyeIcon);

const WorkerCard = ({ worker }) => {
  return <CdsCard>
    <div className="worker-card">
      <h3 cds-layout="m-t:xs m-b:md" cds-text="section">{worker.name}</h3>
      <CdsDivider cds-card-remove-margin />
      <p>
        <b>Endpoint:</b> {worker.path}
      </p>
      <p>
        <b>Filepath:</b> {worker.filepath}
      </p>
      <CdsDivider cds-card-remove-margin />
      <div cds-layout="horizontal m-t:md m-b:xxs gap:sm align:right">
        <Link to={worker.id}>
          <CdsButton action="flat-inline">
            <CdsIcon shape={eyeIconName} size="sm" /> Details
          </CdsButton>
        </Link>
        <a href={worker.path} target="_blank">
          <CdsButton action="flat-inline">
            <CdsIcon shape={linkIconName} size="sm" /> View
          </CdsButton>
        </a>
      </div>
    </div>
  </CdsCard>
};

export default WorkerCard;
