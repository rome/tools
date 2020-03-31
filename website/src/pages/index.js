/**
 * Copyright (c) Facebook, Inc. and its affiliates.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the root directory of this source tree.
 *
 * @format
 */

import React from 'react';
import classnames from 'classnames';
import Layout from '@theme/Layout';
import Link from '@docusaurus/Link';
import useDocusaurusContext from '@docusaurus/useDocusaurusContext';
import useBaseUrl from '@docusaurus/useBaseUrl';
import styles from './styles.module.css';

function Home() {
  const context = useDocusaurusContext();
  const {siteConfig = {}} = context;
  return (
    <Layout title={siteConfig.tagline} description={siteConfig.tagline}>
      <header
        className={classnames(
          'hero',
          styles.heroBanner,
          styles.heroBannerBackground,
        )}>
        <div className="container">
          <div className="margin-bottom--lg">
            <h1
              className={classnames(
                styles.heroBannerTitle,
                'margin-bottom--lg',
              )}>
              Rome is an experimental <br />
              JavaScript toolchain
            </h1>
            <h2 className={styles.heroBannerSubtitle}>
              A compiler, linter, formatter, bundler, testing framework and more
            </h2>
          </div>
          <div className={styles.buttons}>
            <Link
              className={classnames(
                'button button--primary button--lg',
                styles.getStarted,
              )}
              to={useBaseUrl('docs/introduction/installation/')}>
              Get Started&nbsp;&nbsp;→
            </Link>
          </div>
          <div className="margin-top--lg">
            <iframe
              src="https://ghbtns.com/github-btn.html?user=facebookexperimental&amp;repo=rome&amp;type=star&amp;count=true&amp;size=large"
              frameBorder={0}
              scrolling={0}
              width={160}
              height={30}
              title="GitHub Stars"
            />
          </div>
        </div>
      </header>
      <main>
        <div
          className={classnames(
            'margin-bottom--lg',
            'padding-vert--lg',
            styles.calloutPrimary,
          )}>
          <div className="container">
            <div className="row">
              <div className="col col--8 col--offset-2">
                <div className="margin-vert--md text--center">
                  <p className={styles.calloutTagline}>
                    Rome is experimental and in active development. It is open
                    for contributors and those interested in experimental tools.
                  </p>
                </div>
              </div>
            </div>
          </div>
        </div>
        <div>
          <div className="container">
            <div className="row">
              <div className="col col--10 col--offset-1">
                {[
                  {
                    title: <>All Roads Lead to Rome</>,
                    imageUrl: useBaseUrl('img/undraw_mind_map_cwng.svg'),
                    imageAlt: 'Abstract Syntax Tree',
                    description: (
                      <>
                        Rome includes a compiler, linter, formatter, bundler,
                        testing framework and more. It aims to be a
                        comprehensive tool for anything related to the
                        processing of JavaScript source code.
                      </>
                    ),
                  },
                  {
                    title: <>No Third Party Dependencies</>,
                    imageUrl: useBaseUrl(
                      'img/undraw_under_construction_46pa.svg',
                    ),
                    imageAlt: 'House under construction',
                    description: (
                      <>
                        Rome is not a collection of existing tools. All
                        components are custom and use no third-party
                        dependencies.
                      </>
                    ),
                  },
                  {
                    title: <>Replaces Existing JavaScript Tools</>,
                    imageUrl: useBaseUrl('img/undraw_abstract_x68e.svg'),
                    imageAlt: 'Girl holding a building block',
                    description: (
                      <>
                        Rome aims to be a replacement for many existing
                        JavaScript tools. We will, however, offer integrations
                        for components in other tools. For example, using the
                        Rome compiler as a plugin for another bundler.
                      </>
                    ),
                  },
                ].map(({title, imageAlt, imageUrl, description}, index) => (
                  <div
                    class={classnames('row', styles.featureRow, {
                      [styles.featureReverse]: index % 2 === 0,
                    })}
                    key={index}>
                    <div class="col">
                      <img
                        alt={imageAlt}
                        className={styles.featureImage}
                        src={imageUrl}
                      />
                    </div>
                    <div class={classnames('col', styles.featureTextCol)}>
                      <div
                        className={classnames(
                          'padding-vert--lg',
                          styles.featureTextContainer,
                        )}>
                        <h3 className={styles.featureTitle}>{title}</h3>
                        <p className={styles.featureDescription}>
                          {description}
                        </p>
                      </div>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          </div>
        </div>
      </main>
    </Layout>
  );
}

export default Home;
