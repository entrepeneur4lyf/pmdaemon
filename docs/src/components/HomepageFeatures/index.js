import clsx from 'clsx';
import Heading from '@theme/Heading';
import styles from './styles.module.css';

const FeatureList = [
  {
    title: '🚀 Advanced Port Management',
    Svg: require('@site/static/img/port-management.svg').default,
    description: (
      <>
        Automatic port range distribution for clusters, auto-assignment from ranges,
        built-in conflict detection, and runtime port overrides. Features that go beyond PM2.
      </>
    ),
  },
  {
    title: '🏥 Health Checks & Monitoring',
    Svg: require('@site/static/img/health-checks.svg').default,
    description: (
      <>
        HTTP and script-based health checks with configurable timeouts and retries.
        Blocking start commands that wait for processes to be ready. Real-time monitoring with beautiful displays.
      </>
    ),
  },
  {
    title: '⚡ Rust Performance',
    Svg: require('@site/static/img/rust-performance.svg').default,
    description: (
      <>
        Built with Rust for memory safety and blazing performance. Async/await architecture
        with comprehensive error handling and production-ready reliability.
      </>
    ),
  },
  {
    title: '🌐 Modern Web API',
    Svg: require('@site/static/img/web-api.svg').default,
    description: (
      <>
        Full REST API with PM2-compatible responses, real-time WebSocket updates,
        and production-ready web server with CORS and security headers.
      </>
    ),
  },
  {
    title: '🔧 Enhanced CLI',
    Svg: require('@site/static/img/cli-enhanced.svg').default,
    description: (
      <>
        Familiar PM2-like commands with enhancements: bulk deletion, status-based operations,
        configurable monitoring intervals, and beautiful table formatting.
      </>
    ),
  },
  {
    title: '📊 Production Ready',
    Svg: require('@site/static/img/production-ready.svg').default,
    description: (
      <>
        Comprehensive test suite (267 tests), robust error handling, configuration persistence,
        and ecosystem file support with schema validation.
      </>
    ),
  },
];

function Feature({Svg, title, description}) {
  return (
    <div className={clsx('col col--4')}>
      <div className="text--center">
        <Svg className={styles.featureSvg} role="img" />
      </div>
      <div className="text--center padding-horiz--md">
        <Heading as="h3">{title}</Heading>
        <p>{description}</p>
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
