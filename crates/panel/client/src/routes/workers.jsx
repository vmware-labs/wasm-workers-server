// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useState } from 'react'
import WorkerCard from "../components/workerCard";
import { CdsProgressCircle } from "@cds/react/progress-circle";

const Workers = () => {
  const [result, setResult] = useState(undefined);

  useEffect(() => {
    fetch("/_api/v0/workers")
      .then(res => res.json())
      .then(json => setResult(json));
  }, []);

  return <>
    <h2 cds-text="heading">Workers</h2>
    {result === undefined ? (
      <CdsProgressCircle size="xl" />
    ) : (
      <div cds-layout="grid cols:auto gap:lg align:left">
        {result.map((p, i) => (
          <WorkerCard worker={p} key={i} />
        ))}
      </div>
    )}
  </>
}

export default Workers;
