import React from 'react';
import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';
import Diagram from "@site/src/pages/diagram.svg";

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
            className="button button--primary button--lg"
            to="/docs/get-started/quickstart">
            Get Started in 5 min ⏱️
          </Link>
        </div>
        <div className="hero__diagram" aria-label="A diagram showing how Wasm Workers Server loads several files from the filesystem and run them as workers">
          <Diagram />
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
      <main className='home__main'>
        <HomepageFeatures />
        <pre className={styles.codeHero}>
          <code>{`$ curl -fsSL https://workers.wasmlabs.dev/install | bash
$ wws --help
A WebAssembly framework to develop and run serverless applications anywhere

Usage: wws [OPTIONS] [PATH] [COMMAND]

Commands:
  runtimes  Manage the language runtimes in your project
  help      Print this message or the help of the given subcommand(s)

Arguments:
  [PATH]  Location of the wws project. It could be a local folder or a git repository [default: .]

Options:
      --host <HOSTNAME>          Hostname to initiate the server [default: 127.0.0.1]
  -p, --port <PORT>              Port to initiate the server [default: 8080]
      --prefix <PREFIX>          Prepend the given path to all URLs [default: ]
      --ignore <IGNORE>          Patterns to ignore when looking for worker files [default: ]
  -i, --install-runtimes         Install missing runtimes automatically
      --git-commit <GIT_COMMIT>  Set the commit when using a git repository as project
      --git-tag <GIT_TAG>        Set the tag when using a git repository as project
      --git-branch <GIT_BRANCH>  Set the branch when using a git repository as project
      --git-folder <GIT_FOLDER>  Change the directory when using a git repository as project
      --enable-panel             Enable the administration panel
      --cors <CORS>              CORS headers to add to all workers if not already set by the worker
  -h, --help                     Print help
  -V, --version                  Print version`}</code></pre>
      </main>
    </Layout>
  );
}
