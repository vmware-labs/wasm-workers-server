// Copyright 2023 VMware, Inc.
// SPDX-License-Identifier: Apache-2.0

import { useEffect, useState } from 'react'
import { CdsProgressCircle } from "@cds/react/progress-circle";
import { useParams } from 'react-router-dom';

const Worker = () => {
  const [result, setResult] = useState(undefined);
  let params = useParams();

  useEffect(() => {
    fetch(`/_api/v0/workers/${params.id}`)
      .then(res => res.json())
      .then(json => setResult(json));
  }, []);

  return <>
    <h2 cds-text="heading">Worker information</h2>
    <p>ID: {params.id}</p>
    {result === undefined ? (
      <CdsProgressCircle size="xl" />
    ) : (
      <>
        <h3 cds-text="section">Key / Value store</h3>
        <table cds-table="border:row border:outside" cds-text="left">
          <thead>
            <tr>
              <th>Namespace</th>
            </tr>
          </thead>
          <tbody>
            <tr>
              <td>{result.data.kv ? result.data.kv : "-"}</td>
            </tr>
          </tbody>
        </table>
        <h3 cds-layout="m-t:md" cds-text="section">Environment variables</h3>
        <table cds-table="border:row border:outside" cds-text="left">
          <thead>
            <tr>
              <th>Variable</th>
              <th>Value</th>
            </tr>
          </thead>
          <tbody>
            {Object.keys(result.vars).length == 0 ? (
              <tr>
                <td>
                  -
                </td>
                <td>
                  -
                </td>
              </tr>
            ) : (
              Object.keys(result.vars).map((k, i) => (
                <tr key={i}>
                  <td>{k}</td>
                  <td>{result.vars[k]}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
        <h3 cds-layout="m-t:md" cds-text="section">Mount folders</h3>
        <table cds-table="border:row border:outside" cds-text="left">
          <thead>
            <tr>
              <th>From</th>
              <th>To</th>
            </tr>
          </thead>
          <tbody>
            {result.folders.length == 0 ? (
              <tr>
                <td>
                  -
                </td>
                <td>
                  -
                </td>
              </tr>
            ) : (
              result.folders.map((f, i) => (
                <tr key={i}>
                  <td>{f.from}</td>
                  <td>{f.to}</td>
                </tr>
              ))
            )}
          </tbody>
        </table>
      </>
    )}
  </>
}

export default Worker;
