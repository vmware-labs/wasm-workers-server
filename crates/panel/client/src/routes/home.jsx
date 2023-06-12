// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useState } from 'react'
import { CdsTag } from "@cds/react/tag";
import { CdsProgressCircle } from "@cds/react/progress-circle";
import { CdsCard } from "@cds/react/card";
import { CdsIcon } from "@cds/react/icon";
import { CdsButton } from "@cds/react/button";
import { CdsDivider } from "@cds/react/divider";
import { ClarityIcons, eyeIcon, eyeIconName } from '@cds/core/icon';
import { Link } from "react-router-dom";

ClarityIcons.addIcons(eyeIcon);

const Home = () => {
  const [result, setResult] = useState(undefined);

  useEffect(() => {
    fetch("/_api/v0/workers")
      .then(res => res.json())
      .then(json => setResult(json));
  }, []);

  return <>
    <h2 cds-text="heading">Server</h2>
    {result === undefined ? (
      <CdsProgressCircle size="xl" />
    ) : (
      <>
        <CdsTag readonly status="success">Running</CdsTag>
        <div cds-layout="grid cols:auto gap:lg align:left">
          <CdsCard>
            <h3 cds-layout="m-t:xs m-b:md" cds-text="section">Workers</h3>
            <CdsDivider cds-card-remove-margin />
            <p cds-layout="m-t:lg m-b:lg" cds-text="display right">
              <b>{result.length}</b>
            </p>
            <CdsDivider cds-card-remove-margin />
            <div cds-layout="horizontal m-t:md m-b:xxs gap:sm align:right">
              <Link to="workers">
                <CdsButton action="flat-inline">
                  <CdsIcon shape={eyeIconName} size="sm" /> See workers
                </CdsButton>
              </Link>
            </div>
          </CdsCard>
        </div>
      </>
    )}
  </>
};

export default Home;
