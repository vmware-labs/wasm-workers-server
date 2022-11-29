import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Simple',
    emoji: "⚡️",
    description: (
      <>
        Start serving your first responses in 5 minutes.
      </>
    ),
  },
  {
    title: 'Compatible',
    emoji: "⚙️",
    description: (
      <>
        Create workers in different languages thanks to WebAssembly.
      </>
    ),
  },
  {
    title: 'Run anywhere',
    emoji: "🚀",
    description: (
      <>
        Run it locally, in a small device, free-tier VPS, etc. Almost anywhere.
      </>
    ),
  },
];

function Feature({ emoji, title, description }) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <span className={styles.featureEmoji}>{emoji}</span>
      </div>
      <div className="text--center padding-horiz--md">
        <h3>{title}</h3>
        <p className={styles.featureDescription}>{description}</p>
      </div>
    </div>
  );
}

export default function HomepageFeatures() {
  return (
    <section className={styles.features}>
      <div className="container">
        <div className="row">
          {FeatureList.map((props, idx) => (
            <Feature key={idx} {...props} />
          ))}
        </div>
      </div>
    </section>
  );
}
