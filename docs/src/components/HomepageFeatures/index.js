import React from 'react';
import clsx from 'clsx';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: 'Simple',
    emoji: "⚡️",
    description: (
      <>
        Start serving your first responses in 1 minute. <a href="https://twitter.com/vmwwasm/status/1582316125865775106" target="_blank" rel="noopener noreferrer">Don't you trust us?</a>
      </>
    ),
  },
  {
    title: 'Multi-language',
    emoji: "⚙️",
    description: (
      <>
        Create workers in different languages like JavaScript, Ruby, Python, Rust and Go thanks to WebAssembly.
      </>
    ),
  },
  {
    title: 'Compatible',
    emoji: "🚀",
    description: (
      <>
        Run your workers locally, in a small device, free-tier VPS, etc. Even in other platforms.
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
