import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HomepageHeader() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/get-started/quickstart">
            Get Started - 5min ⏱️
          </Link>
        </div>
      </div>
    </header>
  );
}

export default function Home() {
  const { siteConfig } = useDocusaurusContext();
  return (
    <Layout
      title={siteConfig.title}
      description="Wasm Workers Server is a framework that allows you to to develop and run serverless code using a lightweight construct called workers. It's a self-contained binary that you can run almost anywhere.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />
        <pre className={styles.codeHero}>
          <code>{`$ curl https://raw.githubusercontent.com/vmware-labs/wasm-workers-server/main/install.sh | bash
$ wws --help
Usage: wws [OPTIONS] [PATH] [COMMAND]

Commands:
  runtimes  Manage the language runtimes in your project
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Folder to read WebAssembly modules from [default: .]

Options:
      --host <HOSTNAME>  Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>      Port to initiate the server [default: 8080]
      --prefix <PREFIX>  Prepend the given path to all URLs [default: ]
  -h, --help             Print help information
  -V, --version          Print version information`}</code></pre>
      </main>
    </Layout>
  );
}
