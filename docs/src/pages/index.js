import clsx from 'clsx';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import Layout from '@theme/Layout';
import HomepageFeatures from '@site/src/components/HomepageFeatures';

import styles from './index.module.css';

function HomepageHeader() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <header className={clsx('hero hero--primary', styles.heroBanner)}>
      <div className="container">
        <h1 className="hero__title">{siteConfig.title}</h1>
        <p className="hero__subtitle">{siteConfig.tagline}</p>
        <div className={styles.buttons}>
          <Link
            className="button button--secondary button--lg"
            to="/docs/getting-started/introduction">
            Get Started - 5min ⏱️
          </Link>
          <Link
            className="button button--outline button--secondary button--lg"
            to="/docs/getting-started/quick-start"
            style={{marginLeft: '1rem'}}>
            Quick Start
          </Link>
        </div>
        <div className={styles.badges}>
          <img src="https://img.shields.io/github/stars/entrepeneur4lyf/pmdaemon?style=flat-square" alt="GitHub Stars" />
          <img src="https://img.shields.io/crates/v/pmdaemon?style=flat-square" alt="Crates.io Version" />
          <img src="https://img.shields.io/crates/d/pmdaemon?style=flat-square" alt="Crates.io Downloads" />
          <img src="https://img.shields.io/badge/license-MIT-blue?style=flat-square" alt="License" />
          <img src="https://img.shields.io/github/actions/workflow/status/entrepeneur4lyf/pmdaemon/ci.yml?branch=main&style=flat-square" alt="Build Status" />
          <img src="https://img.shields.io/badge/rust-1.70+-orange.svg?style=flat-square" alt="Rust" />
          <img src="https://img.shields.io/badge/tests-272%20passing-brightgreen.svg?style=flat-square" alt="Tests" />
        </div>
      </div>
    </header>
  );
}

export default function Home() {
  const {siteConfig} = useDocusaurusContext();
  return (
    <Layout
      title={`${siteConfig.title} - Advanced Process Manager`}
      description="A high-performance process manager built in Rust, inspired by PM2 with innovative features that exceed the original.">
      <HomepageHeader />
      <main>
        <HomepageFeatures />

        {/* Quick Example Section */}
        <section className={styles.quickExample}>
          <div className="container">
            <div className="row">
              <div className="col col--6">
                <h2>🚀 Get Started in Seconds</h2>
                <p>
                  PMDaemon provides a familiar PM2-like interface with powerful enhancements.
                  Start managing your processes immediately with advanced port management,
                  health checks, and real-time monitoring.
                </p>
              </div>
              <div className="col col--6">
                <div className="command-example">
                  <pre><code>{`# Install PMDaemon
cargo install pmdaemon

# Start a clustered application
pmdaemon start app.js \\
  --instances 4 \\
  --port 3000-3003 \\
  --health-check-url http://localhost:3000/health

# Monitor in real-time
pmdaemon monit`}</code></pre>
                </div>
              </div>
            </div>
          </div>
        </section>

        {/* Key Advantages Section */}
        <section className={styles.advantages}>
          <div className="container">
            <h2 className="text--center">Why Choose PMDaemon?</h2>
            <div className="row">
              <div className="col col--4">
                <div className="feature-card">
                  <h3>🎯 Beyond PM2</h3>
                  <p>
                    Advanced port management with ranges, auto-assignment, and conflict detection.
                    Health checks with HTTP and script validation. Features PM2 doesn't have.
                  </p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature-card">
                  <h3>⚡ Rust Performance</h3>
                  <p>
                    Built with Rust for memory safety and blazing performance.
                    Async/await architecture with Tokio for efficient resource usage.
                  </p>
                </div>
              </div>
              <div className="col col--4">
                <div className="feature-card">
                  <h3>🔧 Production Ready</h3>
                  <p>
                    Comprehensive test suite (272 tests), robust error handling,
                    and production-grade web API with WebSocket support.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </section>
      </main>
    </Layout>
  );
}
